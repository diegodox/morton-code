#[cfg(target_pointer_width = "64")]
const MASK: usize =
    0b0_001_001_001_001_001_001_001_001_001_001_001_001_001_001_001_001_001_001_001_001_001;

#[cfg(target_pointer_width = "32")]
const MASK: usize = 0b00_001_001_001_001_001_001_001_001_001_001;

/// Number of bits for usize.
const NUM_BITS_USIZE: usize = std::mem::size_of::<usize>() * 8;

/// Max number of depth
#[allow(dead_code)]
const MAX_DEPTH: usize = NUM_BITS_USIZE / 3;

#[cfg(target_pointer_width = "64")]
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// 3-dimension morton code(zyx), 21-level, first bit is used for 1-bit flag.
pub struct Morton3D(usize);

#[cfg(target_pointer_width = "32")]
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// 3-dimension morton code(zyx), 10-level, first bit is used for 1-bit flag.
///
/// To be compatible with 64-bit Morton3D, second bit is not used.
pub struct Morton3D(usize);

impl Morton3D {
    pub const fn is_flag_set(self) -> bool {
        (self.0 >> (NUM_BITS_USIZE - 1)) == 1
    }
    pub fn set_flag(&mut self) {
        self.0 |= 1 << (NUM_BITS_USIZE - 1)
    }
    pub fn unset_flag(&mut self) {
        self.0 &= !(1 << (NUM_BITS_USIZE - 1))
    }

    /// generate mask bits
    const fn mask_n(n: usize) -> usize {
        MASK << (n % 3)
    }

    /// decrease n-th dim (0: x, 1: y, 2: z) morton code,
    /// panic if it can't
    const fn decrease_nth_dim(self, n: usize) -> Self {
        Self((((self.0 & Self::mask_n(n)) - 1) & Self::mask_n(n)) | ((self.0) & !Self::mask_n(n)))
    }
    /// increase n-th dim (0: x, 1: y, 2: z) morton code,
    /// panic if it can't
    const fn increase_nth_dim(self, n: usize) -> Self {
        Self((((self.0 | !Self::mask_n(n)) + 1) & Self::mask_n(n)) | ((self.0) & !Self::mask_n(n)))
    }

    pub const fn decrease_x(self) -> Self {
        self.decrease_nth_dim(0)
    }
    pub const fn decrease_y(self) -> Self {
        self.decrease_nth_dim(1)
    }
    pub const fn decrease_z(self) -> Self {
        self.decrease_nth_dim(2)
    }

    pub const fn increase_x(self) -> Self {
        self.increase_nth_dim(0)
    }
    pub const fn increase_y(self) -> Self {
        self.increase_nth_dim(1)
    }
    pub const fn increase_z(self) -> Self {
        self.increase_nth_dim(2)
    }
}

impl From<usize> for Morton3D {
    fn from(v: usize) -> Self {
        Self(v)
    }
}

impl Into<usize> for Morton3D {
    fn into(self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::MAX_DEPTH;

    use super::Morton3D;

    #[test]
    fn test_flag() {
        let mut morton = Morton3D(0);
        morton.set_flag();
        assert!(morton.is_flag_set());
        morton.unset_flag();
        assert_eq!(morton, Morton3D(0));
        assert!(!morton.is_flag_set());
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn test_max() {
        let mut morton = Morton3D(0);
        for _ in 0..(2_u64.pow(MAX_DEPTH.try_into().unwrap()) - 1) {
            morton = morton.increase_x();
            morton = morton.increase_y();
            morton = morton.increase_z();
        }
        // println!("actual: 0b{:064b}", morton.0);
        // println!("should: 0b0111111111111111111111111111111111111111111111111111111111111111");
        assert_eq!(morton, Morton3D(0b0_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111_111));
    }

    #[test]
    #[cfg(target_pointer_width = "32")]
    fn test_max() {
        let mut morton = Morton3D(0);
        for _ in 0..(2_u64.pow(MAX_DEPTH.try_into().unwrap()) - 1) {
            morton = morton.increase_x();
            morton = morton.increase_y();
            morton = morton.increase_z();
        }
        // println!("actual: 0b{:064b}", morton.0);
        // println!("should: 0b0111111111111111111111111111111111111111111111111111111111111111");
        assert_eq!(
            morton,
            Morton3D(0b00_111_111_111_111_111_111_111_111_111_111)
        );
    }

    #[test]
    fn test_inc() {
        let morton = Morton3D(0b000_001);

        let morton = morton.increase_y();
        assert_eq!(morton, Morton3D(0b000_011));

        let morton = morton.increase_y();
        assert_eq!(morton, Morton3D(0b010_001));

        let morton = morton.increase_y();
        assert_eq!(morton, Morton3D(0b010_011));

        let morton = morton.increase_y().increase_z();
        assert_eq!(morton, Morton3D(0b010_000_101));
    }

    #[test]
    fn test_dec() {
        let morton = Morton3D(0b010_000_101);

        let morton = morton.decrease_y();
        assert_eq!(morton, Morton3D(0b000_010_111));

        let morton = morton.decrease_y();
        assert_eq!(morton, Morton3D(0b000_010_101));

        let morton = morton.decrease_z().decrease_y();
        assert_eq!(morton, Morton3D(0b000_000_011));

        let morton = morton.decrease_y();
        assert_eq!(morton, Morton3D(0b000_000_001));
    }
}
