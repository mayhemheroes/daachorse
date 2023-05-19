#![no_main]
use libfuzzer_sys::fuzz_target;
use daachorse::DoubleArrayAhoCorasick;
use std::hash::Hash;
use std::collections::HashSet;

fn dedup<T: Eq + Hash + Copy>(v: &mut Vec<T>) {
    let mut uniques: HashSet<T> = HashSet::new();
    v.retain(|e| uniques.insert(*e));
}

fuzz_target!(|input: (Vec<String>, String)| {
    let mut data: Vec<&str> = input.0.iter().map(|x| x.as_str()).collect();
    dedup(&mut data);
    if data.len() >= 1 && data.iter().all(|x| x.len() > 0) {
        let pma: DoubleArrayAhoCorasick<u32> = DoubleArrayAhoCorasick::new(data).unwrap();
        pma.find_overlapping_iter(input.1);
    }
});