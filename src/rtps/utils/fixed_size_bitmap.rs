use bitintr::*;
use std::cmp;

type bitmap_type = [u32; BitmapRange::NITEMS];

#[derive(Debug)]
pub struct BitmapRange {
    base: u32,
    range_max: u32,
    bitmap: bitmap_type,
    num_bits: u32,
}

impl BitmapRange {
    pub const NBITS: usize = 256;
    pub const NITEMS: usize = ((BitmapRange::NBITS + 31usize) / 32usize);

    pub fn new() -> Self {
        let base: u32 = 0;
        let range_max: u32 = base + (BitmapRange::NBITS as u32 - 1);
        Self {
            base: base,
            range_max: range_max,
            bitmap: [0; BitmapRange::NITEMS],
            num_bits: 0,
        }
    }

    pub fn base(&self) -> u32 {
        return self.base;
    }

    pub fn from_base(&mut self, base: u32) {
        self.base = base;
        self.range_max = base + (BitmapRange::NBITS as u32 - 1);
        self.num_bits = 0;
        self.bitmap.fill(0u32);
    }

    pub fn base_update(&mut self, base: u32) {
        if base == self.base {
            return;
        }

        if base > self.base {
            let n_bits = BitmapRange::d_func(base, self.base);
            self.shift_map_left(n_bits);
        } else {
            let n_bits = BitmapRange::d_func(self.base, base);
            self.shift_map_right(n_bits);
        }
    }

    pub fn empty(&self) -> bool {
        return self.num_bits == 0;
    }

    pub fn max(&self) -> u32 {
        return self.base - (self.num_bits - 1);
    }

    pub fn min(&self) -> u32 {
        let mut item = self.base;
        let n_longs = (self.num_bits + 31u32) / 32u32;
        for i in 0..n_longs {
            let bits = self.bitmap[i as usize];
            if bits > 0 {
                let offset: u32 = bits.lzcnt();
                return item + offset;
            }

            item = item + 32u32;
        }

        return self.base;
    }

    pub fn is_set(&self, item: &u32) -> bool {
        if *item >= self.base && self.range_max >= *item {
            let mut diff = BitmapRange::d_func(*item, self.base);
            if diff < self.num_bits {
                let pos: u32 = diff >> 5;
                diff &= 31u32;
                return (self.bitmap[pos as usize] & (1u32 << (31u32 - diff))) != 0;
            }
        }
        return false;
    }

    pub fn add(&mut self, item: &u32) -> bool {
        if *item >= self.base && self.range_max >= *item {
            let mut diff = BitmapRange::d_func(*item, self.base);
            self.num_bits = cmp::max(diff + 1, self.num_bits);
            let pos = diff >> 5;
            diff &= 31u32;
            self.bitmap[pos as usize] |= 1u32 << (31u32 - diff);
            return true;
        }

        return false;
    }

    pub fn add_range(&mut self, from: &u32, to: &u32) {
        let full_mask = u32::MAX;
        let min = if self.base >= *from { self.base } else { *from };
        let max = if *to >= self.base + BitmapRange::NBITS as u32 {
            self.base + BitmapRange::NBITS as u32
        } else {
            *to
        };

        if min >= max {
            return;
        }

        let mut offset = BitmapRange::d_func(min, self.base);
        let mut n_bits = BitmapRange::d_func(max, min);
        self.num_bits = cmp::max(self.num_bits, offset + n_bits);

        let mut pos = offset >> 5;
        offset &= 31u32;
        let mut mask = full_mask;
        mask >>= offset;
        let mut bits_in_mask = 32u32 - offset;

        while n_bits >= bits_in_mask {
            self.bitmap[pos as usize] |= mask;
            pos += 1;
            n_bits -= bits_in_mask;
            mask = full_mask;
            bits_in_mask = 32u32;
        }

        if n_bits > 0 {
            self.bitmap[pos as usize] |= mask & (full_mask << (bits_in_mask - n_bits));
        }
    }

    pub fn remove(&mut self, item: &u32) {
        let max_value = self.max();
        if (*item >= self.base) && (max_value >= *item) {
            let mut diff = BitmapRange::d_func(*item, self.base);
            let pos = diff >> 5;
            diff &= 31u32;
            self.bitmap[pos as usize] &= !(1u32 << (31u32 - diff));

            if *item == max_value {
                self.calc_maximum_bit_set(pos + 1, 0);
            }
        }
    }

    pub fn bitmap_get(
        &self,
        num_bits: &mut u32,
        bitmap: &mut bitmap_type,
        num_longs_used: &mut u32,
    ) {
        *num_bits = self.num_bits;
        *num_longs_used = (self.num_bits + 31u32) / 32u32;
        *bitmap = self.bitmap;
    }

