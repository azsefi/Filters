extern crate filters;

use filters::bloom::BloomFilter;

fn main() {
    let words = vec!['a', 'b', 'c'];
    let mut bloom = BloomFilter::new(0.1, 100);
    words.iter().for_each(|c| bloom.put(c));

    println!("n_bits: {}", bloom.n_bits());
    println!("n_hashes: {}", bloom.n_hashes());
}
