const CONVERSION_ERROR: &'static str = "conversion from u64 to u32 was not lossless";

pub(crate) fn mul_mod_u32(a: u32, b: u32, m: u32) -> u32 {
    let result = ((a as u64) * (b as u64)) % (m as u64);
    u32::try_from(result).expect(CONVERSION_ERROR)
}

pub(crate) fn add_mod_u32(a: u32, b: u32, m: u32) -> u32 {
    let result = ((a as u64) + (b as u64)) % (m as u64);
    u32::try_from(result).expect(CONVERSION_ERROR)
}

pub(crate) fn sub_mod_u32(a: u32, b: u32, m: u32) -> u32 {
    let mut a = a as u64;
    let b = b as u64;
    if a < b {
        a += m as u64;
    }
    let result = (a - b) % (m as u64);
    u32::try_from(result).expect(CONVERSION_ERROR)
}
