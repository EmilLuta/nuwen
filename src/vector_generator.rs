use rand::{distributions::Uniform, Rng};

/// Generates a vector of specified size with random values.
/// Simulates binary data read from some file.
pub(crate) fn generate_random_vector(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, u8::MAX);
    (0..size).map(|_| rng.sample(&range)).collect()
}
