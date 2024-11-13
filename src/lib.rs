#[cfg(any(not(target_os = "solana"), feature = "no-syscall"))]
use dashu::{integer::UBig, integer::fast_div::ConstDivisor};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BigModExpParams {
    base: *const u8,
    base_len: usize,
    exponent: *const u8,
    exponent_len: usize,
    modulus: *const u8,
    modulus_len: usize,
}

#[cfg(all(not(feature="no-syscall"), target_os = "solana"))]
extern "C" {
    fn sol_big_mod_exp(
        param: *const BigModExpParams,
        return_value: *mut u8,
    ) -> u64;
}

#[cfg(all(not(feature="no-syscall"), target_os = "solana"))]
#[inline(always)]
pub fn big_mod_exp_fixed<const N: usize>(base: &[u8], exponent: &[u8], modulus: &[u8;N]) -> [u8;N] {
    use std::mem::MaybeUninit;

    let mut return_value = MaybeUninit::<[u8;N]>::uninit();

    let params = BigModExpParams {
        base: base.as_ptr(),
        base_len: base.len(),
        exponent: exponent.as_ptr(),
        exponent_len: exponent.len(),
        modulus: modulus.as_ptr(),
        modulus_len: N,
    };

    unsafe {
        sol_big_mod_exp(&params as *const BigModExpParams, return_value.as_mut_ptr() as *mut u8);
        return_value.assume_init()
    }
}

#[cfg(all(not(feature="no-syscall"), target_os = "solana"))]
#[inline(always)]
pub fn big_mod_exp(base: &[u8], exponent: &[u8], modulus: &[u8]) -> Vec<u8> {
    let mut return_value = Vec::with_capacity(modulus.len());
    
    let params = BigModExpParams {
        base: base.as_ptr(),
        base_len: base.len(),
        exponent: exponent.as_ptr(),
        exponent_len: exponent.len(),
        modulus: modulus.as_ptr(),
        modulus_len: modulus.len(),
    };

    unsafe {
        sol_big_mod_exp(&params as *const BigModExpParams, return_value.as_mut_ptr() as *mut u8);
    }
    return_value
}

#[cfg(any(feature="no-syscall", not(target_os = "solana")))]
pub fn big_mod_exp(base: &[u8], exponent: &[u8], modulus: &[u8]) -> Vec<u8> {
    let ring = ConstDivisor::new(UBig::from_be_bytes(modulus));
    let base = ring.reduce(UBig::from_be_bytes(base));
    let result = base.pow(&UBig::from_be_bytes(exponent)).residue();
    result.to_be_bytes().to_vec()
}

