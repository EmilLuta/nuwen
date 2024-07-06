mod operations;
mod reed_solomon_fingerprinting;
mod vector_generator;

use reed_solomon_fingerprinting::ReedSolomonFingerprint;
use vector_generator::generate_random_vector;

fn main() {
    let data = generate_random_vector(1000);
    let rs_fingerprint = ReedSolomonFingerprint::new(&data);
    let mut same_data = data.clone();
    assert!(rs_fingerprint.equal(&same_data));
    assert!(rs_fingerprint.perm(&same_data));
    let tmp = same_data[0];
    same_data[0] = same_data[1];
    same_data[1] = tmp;
    assert!(!rs_fingerprint.equal(&same_data));
    assert!(rs_fingerprint.perm(&same_data));

    println!("{:?}", rs_fingerprint);
}
