use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn shuffle_vec<T>(v: &mut Vec<T>) {
    v.shuffle(&mut thread_rng());
}
