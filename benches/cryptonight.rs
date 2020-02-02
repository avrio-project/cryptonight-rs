use std::str;
#[macro_use]

extern crate bencher;
extern crate cryptonight;
use bencher::Bencher;
use cryptonight::cryptonight;

fn benchmark_print() {
    let bytes = [1u8; 43];
    let out = cryptonight(&bytes, bytes.len(), 1);
    let mut out_raw: u64 = 0;
    let out_string: String = str::from_utf8(&out).unwrap().to_string();
    println!("{}", out_string)
}

fn benchmark_cryptonight_43_1(bench: &mut Bencher){
    benchmark_print();
    let bytes = [1u8; 43];
    bench.iter(|| cryptonight(&bytes, bytes.len(), 1));
}

fn benchmark_cryptonight_1k_1(bench: &mut Bencher){
    let bytes = [1u8; 1024];
    bench.iter(|| cryptonight(&bytes, bytes.len(), 1));
}

fn benchmark_cryptonight_64k_1(bench: &mut Bencher){
    let bytes = [1u8; 65536];
    bench.iter(|| cryptonight(&bytes, bytes.len(), 1));
}

fn benchmark_cryptonight_43_0(bench: &mut Bencher){
    let bytes = [1u8; 43];
    bench.iter(|| cryptonight(&bytes, bytes.len(), 0));
}

fn benchmark_cryptonight_1k_0(bench: &mut Bencher){
    let bytes = [1u8; 1024];
    bench.iter(|| cryptonight(&bytes, bytes.len(), 0));
}

fn benchmark_cryptonight_64k_0(bench: &mut Bencher){
    let bytes = [1u8; 65536];
    bench.iter(|| cryptonight(&bytes, bytes.len(), 0));
}

benchmark_group!(benches, 
benchmark_cryptonight_43_1, 
benchmark_cryptonight_1k_1, 
benchmark_cryptonight_64k_1,
benchmark_cryptonight_43_0,
benchmark_cryptonight_1k_0,
benchmark_cryptonight_64k_0);
benchmark_main!(benches);
