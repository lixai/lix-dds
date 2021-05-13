use std::ops::{Add, AddAssign, Sub};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct SequenceNumber_t {
    high: i32,
    low: u32,
}

impl SequenceNumber_t {
    pub fn new(hi: i32, lo: u32) -> Self {
        SequenceNumber_t { high: hi, low: lo }
    }

    pub fn to64long(&self) -> u64 {
        (self.high as u64) << 32 + self.low
    }

    pub fn incrememt(&mut self) -> Self {
        self.low = self.low.wrapping_add(1);
        if self.low == 0 {
            self.high += 1;
        }
        return *self;
    }

    pub fn unknown() -> Self {
        SequenceNumber_t { high: -1, low: 0 }
    }

    pub const c_SequenceNumber_Unknown: SequenceNumber_t = SequenceNumber_t { high: -1, low: 0 };

    pub fn sort_seqNum(s1: Self, s2: Self) -> bool {
        s1 < s2
    }
}

impl From<u64> for SequenceNumber_t {
    fn from(u: u64) -> Self {
        SequenceNumber_t {
            high: (u >> 32) as i32,
            low: u as u32,
        }
    }
}

impl AddAssign<i32> for SequenceNumber_t {
    fn add_assign(&mut self, inc: i32) {
        assert!(inc >= 0);
        let aux_low = self.low;
        self.low = self.low.wrapping_add(inc as u32);
        if self.low < aux_low {
            self.high += 1;
        };
    }
}

impl Sub<u32> for SequenceNumber_t {
    type Output = Self;
    fn sub(self, inc: u32) -> Self {
        let low: u32 = self.low.wrapping_sub(inc);
        let mut res = SequenceNumber_t {
            high: self.high,
            low,
        };
        if inc > self.low {
            res.high -= 1;
        }
        return res;
    }
}

impl Sub<SequenceNumber_t> for SequenceNumber_t {
    type Output = Self;
    fn sub(self, subtrahend: SequenceNumber_t) -> Self {
        assert!(self >= subtrahend);
        let low: u32 = self.low.wrapping_sub(subtrahend.low);
        let mut res = SequenceNumber_t {
            high: self.high - subtrahend.high,
            low,
        };
        if self.low < subtrahend.low {
            res.high -= 1;
        }
        return res;
    }
}

impl Add<u32> for SequenceNumber_t {
    type Output = Self;
    fn add(self, inc: u32) -> Self {
        let low: u32 = self.low.wrapping_add(inc);
        let mut res = SequenceNumber_t {
            high: self.high,
            low,
        };
        if res.low < self.low {
            res.high += 1;
        }
        return res;
    }
}

impl Add<SequenceNumber_t> for SequenceNumber_t {
    type Output = Self;
    fn add(self, inc: SequenceNumber_t) -> Self {
        let low: u32 = self.low.wrapping_add(inc.low);
        let mut res = SequenceNumber_t {
            high: self.high + inc.high,
            low,
        };
        if res.low < self.low {
            res.high += 1;
        }
        return res;
    }
}

struct SequenceNumberDiff {}