    pub fn bitmap_set(&mut self, num_bits: u32, bitmap: &bitmap_type) {
        self.num_bits = cmp::min(num_bits, BitmapRange::NBITS as u32);
        let num_items = (self.num_bits + 31u32) / 32u32;
        self.bitmap.fill(0u32);
        self.bitmap[..num_items as usize].clone_from_slice(bitmap);
        if 0 < num_bits {
            self.bitmap[num_items as usize - 1] &= !(u32::MAX >> (num_bits & 31u32));
        }
        self.calc_maximum_bit_set(num_items, 0);
    }

    pub fn for_each(&mut self, f: fn(u32)->()) {
        let mut item = self.base;
        // Traverse through the significant items on the bitmap
        let n_longs = (self.num_bits + 31) / 32;
        for i in 0..n_longs {
            // Traverse through the bits set on the item, msb first.
            // Loop will stop when there are no bits set.
            let mut bits = self.bitmap[i as usize];
            while bits > 0 {
                // We use an intrinsic to find the index of the highest bit set.
                // Most modern CPUs have an instruction to count the leading zeroes of a word.
                // The number of leading zeroes will give us the index we need.
                let offset: u32 = bits.lzcnt();
                let bit = 31u32 ^ offset;

                // Call the function for the corresponding item
                f(item + offset);

                // Clear the most significant bit
                bits &= !(1u32 << bit);
            }

            // There are 32 items on each bitmap item.
            item = item + 32u32;
        }
    }

    fn d_func(a: u32, b: u32) -> u32 {
        a - b
    }

    fn shift_map_left(&mut self, mut n_bits: u32) {
        if n_bits >= self.num_bits {
            // Shifting more than most significant. Clear whole bitmap.
            self.num_bits = 0;
            self.bitmap.fill(0);
        } else {
            // Significant bit will move left by n_bits
            self.num_bits -= n_bits;

            // Div and mod by 32
            let n_items = n_bits >> 5;
            n_bits &= 31;
            if n_bits == 0 {
                // Shifting a multiple of 32 bits, just move the bitmap integers
                self.bitmap.copy_within(n_items as usize.., 0);
                self.bitmap[BitmapRange::NITEMS - n_items as usize..].fill(0);
            } else {
                // Example. Shifting 44 bits. Should shift one complete word and 12 bits.
                // Need to iterate forward and take 12 bits from next word (shifting it 20 bits).
                // aaaaaaaa bbbbbbbb cccccccc dddddddd
                // bbbbbccc bbbbbbbb cccccccc dddddddd
                // bbbbbccc cccccddd ddddd000 dddddddd
                // bbbbbccc cccccddd ddddd000 00000000
                let overflow_bits = 32u32 - n_bits;
                let last_index = BitmapRange::NITEMS - 1usize;
                let mut i: usize = 0;
                for n in n_items as usize..last_index {
                    self.bitmap[i] = (self.bitmap[n] << n_bits) | (self.bitmap[n + 1] >> overflow_bits);
                    i += 1;
                }
                // Last one does not have next word
                self.bitmap[last_index - n_items as usize] = self.bitmap[last_index] << n_bits;
                // Last n_items will become 0
                self.bitmap[BitmapRange::NITEMS - n_items as usize..].fill(0);
            }
        }
    }

    fn shift_map_right(&mut self, mut n_bits: u32) {
        if n_bits as usize >= BitmapRange::NBITS {
            // Shifting more than total bitmap size. Clear whole bitmap.
            self.num_bits = 0;
            self.bitmap.fill(0u32);
        } else {
            // Detect if highest bit will be dropped and take note, as we will need
            // to find new maximum bit in that case
            let new_num_bits = self.num_bits + n_bits;
            let find_new_max = new_num_bits as usize > BitmapRange::NBITS;

            // Div and mod by 32
            let n_items = n_bits >> 5;
            n_bits &= 31;
            if n_bits == 0 {
                // Shifting a multiple of 32 bits, just move the bitmap integers
                self.bitmap.copy_within(..BitmapRange::NITEMS - n_items  as usize, n_items as usize);
                self.bitmap[..n_items as usize].fill(0);
            } else {
                // Example. Shifting 44 bits. Should shift one complete word and 12 bits.
                // Need to iterate backwards and take 12 bits from previous word (shifting it 20 bits).
                // aaaaaaaa bbbbbbbb cccccccc dddddddd
                // aaaaaaaa bbbbbbbb cccccccc bbbccccc
                // aaaaaaaa bbbbbbbb aaabbbbb bbbccccc
                // aaaaaaaa 000aaaaa aaabbbbb bbbccccc
                // 00000000 000aaaaa aaabbbbb bbbccccc
                let overflow_bits = 32 - n_bits;
                let last_index = BitmapRange::NITEMS - 1;
                let mut i = last_index;
                for n in last_index - n_items as usize .. 0 {
                    self.bitmap[i] = (self.bitmap[n] >> n_bits) | (self.bitmap[n - 1] << overflow_bits);
                    i -= 1;
                }
                // First item does not have previous word
                self.bitmap[n_items as usize] = self.bitmap[0] >> n_bits;
                // First n_items will become 0
                self.bitmap[..n_items as usize].fill(0);
            }

            self.num_bits = new_num_bits;
            if find_new_max {
                self.calc_maximum_bit_set(BitmapRange::NITEMS as u32, n_items);
            }
        }
    }

