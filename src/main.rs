use bit_set::BitSet;
use std::collections::hash_map::{RandomState, DefaultHasher};
use std::hash::{BuildHasher, Hasher, Hash};

pub struct BloomFilter {
    pub n_hashes: u16,
    pub n_bits: usize,
    bit_set: BitSet,
    states: Vec<RandomState>
}

impl BloomFilter {
    pub fn new(false_positive_rate: f64, expected_item_count: u64) -> BloomFilter {
        let n_hashes = BloomFilter::get_hash_count(false_positive_rate);
        let n_bits = BloomFilter::get_bit_count(n_hashes, expected_item_count);
        let bit_set = BitSet::with_capacity(n_bits);
        let states: Vec<RandomState> = (0..n_hashes).map(|_x| RandomState::new()).collect();

        BloomFilter { n_hashes, n_bits, bit_set, states }
    }

    pub fn put<T: Hash>(&mut self, value: T) {
        let bit_set = &mut self.bit_set;

        Self::get_bits(&self.states, &value, &self.n_bits).
            for_each(|bit| { bit_set.insert(bit); })
    }

    pub fn contains<T: Hash>(&self, value: T) -> bool {
        Self::get_bits(&self.states, &value, &self.n_bits)
            .all(|bit| self.bit_set.contains(bit))
    }

    fn get_hash_count(false_positive_rate: f64) -> u16 {
        -false_positive_rate.log2().ceil() as u16
    }

    fn get_bit_count(n_hashes: u16, expected_item_count: u64) -> usize {
        (expected_item_count as usize) * (1.44 as usize) * (n_hashes as usize)
    }

    fn hash<T: Hash>(value: &T, mut hasher: DefaultHasher) -> u64 {
        value.hash(&mut hasher);
        hasher.finish()
    }

    fn get_bits<'b, T:'b + Hash>(states: &'b Vec<RandomState>, value: &'b T, n_bits: &'b usize)
        -> impl Iterator<Item = usize> + 'b {
        states.iter().
            map(move |state| Self::hash(value, state.build_hasher())).
            map(move |hash_value| (hash_value as usize) % n_bits)
    }
}


fn main() {
    let words = vec!['a', 'b', 'c'];
    let mut bloom = BloomFilter::new(0.1, 100);
    words.iter().for_each(|c| bloom.put(c));

    println!("{}", bloom.contains('a'));
}
