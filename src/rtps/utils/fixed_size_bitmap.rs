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

    pub fn new_from_base(base: u32) -> Self {
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

        self.base = base;
        self.range_max = self.base + (BitmapRange::NBITS as u32 - 1);
    }

    pub fn empty(&self) -> bool {
        return self.num_bits == 0;
    }

    pub fn max(&self) -> u32 {
        return self.base.wrapping_add(self.num_bits.wrapping_sub(1));
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
        for i in 0..num_items {
            self.bitmap[i as usize] = bitmap[i as usize];
        }
        if 0 < num_bits {
            self.bitmap[num_items as usize - 1] &= !(u32::MAX >> (num_bits & 31u32));
        }
        self.calc_maximum_bit_set(num_items, 0);
    }

    pub fn for_each<F: FnMut(u32) -> ()>(&mut self, mut f: F) {
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
                let mut n: usize = n_items as usize;
                while n < last_index {
                    self.bitmap[i] =
                        (self.bitmap[n] << n_bits) | (self.bitmap[n + 1] >> overflow_bits);
                    i += 1;
                    n += 1;
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
                self.bitmap
                    .copy_within(..BitmapRange::NITEMS - n_items as usize, n_items as usize);
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
                let mut n: usize = last_index - n_items as usize;
                while n > 0 {
                    self.bitmap[i] =
                        (self.bitmap[n] >> n_bits) | (self.bitmap[n - 1] << overflow_bits);
                    i -= 1;
                    n -= 1;
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
        let mut i: usize = starting_index as usize;
        let end: usize = min_index as usize;
        while i > end {
            i -= 1;
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
    use std::collections::HashSet;

    #[derive(Copy, Clone, Debug)]
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
            return self.bitmap[..self.num_longs as usize]
                == check.bitmap[..self.num_longs as usize];
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

    struct TestStep<T: InputType> {
        input: T,
        expected_result: TestResult,
    }

    struct TestCase<T: InputType> {
        initialization: TestResult,
        steps: Vec<TestStep<T>>,
    }

    impl<T: InputType> TestCase<T> {
        fn Test(&mut self, base: u32, uut: &mut BitmapRange) {
            assert_eq!(
                self.initialization.Check(self.initialization.result, uut),
                true
            );

            for step in self.steps.iter_mut() {
                let result = step.input.perform_input(base, uut);
                assert_eq!(step.expected_result.Check(result, uut), true);
                assert_eq!(
                    base.wrapping_add(step.expected_result.num_bits.wrapping_sub(1)),
                    uut.max()
                );
            }
        }
    }

    struct BitmapRangeTests {
        test0: TestCase<TestInputAdd>,
    }

    impl BitmapRangeTests {
        const explicit_base: u32 = 123;
        const sliding_base: u32 = 513;

        fn new_test_input_add() -> TestCase<TestInputAdd> {
            let mut steps = Vec::new();
            let adding_base_step: TestStep<TestInputAdd> = TestStep {
                input: TestInputAdd { offset: 0 },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 0,
                    num_bits: 1,
                    num_longs: 1,
                    bitmap: [0x80000000u32, 0, 0, 0, 0, 0, 0, 0],
                },
            };
            steps.push(adding_base_step);

            let adding_base_again_step: TestStep<TestInputAdd> = TestStep {
                input: TestInputAdd { offset: 0 },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 0,
                    num_bits: 1,
                    num_longs: 1,
                    bitmap: [0x80000000u32, 0, 0, 0, 0, 0, 0, 0],
                },
            };
            steps.push(adding_base_again_step);

            let adding_out_of_range_step: TestStep<TestInputAdd> = TestStep {
                input: TestInputAdd { offset: 256 },
                expected_result: TestResult {
                    result: false,
                    min: 0,
                    max: 0,
                    num_bits: 1,
                    num_longs: 1,
                    bitmap: [0x80000000u32, 0, 0, 0, 0, 0, 0, 0],
                },
            };
            steps.push(adding_out_of_range_step);

            let middle_of_first_word_step: TestStep<TestInputAdd> = TestStep {
                input: TestInputAdd { offset: 16 },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 16,
                    num_bits: 17,
                    num_longs: 1,
                    bitmap: [0x80008000u32, 0, 0, 0, 0, 0, 0, 0],
                },
            };
            steps.push(middle_of_first_word_step);

            let before_previous_one_step: TestStep<TestInputAdd> = TestStep {
                input: TestInputAdd { offset: 15 },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 16,
                    num_bits: 17,
                    num_longs: 1,
                    bitmap: [0x80018000u32, 0, 0, 0, 0, 0, 0, 0],
                },
            };
            steps.push(before_previous_one_step);

            let on_third_word_step: TestStep<TestInputAdd> = TestStep {
                input: TestInputAdd { offset: 67 },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 67,
                    num_bits: 68,
                    num_longs: 3,
                    bitmap: [0x80018000u32, 0, 0x10000000u32, 0, 0, 0, 0, 0],
                },
            };
            steps.push(on_third_word_step);

            let before_last_on_third_word_step: TestStep<TestInputAdd> = TestStep {
                input: TestInputAdd { offset: 94 },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 94,
                    num_bits: 95,
                    num_longs: 3,
                    bitmap: [0x80018000u32, 0, 0x10000002u32, 0, 0, 0, 0, 0],
                },
            };
            steps.push(before_last_on_third_word_step);

            let last_on_third_word_step: TestStep<TestInputAdd> = TestStep {
                input: TestInputAdd { offset: 95 },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 95,
                    num_bits: 96,
                    num_longs: 3,
                    bitmap: [0x80018000u32, 0, 0x10000003u32, 0, 0, 0, 0, 0],
                },
            };
            steps.push(last_on_third_word_step);

            let last_possible_item_step: TestStep<TestInputAdd> = TestStep {
                input: TestInputAdd { offset: 255 },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 255,
                    num_bits: 256,
                    num_longs: 8,
                    bitmap: [0x80018000u32, 0, 0x10000003u32, 0, 0, 0, 0, 0x00000001u32],
                },
            };
            steps.push(last_possible_item_step);

            let test_case = TestCase {
                initialization: TestResult::new(),
                steps: steps,
            };

            test_case
        }

        const all_ones: TestResult = TestResult {
            result: true,
            min: 0,
            max: 255,
            num_bits: 256,
            num_longs: 8,
            bitmap: [
                0xFFFFFFFFu32,
                0xFFFFFFFFu32,
                0xFFFFFFFFu32,
                0xFFFFFFFFu32,
                0xFFFFFFFFu32,
                0xFFFFFFFFu32,
                0xFFFFFFFFu32,
                0xFFFFFFFFu32,
            ],
        };

        fn new_test_range0() -> TestCase<TestInputAddRange> {
            let mut steps = Vec::new();
            let empty_input_step: TestStep<TestInputAddRange> = TestStep {
                input: TestInputAddRange {
                    offset_from: 0,
                    offset_to: 0,
                },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 0,
                    num_bits: 0,
                    num_longs: 0,
                    bitmap: [0, 0, 0, 0, 0, 0, 0, 0],
                },
            };
            steps.push(empty_input_step);

            let adding_base_step: TestStep<TestInputAddRange> = TestStep {
                input: TestInputAddRange {
                    offset_from: 0,
                    offset_to: 1,
                },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 0,
                    num_bits: 1,
                    num_longs: 1,
                    bitmap: [0x80000000, 0, 0, 0, 0, 0, 0, 0],
                },
            };
            steps.push(adding_base_step);

            let wrong_order_params_step: TestStep<TestInputAddRange> = TestStep {
                input: TestInputAddRange {
                    offset_from: 10,
                    offset_to: 1,
                },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 0,
                    num_bits: 1,
                    num_longs: 1,
                    bitmap: [0x80000000, 0, 0, 0, 0, 0, 0, 0],
                },
            };
            steps.push(wrong_order_params_step);

            let adding_out_of_range_step: TestStep<TestInputAddRange> = TestStep {
                input: TestInputAddRange {
                    offset_from: 256,
                    offset_to: 257,
                },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 0,
                    num_bits: 1,
                    num_longs: 1,
                    bitmap: [0x80000000, 0, 0, 0, 0, 0, 0, 0],
                },
            };
            steps.push(adding_out_of_range_step);

            let middle_of_first_word_step: TestStep<TestInputAddRange> = TestStep {
                input: TestInputAddRange {
                    offset_from: 15,
                    offset_to: 17,
                },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 16,
                    num_bits: 17,
                    num_longs: 1,
                    bitmap: [0x80018000, 0, 0, 0, 0, 0, 0, 0],
                },
            };
            steps.push(middle_of_first_word_step);

            let on_second_and_third_word_step: TestStep<TestInputAddRange> = TestStep {
                input: TestInputAddRange {
                    offset_from: 35,
                    offset_to: 68,
                },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 67,
                    num_bits: 68,
                    num_longs: 3,
                    bitmap: [0x80018000, 0x1FFFFFFF, 0xF0000000, 0, 0, 0, 0, 0],
                },
            };
            steps.push(on_second_and_third_word_step);

            let crossing_more_than_one_word_step: TestStep<TestInputAddRange> = TestStep {
                input: TestInputAddRange {
                    offset_from: 94,
                    offset_to: 133,
                },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 132,
                    num_bits: 133,
                    num_longs: 5,
                    bitmap: [
                        0x80018000, 0x1FFFFFFF, 0xF0000003, 0xFFFFFFFF, 0xF8000000, 0, 0, 0,
                    ],
                },
            };
            steps.push(crossing_more_than_one_word_step);

            let exactly_one_word_step: TestStep<TestInputAddRange> = TestStep {
                input: TestInputAddRange {
                    offset_from: 64,
                    offset_to: 96,
                },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 132,
                    num_bits: 133,
                    num_longs: 5,
                    bitmap: [
                        0x80018000, 0x1FFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xF8000000, 0, 0, 0,
                    ],
                },
            };
            steps.push(exactly_one_word_step);

            let exactly_two_words_step: TestStep<TestInputAddRange> = TestStep {
                input: TestInputAddRange {
                    offset_from: 128,
                    offset_to: 192,
                },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 191,
                    num_bits: 192,
                    num_longs: 6,
                    bitmap: [
                        0x80018000, 0x1FFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0,
                        0,
                    ],
                },
            };
            steps.push(exactly_two_words_step);

            let full_range_step: TestStep<TestInputAddRange> = TestStep {
                input: TestInputAddRange {
                    offset_from: 0,
                    offset_to: 512,
                },
                expected_result: BitmapRangeTests::all_ones,
            };
            steps.push(full_range_step);

            let test_case = TestCase {
                initialization: TestResult::new(),
                steps: steps,
            };

            test_case
        }

        fn new_test_remove0() -> TestCase<TestInputRemove> {
            let mut steps = Vec::new();
            let removing_out_of_range_step: TestStep<TestInputRemove> = TestStep {
                input: TestInputRemove {
                    offset_begin: 32,
                    offset_end: 33,
                },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 31,
                    num_bits: 32,
                    num_longs: 1,
                    bitmap: [0xFFFFFFFF, 0, 0, 0, 0, 0, 0, 0],
                },
            };
            steps.push(removing_out_of_range_step);

            let removing_single_in_the_middle_step: TestStep<TestInputRemove> = TestStep {
                input: TestInputRemove {
                    offset_begin: 5,
                    offset_end: 6,
                },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 31,
                    num_bits: 32,
                    num_longs: 1,
                    bitmap: [0xFBFFFFFF, 0, 0, 0, 0, 0, 0, 0],
                },
            };
            steps.push(removing_single_in_the_middle_step);

            let removing_several_in_the_middle_step: TestStep<TestInputRemove> = TestStep {
                input: TestInputRemove {
                    offset_begin: 6,
                    offset_end: 31,
                },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 31,
                    num_bits: 32,
                    num_longs: 1,
                    bitmap: [0xF8000001, 0, 0, 0, 0, 0, 0, 0],
                },
            };
            steps.push(removing_several_in_the_middle_step);

            let removing_last_step: TestStep<TestInputRemove> = TestStep {
                input: TestInputRemove {
                    offset_begin: 31,
                    offset_end: 32,
                },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 4,
                    num_bits: 5,
                    num_longs: 1,
                    bitmap: [0xF8000000, 0, 0, 0, 0, 0, 0, 0],
                },
            };
            steps.push(removing_last_step);

            let removing_first_step: TestStep<TestInputRemove> = TestStep {
                input: TestInputRemove {
                    offset_begin: 0,
                    offset_end: 1,
                },
                expected_result: TestResult {
                    result: true,
                    min: 1,
                    max: 4,
                    num_bits: 5,
                    num_longs: 1,
                    bitmap: [0x78000000, 0, 0, 0, 0, 0, 0, 0],
                },
            };
            steps.push(removing_first_step);

            let removing_all_except_first_and_last_step: TestStep<TestInputRemove> = TestStep {
                input: TestInputRemove {
                    offset_begin: 2,
                    offset_end: 4,
                },
                expected_result: TestResult {
                    result: true,
                    min: 1,
                    max: 4,
                    num_bits: 5,
                    num_longs: 1,
                    bitmap: [0x48000000, 0, 0, 0, 0, 0, 0, 0],
                },
            };
            steps.push(removing_all_except_first_and_last_step);

            let removing_last_2_step: TestStep<TestInputRemove> = TestStep {
                input: TestInputRemove {
                    offset_begin: 4,
                    offset_end: 5,
                },
                expected_result: TestResult {
                    result: true,
                    min: 1,
                    max: 1,
                    num_bits: 2,
                    num_longs: 1,
                    bitmap: [0x40000000, 0, 0, 0, 0, 0, 0, 0],
                },
            };
            steps.push(removing_last_2_step);

            let removing_first_2_step: TestStep<TestInputRemove> = TestStep {
                input: TestInputRemove {
                    offset_begin: 1,
                    offset_end: 2,
                },
                expected_result: TestResult {
                    result: true,
                    min: 0,
                    max: 0,
                    num_bits: 0,
                    num_longs: 0,
                    bitmap: [0, 0, 0, 0, 0, 0, 0, 0],
                },
            };
            steps.push(removing_first_2_step);

            let test_case = TestCase {
                initialization: TestResult {
                    result: true,
                    min: 0,
                    max: 31,
                    num_bits: 32,
                    num_longs: 1,
                    bitmap: [0xFFFFFFFF, 0, 0, 0, 0, 0, 0, 0],
                },
                steps,
            };

            test_case
        }
    }

    #[test]
    fn default_constructor_test() {
        let mut uut: BitmapRange = BitmapRange::new();
        let mut test = BitmapRangeTests::new_test_input_add();
        test.Test(0u32, &mut uut);
    }

    #[test]
    fn explicit_constructor_test() {
        let mut uut: BitmapRange = BitmapRange::new_from_base(BitmapRangeTests::explicit_base);
        let mut test = BitmapRangeTests::new_test_input_add();
        test.Test(BitmapRangeTests::explicit_base, &mut uut);
    }

    #[test]
    fn range_default_constructor_test() {
        let mut uut: BitmapRange = BitmapRange::new();
        let mut test = BitmapRangeTests::new_test_range0();
        test.Test(0u32, &mut uut);
    }

    #[test]
    fn range_explicit_constructor_test() {
        let mut uut: BitmapRange = BitmapRange::new_from_base(BitmapRangeTests::explicit_base);
        let mut test = BitmapRangeTests::new_test_range0();
        test.Test(BitmapRangeTests::explicit_base, &mut uut);
    }

    #[test]
    fn change_base_test() {
        let mut uut: BitmapRange = BitmapRange::new();
        let mut test = BitmapRangeTests::new_test_input_add();
        test.Test(0u32, &mut uut);

        // Change base and test again
        uut.from_base(BitmapRangeTests::explicit_base);
        test.Test(BitmapRangeTests::explicit_base, &mut uut);
    }

    #[test]
    fn full_range_test() {
        let mut uut: BitmapRange = BitmapRange::new_from_base(BitmapRangeTests::explicit_base);

        // Add all possible items in range
        for item in BitmapRangeTests::explicit_base..BitmapRangeTests::explicit_base + 256 {
            assert_eq!(uut.add(&item), true);
        }
        BitmapRangeTests::all_ones.Check(BitmapRangeTests::all_ones.result, &uut);
    }

    #[test]
    fn serialization_test() {
        let mut num_bits: u32 = 0;
        let mut num_longs: u32 = 0;
        let mut bitmap: bitmap_type = [0; BitmapRange::NITEMS];

        // Populate the range using the test case
        let mut uut: BitmapRange = BitmapRange::new_from_base(BitmapRangeTests::explicit_base);
        let mut test = BitmapRangeTests::new_test_input_add();
        test.Test(BitmapRangeTests::explicit_base, &mut uut);

        // Get bitmap serialization and set it again
        uut.bitmap_get(&mut num_bits, &mut bitmap, &mut num_longs);
        uut.bitmap_set(num_bits, &bitmap);

        // Bitmap should be equal to the one of the last result
        let last_result = test.steps.iter().rev().next().unwrap().expected_result;
        last_result.Check(last_result.result, &uut);

        num_bits = 20;
        num_longs = 1;
        bitmap.fill(u32::MAX);
        uut.bitmap_set(num_bits, &bitmap);
        uut.bitmap_get(&mut num_bits, &mut bitmap, &mut num_longs);
        assert_eq!(num_bits, 20);
        assert_eq!(num_longs, 1);
        assert_eq!(bitmap[0], 0xFFFFF000);
    }

    #[test]
    fn traversal_test() {
        // Populate the range using the test case
        let mut uut: BitmapRange = BitmapRange::new_from_base(BitmapRangeTests::explicit_base);
        let mut test = BitmapRangeTests::new_test_input_add();
        test.Test(BitmapRangeTests::explicit_base, &mut uut);

        // Collect the items that should be processed
        let mut items: HashSet<u32> = HashSet::new();
        for step in test.steps.iter_mut() {
            if step.expected_result.result {
                items.insert(BitmapRangeTests::explicit_base + step.input.offset);
            }
        }

        uut.for_each(|t: u32| -> () {
            assert_eq!(items.contains(&t), true);
            items.remove(&t);
        });

        assert_eq!(items.is_empty(), true);
    }

    #[test]
    fn sliding_window_test() {
        let mut uut: BitmapRange = BitmapRange::new_from_base(BitmapRangeTests::sliding_base);
        uut.add(&BitmapRangeTests::sliding_base);

        // Check shifting right and then left
        for i in 0..256u32 {
            uut.base_update(BitmapRangeTests::sliding_base - i);
            assert_eq!(uut.max(), BitmapRangeTests::sliding_base);
            uut.base_update(BitmapRangeTests::sliding_base);
            assert_eq!(uut.max(), BitmapRangeTests::sliding_base);
        }

        // Check shifting left and then right
        for i in 0..256u32 {
            uut.base_update(BitmapRangeTests::sliding_base - i);
            assert_eq!(uut.max(), BitmapRangeTests::sliding_base);
            uut.base_update(BitmapRangeTests::sliding_base - 255);
            assert_eq!(uut.max(), BitmapRangeTests::sliding_base);
        }

        // Check cases dropping the most significant bit
        let v = BitmapRangeTests::sliding_base - 100u32;
        uut.add(&v);
        uut.base_update(BitmapRangeTests::sliding_base - 256);
        assert_eq!(uut.max(), BitmapRangeTests::sliding_base - 100);
        uut.base_update(0);
        assert!(uut.empty());
    }

    #[test]
    fn remove_test() {
        let mut uut: BitmapRange = BitmapRange::new_from_base(BitmapRangeTests::explicit_base);
        uut.add_range(
            &BitmapRangeTests::explicit_base,
            &(BitmapRangeTests::explicit_base + 32),
        );
        let mut test = BitmapRangeTests::new_test_remove0();
        test.Test(BitmapRangeTests::explicit_base, &mut uut);
    }
}
