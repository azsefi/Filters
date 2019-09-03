use bit_set::BitSet;

pub struct BloomFilter {
    pub n_hashes: u16,
    pub n_bits: u64,
    bit_set: BitSet
}


fn main() {
    println!("Hello, world!");
}
