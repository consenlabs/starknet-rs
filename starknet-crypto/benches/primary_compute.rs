use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hex_literal::hex;
use starknet_crypto::FieldElement;
use starknet_crypto::ec_point::EcPoint;
use starknet_crypto::pedersen_params::CONSTANT_POINTS;


pub fn criterion_benchmark(c: &mut Criterion) {

    let mut pk_data = hex!("04a724706e80e5ea88b9ee60a7ede83cbc2de27da0659bef2929381a298b672d");
    let mut pri_key = FieldElement::from_hex_be("04a724706e80e5ea88b9ee60a7ede83cbc2de27da0659bef2929381a298b672d").unwrap();

    let pub_key =  (&CONSTANT_POINTS[1]).multiply(&pri_key.to_bits_le());

    c.bench_function("seckey_invert", |b| {
        b.iter(|| {
            black_box(pri_key.invert()).unwrap();
        });
    });

    c.bench_function("pubkey_mul", |b| {
        b.iter(|| {
            black_box(pub_key.multiply(&pri_key.to_bits_le()));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
