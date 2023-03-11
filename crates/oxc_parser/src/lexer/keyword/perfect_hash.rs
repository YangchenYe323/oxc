/// An efficient hashing function for Javascript's keywords which all have
/// unique (first 2 bytes, last 2 bytes) tuples. This function takes that advantage
/// and uses this tuple as a u32 hash value of the string.
#[inline]
pub(crate) unsafe fn select(bytes: &[u8]) -> u32 {
    let len = bytes.len();
    fn read_16_bits(buf: &[u8]) -> u32 {
        (buf[0] as u32) | (buf[1] as u32) << 8
    }
    // Take the first 2 characters.
    let lo = read_16_bits(bytes);
    // Take the last 2 characters.
    let hi = read_16_bits(&bytes[len - 2..]);
    hi << 16 | lo
}

/// A very fast hash function but generates a lot of collisions.
/// We will try at build time different seed until we find one without
/// collisions.
#[inline]
pub(crate) fn mix(selection: u32, seed: u64) -> u32 {
    const MAGIC: u64 = 4292484099903637661;
    let x = (selection as u64) ^ seed;
    let res = multiply_u64_get_top_64(x, MAGIC);
    res as u32
}

#[inline]
fn multiply_u64_get_top_64(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) >> 64) as u64
}