    fn calc_maximum_bit_set(&mut self, starting_index: u32, min_index: u32) {
        self.num_bits = 0;
        for i in starting_index as usize..min_index as usize {
            let mut bits = self.bitmap[i as usize];
            if bits != 0 {
                bits = bits & !(bits - 1);
                let offset: u32 = bits.lzcnt() + 1;
                self.num_bits = ((i as u32) << 5) + offset;
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestResult {
        result: bool,
        min: u32,
        max: u32,
        num_bits: u32,
        num_longs: u32,
        bitmap: bitmap_type,
    }

    impl TestResult {
        pub fn new() -> Self {
            Self {
                result: true,
                min: 0,
                max: 0,
                num_bits: 0,
                num_longs: 0,
                bitmap: [0; BitmapRange::NITEMS],
            }
        }

        pub fn Check(&self, ret_val: bool, uut: &BitmapRange) -> bool {
            if self.result != ret_val {
                return false;
            }

            let mut check = TestResult::new();
            uut.bitmap_get(&mut check.num_bits, &mut check.bitmap, &mut check.num_longs);
            if self.num_bits != check.num_bits || self.num_longs != check.num_longs {
                return false;
            }

            if !uut.empty() {
                let base = uut.base();
                if uut.max() != (base + self.max) || uut.min() != (base + self.min) {
                    return false;
                }
            }
            return self.bitmap[..self.num_longs as usize] == check.bitmap[..self.num_longs as usize];
        }
    }

    pub trait InputType {
        fn perform_input(&mut self, base: u32, uut: &mut BitmapRange) -> bool;
    }

    struct TestInputAdd {
        offset: u32,
    }

    impl InputType for TestInputAdd {
        fn perform_input(&mut self, base: u32, uut: &mut BitmapRange) -> bool {
            return uut.add(&(base + self.offset));
        }
    }

    struct TestInputAddRange {
        offset_from: u32,
        offset_to: u32,
    }

    impl InputType for TestInputAddRange {
        fn perform_input(&mut self, base: u32, uut: &mut BitmapRange) -> bool {
            uut.add_range(&(base + self.offset_from), &(base + self.offset_to));
            return true;
        }
    }

    struct TestInputRemove {
        offset_begin: u32,
        offset_end: u32,
    }

    impl InputType for TestInputRemove {
        fn perform_input(&mut self, base: u32, uut: &mut BitmapRange) -> bool {
            for offset in self.offset_begin as usize..self.offset_end as usize {
                uut.remove(&(base + offset as u32));
            }
            return true;
        }
    }

    struct TestStep {
        input: TestInputAdd,
        expected_result: TestResult,
    }

    struct TestCase {
        initialization: TestResult,
        steps: Vec<TestStep>,
    }

    impl TestCase {
        fn Test(&mut self, base: u32, uut: &mut BitmapRange) {
            assert_eq!(self.initialization.Check(self.initialization.result, uut), true);

            for step in self.steps.iter_mut() {
                let result = step.input.perform_input(base, uut);
                assert_eq!(step.expected_result.Check(result, uut), true);
                assert_eq!(base + step.expected_result.num_bits - 1, uut.max());
            }
        }
    }

    struct BitmapRangeTests {
        test0: TestCase,
    }

    impl BitmapRangeTests {
        const explicit_base: u32 = 123;
        const sliding_base: u32 = 513;

        fn new_test_input_add() -> Self {
            let mut steps = Vec::new();
            let test_input_add = TestInputAdd {
                offset: 0,
            };

            let test_result = TestResult {
                result: true,
                min: 0,
                max: 0,
                num_bits: 1,
                num_longs: 1,
                bitmap: [0x80000000u32, 0, 0, 0, 0, 0, 0, 0],
            };

            let test_step: TestStep = TestStep {
                input: test_input_add,
                expected_result: test_result,
            };

            steps.push(test_step);

            let test_case = TestCase {
                initialization: TestResult::new(),
                steps: steps
            };

            Self {
                test0: test_case,
            }
        }
    }

    #[test]
    fn default_constructor_test() {
        let mut uut: BitmapRange = BitmapRange::new();
        let mut test = BitmapRangeTests::new_test_input_add();
        test.test0.Test(0u32, &mut uut);
    }
}