#[cfg(any(feature="no-syscall", not(target_os = "solana")))]
pub fn big_mod_exp_fixed<const N: usize>(base: &[u8], exponent: &[u8], modulus: &[u8;N]) -> [u8;N] {
    let ring = ConstDivisor::new(UBig::from_be_bytes(modulus));
    let base = ring.reduce(UBig::from_be_bytes(base));
    let result = base.pow(&UBig::from_be_bytes(exponent)).residue().to_be_bytes();
    let mut output = [0u8;N];
    let start = N.saturating_sub(result.len());
    output[start..].copy_from_slice(&result);
    output
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn big_number_test() {
        let modulus = [
            0x9b, 0x24, 0x05, 0x12, 0xa4, 0x43, 0x50, 0x86, 0xc0, 0x05, 0xd0, 0xbb, 0xb3, 0x70,
            0x86, 0x6f, 0xe8, 0x21, 0xb1, 0xa4, 0x60, 0x4d, 0x22, 0xd3, 0x00, 0x5d, 0xb4, 0xd5,
            0x75, 0x05, 0x7c, 0xc1, 0x91, 0xf6, 0x2a, 0x0b, 0x46, 0x14, 0xa2, 0x7e, 0x5b, 0xf8,
            0xe9, 0x1b, 0x37, 0x5d, 0x35, 0x87, 0x87, 0xba, 0x27, 0xc7, 0xa8, 0xbc, 0xec, 0x97,
            0x2f, 0xd7, 0x40, 0x3f, 0xb7, 0xbf, 0x66, 0xd0, 0x57, 0xbc, 0x82, 0x8c, 0x34, 0xb4,
            0xe7, 0x56, 0xf7, 0x76, 0x3d, 0x95, 0x9f, 0x7f, 0x24, 0x1c, 0x1c, 0x24, 0xfb, 0x26,
            0x3d, 0x86, 0x0e, 0xce, 0xa6, 0xca, 0xe6, 0x72, 0xf7, 0x0a, 0x74, 0xf4, 0xb4, 0xe7,
            0x8c, 0xe2, 0xcb, 0x70, 0x19, 0xff, 0x02, 0xe8, 0xb8, 0xac, 0x01, 0xc1, 0x97, 0x5b,
            0x60, 0xd7, 0x3b, 0xcb, 0x41, 0x5d, 0x00, 0x75, 0x48, 0xfc, 0xe9, 0x02, 0x08, 0xfa,
            0x74, 0x22, 0x75, 0x9e, 0xf1, 0x33, 0x4c, 0x41, 0x4f, 0x36, 0x56, 0x46, 0x5e, 0xdb,
            0x0f, 0x5d, 0xac, 0x06, 0x9b, 0x5b, 0x3f, 0xce, 0xad, 0x52, 0x09, 0xa6, 0xe9, 0x6c,
            0xae, 0xe3, 0xd5, 0xa4, 0xda, 0x25, 0xd5, 0xee, 0xe7, 0xd7, 0x5e, 0x4d, 0xa0, 0x5e,
            0x60, 0xdf, 0xcd, 0x41, 0xcd, 0x49, 0x7f, 0x49, 0x23, 0xb6, 0xb4, 0x34, 0x69, 0x96,
            0xbd, 0x59, 0x44, 0xf0, 0xbb, 0xb0, 0xc9, 0x69, 0xf8, 0x40, 0x71, 0xe1, 0x09, 0xc0,
            0x30, 0x9a, 0x52, 0x16, 0x29, 0x85, 0x1e, 0x42, 0x2e, 0xe1, 0xb4, 0xe8, 0xc1, 0x3c,
            0x1d, 0xfa, 0x63, 0xbe, 0x51, 0xda, 0x89, 0x2d, 0xe4, 0x40, 0x64, 0xe1, 0xb0, 0x63,
            0x74, 0xad, 0x4b, 0xb7, 0xd2, 0x81, 0x67, 0x1a, 0x62, 0x86, 0xdf, 0xa9, 0x7d, 0xcb,
            0x58, 0x77, 0x1b, 0x37, 0x45, 0xc0, 0x41, 0xba, 0xe4, 0x95, 0x23, 0x1d, 0xd5, 0x95,
            0xbe, 0x4d, 0x46, 0xb9,
        ];

        let base = [
            0x10, 0x7d, 0x16, 0xed, 0xb2, 0x91, 0x33, 0xa0, 0x36, 0x4e, 0xb2, 0x22, 0xe2, 0x0a,
            0x6e, 0x8d, 0xca, 0xef, 0x64, 0x0e, 0x16, 0x69, 0x40, 0x82, 0x66, 0x6a, 0x47, 0xae,
            0xdb, 0x0a, 0x37, 0x69, 0x1a, 0xbf, 0x1a, 0x40, 0xf1, 0x62, 0xa6, 0xef, 0x9e, 0xc3,
            0x82, 0xe9, 0x9b, 0xcd, 0xa3, 0x1d, 0xf1, 0x48, 0xbe, 0xbe, 0xb7, 0x16, 0x1d, 0xe1,
            0x1c, 0xe8, 0x1e, 0xb3, 0x9c, 0x09, 0xbc, 0x3c, 0x65, 0x64, 0x39, 0x37, 0x28, 0x04,
            0xc3, 0xb6, 0xde, 0x66, 0xea, 0x54, 0x52, 0xbf, 0xb3, 0xa5, 0x35, 0x95, 0x02, 0xa0,
            0x7c, 0x2d, 0xb9, 0x57, 0xd1, 0x90, 0x67, 0x55, 0x60, 0x0e, 0xd6, 0xf0, 0xaa, 0x7b,
            0xd5, 0x5f, 0x8a, 0xfe, 0x25, 0xa9, 0xa8, 0x03, 0x40, 0x35, 0x96, 0xd7, 0xaf, 0x4b,
            0x36, 0x05, 0x09, 0x3c, 0x01, 0xae, 0xa4, 0x1e, 0x7c, 0x84, 0x9a, 0x45, 0x40, 0xb9,
            0x71, 0x22, 0x20, 0x60, 0xf9, 0x23, 0x22, 0x97, 0xba, 0xbb, 0x57, 0xe6, 0xc9, 0x09,
            0x25, 0x52, 0x4f, 0x70, 0x24, 0x40, 0x8d, 0x63, 0xee, 0x76, 0x8a, 0x8f, 0x71, 0x23,
            0x45, 0x0c, 0xfc, 0xf4, 0x95, 0x47, 0x3e, 0xaf, 0x76, 0x2d, 0x89, 0xa9, 0x30, 0x9a,
            0x8b, 0xaa, 0x33, 0x4e, 0x6e, 0x36, 0x41, 0x40, 0xe5, 0xdb, 0x13, 0xb8, 0x18, 0xf1,
            0x54, 0x6b, 0x19, 0x4f, 0x99, 0xfa, 0x9e, 0xd4, 0x56, 0x0f, 0x14, 0xc8, 0xcf, 0x02,
            0x9e, 0x08, 0xe2, 0x98, 0xe5, 0x1c, 0x21, 0x5f, 0x4b, 0x20, 0xc3, 0x4b, 0xb7, 0xc0,
            0x74, 0x8f, 0x29, 0x6a, 0x38, 0xd8, 0x08, 0xb4, 0x64, 0x12, 0x93, 0xb7, 0xf7, 0x3d,
            0xa2, 0xe3, 0xe2, 0xca, 0x5f, 0x84, 0x1c, 0x89, 0xcb, 0x9c, 0x6f, 0xa9, 0x27, 0x86,
            0x1f, 0xd7, 0x0c, 0x3c, 0x97, 0x2a, 0x36, 0x75, 0xfc, 0x6e, 0x5b, 0x09, 0x46, 0x93,
            0xb6, 0x63, 0x83, 0x3c,
        ];

        let msg = [0x2c, 0xf2, 0x4d, 0xba, 0x5f, 0xb0, 0xa3, 0x0e, 0x26, 0xe8, 0x3b, 0x2a, 0xc5, 0xb9, 0xe2, 0x9e, 0x1b, 0x16, 0x1e, 0x5c, 0x1f, 0xa7, 0x42, 0x5e, 0x73, 0x04, 0x33, 0x62, 0x93, 0x8b, 0x98, 0x24];

        let exponent = [0x01, 0x00, 0x01];

        let amount = big_mod_exp(&base, &exponent, &modulus);

        assert_eq!(amount[amount.len()-msg.len()..], msg);
    }
}