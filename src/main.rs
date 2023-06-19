#[cfg(test)]
mod tests {

    use hex::{decode, encode, FromHex};

    #[test]
    fn test_x25519() {
        use x25519_dalek::{PublicKey, StaticSecret};
        let privateKey = StaticSecret::from(
            <[u8; 32]>::from_hex(
                "880237bde777a014c5a8cb6f25734168919cda52a8789e483c95acac76b38478",
            )
            .unwrap(),
        );
        let publicKey = PublicKey::from(&privateKey);
        let pubKeyHex = encode(&publicKey.as_bytes());
        assert_eq!(
            pubKeyHex,
            "8a7e5a25f7f6d88c53530547ad0e676ae10fdf2e18f2f7281d6d8791ef6ce042"
        );
    }

    #[test]
    fn test_ed25519() {
        use ed25519_dalek::{PublicKey, SecretKey};
        let privateKey = SecretKey::from_bytes(
            &decode("880237bde777a014c5a8cb6f25734168919cda52a8789e483c95acac76b38478").unwrap(),
        )
        .unwrap();

        let publicKey = PublicKey::from(&privateKey);
        let pubKeyHex = encode(&publicKey.as_bytes());
        assert_eq!(
            pubKeyHex,
            "1d91ebfc2aea5c371947d0dfefd6a2153da94e9e6077f40ac90b4ba63b668d19"
        );
    }
}
