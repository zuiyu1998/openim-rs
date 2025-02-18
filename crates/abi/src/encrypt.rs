pub fn md5(bytes: &[u8]) -> String {
    use md5::{Digest, Md5};

    let mut hasher = Md5::new();
    hasher.update(bytes);

    let result = hasher.finalize().to_vec();

    return String::from_utf8_lossy(&result).to_string(); 
}
