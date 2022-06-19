
use std::collections::HashMap;
use base64;

/// A structure that can create minimally compact, non-colliding,
/// base-64 aliases for a set of strings. This is different then
/// a "hasher", because it optimizes for small hash sizes, but two
/// instances won't yield the same hashed values for a given input
/// string unless the order of hash operations is identical.
pub struct StringCompressor {
    expansion_map: HashMap<String, String>,
    compression_map: HashMap<String, String>,
    counter: u64,
}

impl StringCompressor {
    fn increment_key(&mut self) -> String {
        self.counter += 1;
        let bytes = (self.counter - 1).to_ne_bytes();
        base64::encode_config(&bytes, base64::URL_SAFE_NO_PAD)
    }

    /// Create a compressed alias for a given `String`.
    /// Invoking this function multiple times with the same
    /// input will yield the same result.
    pub fn compress(&mut self, expanded: String) -> String {
        match self.compression_map.get(&expanded).is_some() {
            true => self.compression_map.get(&expanded).unwrap().to_string(),
            false => {
                let compressed = self.increment_key();
                self.compression_map.insert(expanded, compressed.clone());
                compressed
            }
        }
    }

    /// Return the original value that yielded a given compressed `String`
    pub fn expand(&mut self, compressed: String) -> Option<String> {
        match self.expansion_map.get(&compressed) {
            Some(expanded) => Some(expanded.to_string()),
            None => None,
        }
    }
}