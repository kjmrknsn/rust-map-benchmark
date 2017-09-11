extern crate fnv;
extern crate rand;

use fnv::FnvHashMap;
use rand::Rng;
use std::collections::HashMap;
use std::ops::{AddAssign, Div};
use std::time::{Duration, Instant};

fn bench_hash_map(n: usize, key_size: usize) {
    let mut total = Duration::new(0, 0);
    let mut rng = rand::thread_rng();
    let mut keys = Vec::with_capacity(n);
    let mut m = HashMap::with_capacity(n);
    for _ in 0..n {
        let mut key = Vec::with_capacity(key_size);
        for _ in 0..key_size {
            key.push(rng.gen::<u8>());
        }
        keys.push(key.clone());
    }
    for key in &keys {
        let ins = Instant::now();
        m.insert(key, 1);
        total.add_assign(ins.elapsed());
    }
    println!("hash_map_insert, key_size: {}, total: {:?}, average: {:?}", key_size, total, total.div(n as u32));

    let mut total = Duration::new(0, 0);
    let mut r = 0;
    for key in &keys {
        let ins = Instant::now();
        if let Some(x) = m.get(&key) {
            r += *x;
        }
        total.add_assign(ins.elapsed());
    }
    println!("hash_map_get, key_size: {}, total: {:?}, average: {:?}, r: {}", key_size, total, total.div(n as u32), r);
}

fn bench_fnv_hash_map(n: usize, key_size: usize) {
    let mut total = Duration::new(0, 0);
    let mut rng = rand::thread_rng();
    let mut keys = Vec::with_capacity(n);
    let mut m = FnvHashMap::with_capacity_and_hasher(n, Default::default());
    for _ in 0..n {
        let mut key = Vec::with_capacity(key_size);
        for _ in 0..key_size {
            key.push(rng.gen::<u8>());
        }
        keys.push(key.clone());
    }
    for key in &keys {
        let ins = Instant::now();
        m.insert(key, 1);
        total.add_assign(ins.elapsed());
    }
    println!("fnv_hash_map_insert, key_size: {}, total: {:?}, average: {:?}", key_size, total, total.div(n as u32));

    let mut total = Duration::new(0, 0);
    let mut r = 0;
    for key in &keys {
        let ins = Instant::now();
        if let Some(x) = m.get(&key) {
            r += *x;
        }
        total.add_assign(ins.elapsed());
    }
    println!("fnv_hash_map_get, key_size: {}, total: {:?}, average: {:?}, r: {}", key_size, total, total.div(n as u32), r);
}

fn main() {
    let n = 10000;
    let key_sizes = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];

    println!("n: {}", n);

    for key_size in key_sizes.iter() {
        bench_hash_map(n, *key_size);
        bench_fnv_hash_map(n, *key_size);
    }
}
