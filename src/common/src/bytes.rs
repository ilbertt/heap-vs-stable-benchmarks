use crate::rand::random_bytes;

const ONE_KB: usize = 1024;
const TEN_KB: usize = 10 * ONE_KB;
const HUNDRED_KB: usize = 100 * ONE_KB;
const ONE_MB: usize = ONE_KB * ONE_KB;
const TWO_MB: usize = 2 * ONE_MB;

pub fn zero_bytes() -> Vec<u8> {
    vec![0u8]
}

pub fn random_1kb() -> Vec<u8> {
    random_bytes(ONE_KB)
}

pub fn random_10kb() -> Vec<u8> {
    random_bytes(TEN_KB)
}

pub fn random_100kb() -> Vec<u8> {
    random_bytes(HUNDRED_KB)
}

pub fn random_1mb() -> Vec<u8> {
    random_bytes(ONE_MB)
}

pub fn random_2mb() -> Vec<u8> {
    random_bytes(TWO_MB)
}
