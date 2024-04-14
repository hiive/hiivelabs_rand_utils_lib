use blake2::digest::{Update, VariableOutput};
use blake2::Blake2bVar;

///
///
/// # Arguments
///
/// * `byte_vec`:
///
/// returns: [u8; 16]
pub fn create_seed_from_bytes(byte_vec: Vec<u8>) -> [u8; 16] {
    // Create a Blake2bVar hasher and input the byte vector.
    let mut hasher = Blake2bVar::new(16).unwrap();
    hasher.update(byte_vec.as_ref());
    let mut seed = [0u8; 16];
    hasher.finalize_variable(&mut seed).unwrap();
    seed
}
