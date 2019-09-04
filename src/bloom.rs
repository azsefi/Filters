use bit_set::BitSet;
use std::collections::hash_map::{RandomState, DefaultHasher};
use std::hash::{BuildHasher, Hasher, Hash};


pub struct BloomFilter {
    n_hashes: u16,
    n_bits: usize,
    bit_set: BitSet,
    states: Vec<RandomState>
}

impl BloomFilter {
    pub fn new(false_positive_rate: f64, expected_item_count: u64) -> Self {
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

    pub fn n_hashes(&self) -> u16 {
        self.n_hashes
    }

    pub fn n_bits(&self) -> usize {
        self.n_bits
    }

    fn get_hash_count(false_positive_rate: f64) -> u16 {
        -false_positive_rate.log2().ceil() as u16
    }

    fn get_bit_count(n_hashes: u16, expected_item_count: u64) -> usize {
        ((expected_item_count as f64) * (14.4 as f64) * (n_hashes as f64)) as usize
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


#[cfg(test)]
pub mod tests {
    use super::*;
    use rand::distributions::{Distribution, Uniform};
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn test_bloom_filter_parameters() {
        let mut bloom = BloomFilter::new(0.1, 100);
        assert_eq!(bloom.n_hashes(), 3);
        assert_eq!(bloom.n_bits(), 4320);
    }

    #[test]
    fn test_bloom_filter_accuracy() {
        let between = Uniform::new(0, 10000000);
        let mut rng = rand::thread_rng();

        let n_inputs = 100000;

        let inputs: HashSet<u64>  = HashSet::from_iter((0..n_inputs).map(|_n| between.sample(&mut rng)));

        let mut bloom = BloomFilter::new(0.1, n_inputs);
        inputs.iter().for_each(|n| bloom.put(n));

        assert!(inputs.iter().all(|n| bloom.contains(n)));

        let false_inputs: Vec<u64> = (0..n_inputs).map(|_n| between.sample(&mut rng)).filter(|n| !inputs.contains(n)).collect();
        let false_error_rate: f64 = (false_inputs.iter().map(|n| bloom.contains(n) as u16).sum::<u16>() as f64) / false_inputs.len() as f64;

        assert!(false_error_rate < 0.1_f64, false_error_rate);
    }
}