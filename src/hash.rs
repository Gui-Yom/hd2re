use std::hash::{BuildHasher, Hasher};

/// murmur64a with a seed of 0
pub(crate) fn stringray_hash(key: &[u8]) -> u64 {
    let m: u64 = 0xc6a4a7935bd1e995;
    let r: u8 = 47;

    let len = key.len();
    let mut h: u64 = (len as u64).wrapping_mul(m);

    let endpos = len - (len & 7);
    let mut i = 0;
    while i != endpos {
        let mut k: u64;

        k = key[i + 0] as u64;
        k |= (key[i + 1] as u64) << 8;
        k |= (key[i + 2] as u64) << 16;
        k |= (key[i + 3] as u64) << 24;
        k |= (key[i + 4] as u64) << 32;
        k |= (key[i + 5] as u64) << 40;
        k |= (key[i + 6] as u64) << 48;
        k |= (key[i + 7] as u64) << 56;

        k = k.wrapping_mul(m);
        k ^= k >> r;
        k = k.wrapping_mul(m);
        h ^= k;
        h = h.wrapping_mul(m);

        i += 8;
    }

    let over = len & 7;
    if over == 7 {
        h ^= (key[i + 6] as u64) << 48;
    }
    if over >= 6 {
        h ^= (key[i + 5] as u64) << 40;
    }
    if over >= 5 {
        h ^= (key[i + 4] as u64) << 32;
    }
    if over >= 4 {
        h ^= (key[i + 3] as u64) << 24;
    }
    if over >= 3 {
        h ^= (key[i + 2] as u64) << 16;
    }
    if over >= 2 {
        h ^= (key[i + 1] as u64) << 8;
    }
    if over >= 1 {
        h ^= key[i + 0] as u64;
    }
    if over > 0 {
        h = h.wrapping_mul(m);
    }

    h ^= h >> r;
    h = h.wrapping_mul(m);
    h ^= h >> r;
    h
}

#[derive(Default)]
pub(crate) struct NoHash;
pub(crate) struct NoHashHasher(u64);

impl Hasher for NoHashHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        unreachable!()
    }

    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
}

impl BuildHasher for NoHash {
    type Hasher = NoHashHasher;

    fn build_hasher(&self) -> Self::Hasher {
        NoHashHasher(0)
    }
}