impl SequenceNumberDiff {
    pub fn diff(a: SequenceNumber_t, b: SequenceNumber_t) -> u32 {
        let diff = a - b;
        return diff.low;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incremental_operator_test() {
        let mut seq = SequenceNumber_t::new(0, u32::MAX);
        seq = seq + 1;
        let mut expected_seq = SequenceNumber_t::new(1, 0);

        assert_eq!(seq, expected_seq);
        seq = seq + 1;
        expected_seq.low = 1;
        assert_eq!(seq, expected_seq);
    }

    #[test]
    fn addition_assigment_operator_test() {
        let mut seq = SequenceNumber_t::new(3, u32::MAX - 3);
        seq += 7;
        let mut expected_seq = SequenceNumber_t::new(4, 3);

        assert_eq!(seq, expected_seq);
        seq += i32::MAX;
        expected_seq.low = i32::MAX as u32;
        expected_seq.low += 3;
        assert_eq!(seq, expected_seq);

        seq += i32::MAX;

        expected_seq.high = 5;
        expected_seq.low = 1;
        assert_eq!(seq, expected_seq);

        seq.high = i32::MAX - 1;
        seq.low = 0;
        seq += i32::MAX;
        expected_seq.high = i32::MAX - 1;
        expected_seq.low = i32::MAX as u32;
        assert_eq!(seq, expected_seq);

        seq += i32::MAX;
        expected_seq.low = u32::MAX - 1;
        assert_eq!(seq, expected_seq);

        seq += i32::MAX;
        expected_seq.high = i32::MAX;
        expected_seq.low = i32::MAX as u32 - 2;
        assert_eq!(seq, expected_seq);
    }

    #[test]
    fn equal_operator_test() {
        let mut seq = SequenceNumber_t::new(10, 4356);
        let seq2 = SequenceNumber_t::new(10, 4356);

        assert_eq!(seq == seq2, true);

        seq.high = 345;
        assert_eq!(seq == seq2, false);

        seq.high = 1;
        assert_eq!(seq == seq2, false);

        seq.high = 10;
        seq.low = 60000;
        assert_eq!(seq == seq2, false);

        seq.low = 100;
        assert_eq!(seq == seq2, false);
    }

    #[test]
    fn not_equal_operator_test() {
        let mut seq = SequenceNumber_t::new(10, 4356);
        let seq2 = SequenceNumber_t::new(10, 4356);

        assert_eq!(seq != seq2, false);

        seq.high = 345;
        assert_eq!(seq != seq2, true);

        seq.high = 1;
        assert_eq!(seq != seq2, true);

        seq.high = 10;
        seq.low = 60000;
        assert_eq!(seq != seq2, true);

        seq.low = 100;
        assert_eq!(seq != seq2, true);
    }

    #[test]
    fn greater_than_operator_test() {
        let mut seq = SequenceNumber_t::new(10, 4356);
        let seq2 = SequenceNumber_t::new(10, 4356);

        assert_eq!(seq > seq2, false);

        seq.high = 345;
        assert_eq!(seq > seq2, true);

        seq.high = 1;
        assert_eq!(seq > seq2, false);

        seq.high = 10;
        seq.low = 60000;
        assert_eq!(seq > seq2, true);

        seq.low = 100;
        assert_eq!(seq > seq2, false);
    }

    #[test]
    fn less_than_operator_test() {
        let mut seq = SequenceNumber_t::new(10, 4356);
        let seq2 = SequenceNumber_t::new(10, 4356);

        assert_eq!(seq < seq2, false);

        seq.high = 345;
        assert_eq!(seq < seq2, false);

        seq.high = 1;
        assert_eq!(seq < seq2, true);

        seq.high = 10;
        seq.low = 60000;
        assert_eq!(seq < seq2, false);

        seq.low = 100;
        assert_eq!(seq < seq2, true);
    }

    #[test]
    fn greater_than_or_equal_operator_test() {
        let mut seq = SequenceNumber_t::new(10, 4356);
        let seq2 = SequenceNumber_t::new(10, 4356);

        assert_eq!(seq >= seq2, true);

        seq.high = 345;
        assert_eq!(seq >= seq2, true);

        seq.high = 1;
        assert_eq!(seq >= seq2, false);

        seq.high = 10;
        seq.low = 60000;
        assert_eq!(seq >= seq2, true);

        seq.low = 100;
        assert_eq!(seq >= seq2, false);
    }

    #[test]
    fn less_than_or_equal_operator_test() {
        let mut seq = SequenceNumber_t::new(10, 4356);
        let seq2 = SequenceNumber_t::new(10, 4356);

        assert_eq!(seq <= seq2, true);

        seq.high = 345;
        assert_eq!(seq <= seq2, false);

        seq.high = 1;
        assert_eq!(seq <= seq2, true);

        seq.high = 10;
        seq.low = 60000;
        assert_eq!(seq <= seq2, false);

        seq.low = 100;
        assert_eq!(seq <= seq2, true);
    }

    #[test]
    fn subtraction_operator_test() {
        let mut seq = SequenceNumber_t::new(4, 3);
        seq = seq - 7;

        let mut expected_seq = SequenceNumber_t::new(3, u32::MAX - 3);

        assert_eq!(seq, expected_seq);

        seq.high = i32::MAX;
        seq.low = u32::MAX - 1;
        seq = seq - i32::MAX as u32;

        expected_seq.high = i32::MAX;
        expected_seq.low = i32::MAX as u32;
        assert_eq!(seq, expected_seq);

        seq.high = 25;
        seq.low = u32::MAX;
        seq = seq - u32::MAX;

        expected_seq.high = 25;
        expected_seq.low = 0;
        assert_eq!(seq, expected_seq);

        seq = seq - u32::MAX;

        expected_seq.high = 24;
        expected_seq.low = 1;

        assert_eq!(seq, expected_seq);
    }

    #[test]
    fn addition_operator() {
        let mut seq = SequenceNumber_t::new(3, u32::MAX - 3);
        seq = seq + 7;

        let mut expected_seq = SequenceNumber_t::new(4, 3);

        assert_eq!(seq, expected_seq);

        seq = seq + i32::MAX as u32;
        expected_seq.low = i32::MAX as u32;
        expected_seq.low += 3;
        assert_eq!(seq, expected_seq);

        seq = seq + i32::MAX as u32;
        expected_seq.high = 5;
        expected_seq.low = 1;
        assert_eq!(seq, expected_seq);

        seq.high = i32::MAX - 1;
        seq.low = 0;
        seq = seq +  i32::MAX as u32;
        expected_seq.high = i32::MAX - 1;
        expected_seq.low = i32::MAX as u32;
        assert_eq!(seq, expected_seq);

        seq = seq + i32::MAX as u32;
        expected_seq.low = u32::MAX - 1;
        assert_eq!(seq, expected_seq);

        seq = seq + i32::MAX as u32;
        expected_seq.high = i32::MAX;
        expected_seq.low = i32::MAX as u32 - 2;
        assert_eq!(seq, expected_seq);

        seq.high = 24;
        seq.low = 1;
        seq = seq + u32::MAX;
        expected_seq.high = 25;
        expected_seq.low = 0;
        assert_eq!(seq, expected_seq);

        seq = seq + u32::MAX;
        expected_seq.low = u32::MAX;
        assert_eq!(seq, expected_seq);
    }

    #[test]
    fn subtraction_between_ses_operator_test() {
        let mut minuend = SequenceNumber_t::new(4, 3);
        let mut subtrahend = SequenceNumber_t::new(0, 7);
        let mut res = minuend - subtrahend;
        let mut expected_seq = SequenceNumber_t::new(3, u32::MAX - 3);

        assert_eq!(res, expected_seq);

        minuend.high = i32::MAX;
        minuend.low = u32::MAX - 1;

        subtrahend.high = 0;
        subtrahend.low = i32::MAX as u32;

        res = minuend - subtrahend;

        expected_seq.high = i32::MAX;
        expected_seq.low = i32::MAX as u32;
        assert_eq!(res, expected_seq);

        minuend.high = 25;
        minuend.low = u32::MAX;

        subtrahend.high = 0;
        subtrahend.low = u32::MAX;

        res = minuend - subtrahend;

        expected_seq.high = 25;
        expected_seq.low = 0;
        assert_eq!(res, expected_seq);

        res = res - subtrahend;

        expected_seq.high = 24;
        expected_seq.low = 1;
        assert_eq!(res, expected_seq);
    }

    #[test]
    fn common_test() {
        let mut s1 = SequenceNumber_t { high: 1, low: 1 };
        let mut s2 = SequenceNumber_t { high: 1, low: 1 };
        assert_eq!(s1 == s2, true);

        s2 += 1;
        assert_eq!(s1 < s2, true);
        assert_eq!(s2 >= s1, true);
        assert_eq!(s1 != s2, true);

        s2 = s1;
        assert_eq!(s1 == s2, true);
        s2 = s2 + 1;
        assert_eq!(s1 <= s2, true);

        s2 = s1;
        assert_eq!(s1 == s2, true);
        s2 = s2 - 1;
        assert_eq!(s1 >= s2, true);

        s2 = s1;
        assert_eq!(s1 == s2, true);
        s2 = s2 - s1;
        let mut s3 = SequenceNumber_t { high: 0, low: 0 };
        assert_eq!(s2 == s3, true);

        s2 = SequenceNumber_t { high: 2, low: 0 };
        s3 = s2 - s1;
        assert_eq!(
            s3,
            SequenceNumber_t {
                high: 0,
                low: u32::MAX
            }
        );

        s2 = SequenceNumber_t { high: 3, low: 0 };
        s3 = SequenceNumber_t {
            high: 2,
            low: u32::MAX,
        };
        s2 = s2 - 1;
        assert_eq!(s2 == s3, true);

        s1 = SequenceNumber_t { high: 0, low: 1 };
        s2 = SequenceNumber_t {
            high: 0,
            low: u32::MAX,
        };
        s3 = SequenceNumber_t { high: 1, low: 0 };
        s2 = s2 + s1;
        assert_eq!(s2 == s3, true);

        s2 = SequenceNumber_t {
            high: 0,
            low: u32::MAX,
        };
        s3 = SequenceNumber_t { high: 1, low: 0 };
        s2 = s2 + 1;
        assert_eq!(s2 == s3, true);

        s1 = SequenceNumber_t { high: 0, low: 1 };
        s2 = SequenceNumber_t { high: 0, low: 0 };
        assert_eq!(SequenceNumber_t::sort_seqNum(s1, s2), false);
        assert_eq!(SequenceNumber_t::sort_seqNum(s2, s1), true);
    }
}
