use std::str;
extern crate hex;

#[macro_use]

extern crate bencher;
extern crate cryptonight;
use bencher::Bencher;
use cryptonight::cryptonight;

fn benchmark_print() {
    let input = "0100fb8e8ac805899323371bb790db19218afd8db8e3755d8b90f39b3d5506a9abce4fa912244500000000ee8146d49fa93ee724deb57d12cbc6c6f3b924d946127c7a97418f9348828f0f02".as_bytes();
    cryptonight::set_params(65536, 32768);
    let out = cryptonight(&input, input.len(), 0);
    println!("{:?}", out);
    let s = hex::encode(out);

    println!("result: {}", s);
}
fn benchmark_cryptonight_43_1(bench: &mut Bencher){
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
