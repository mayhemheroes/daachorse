#![no_main]
use libfuzzer_sys::fuzz_target;
use daachorse::DoubleArrayAhoCorasick;

fuzz_target!(|input: Vec<String>| {
    if input.len() >= 1 && input.iter().all(|x| x.len() > 0) {
        let _: DoubleArrayAhoCorasick<u32> = DoubleArrayAhoCorasick::new(input).unwrap();
    }
});