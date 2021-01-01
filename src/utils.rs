use data_encoding::HEXUPPER;
use sha2::{Digest, Sha256, Sha512};

pub fn hasher(content: &str) -> String {
    let mut hasher = Sha256::new();

    // write input message
    hasher.update(&content);

    // read hash digest and consume hasher
    let hash = hasher.finalize();
    HEXUPPER.encode(&hash)
}
