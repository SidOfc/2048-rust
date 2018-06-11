// a 64bit mask with a single section of 16 bits set to 0.
// used to extract a "horizontal slice" out of a 64 bit integer.
pub static ROW_MASK: u64 = 0x0000_0000_0000_FFFF_u64;

// a 64bit mask with 4 sections each starting after the n * 16th bit.
// used to extract a "vertical slice" out of a 64 bit integer.
pub static COL_MASK: u64 = 0x000F_000F_000F_000F_u64;
