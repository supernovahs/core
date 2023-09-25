use crate::{bits::FixedBytes, B256};

/// Hash a message according to [EIP-191] (version `0x01`).
///
/// The final message is a UTF-8 string, encoded as follows:
/// `"\x19Ethereum Signed Message:\n" + message.length + message`
///
/// This message is then hashed using [Keccak-256](keccak256).
///
/// [EIP-191]: https://eips.ethereum.org/EIPS/eip-191
pub fn hash_message<T: AsRef<[u8]>>(message: T) -> B256 {
    const PREFIX: &str = "\x19Ethereum Signed Message:\n";

    let message = message.as_ref();
    let len = message.len();
    let len_string = len.to_string();

    let mut eth_message = Vec::with_capacity(PREFIX.len() + len_string.len() + len);
    eth_message.extend_from_slice(PREFIX.as_bytes());
    eth_message.extend_from_slice(len_string.as_bytes());
    eth_message.extend_from_slice(message);

    keccak256(&eth_message)
}

/// Simple interface to the [`keccak256`] hash function.
///
/// [`keccak256`]: https://en.wikipedia.org/wiki/SHA-3
pub fn keccak256<T: AsRef<[u8]>>(bytes: T) -> FixedBytes<32> {
    cfg_if::cfg_if! {
        if #[cfg(all(feature = "native-keccak", not(feature = "tiny-keccak")))] {
            #[link(wasm_import_module = "vm_hooks")]
            extern "C" {
                /// When targeting VMs with native keccak hooks, the `native-keccak` feature
                /// can be enabled to import and use the host environment's implementation
                /// of [`keccak256`] in place of [`tiny_keccak`]. This is overridden when
                /// the `tiny-keccak` feature is enabled.
                ///
                /// # Safety
                ///
                /// The VM accepts the preimage by pointer and length, and writes the
                /// 32-byte hash.
                /// - `bytes` must point to an input buffer at least `len` long.
                /// - `output` must point to a buffer that is at least 32-bytes long.
                ///
                /// [`keccak256`]: https://en.wikipedia.org/wiki/SHA-3
                /// [`tiny_keccak`]: https://docs.rs/tiny-keccak/latest/tiny_keccak/
                fn native_keccak256(bytes: *const u8, len: usize, output: *mut u8);
            }

            /// Calls an external native keccak hook when `native-keccak` is enabled.
            /// This is overridden when `tiny-keccak` is enabled.
            fn keccak256(bytes: &[u8]) -> FixedBytes<32> {
                let mut output = [0; 32];
                // SAFETY: The output is 32-bytes, and the input comes from a slice.
                unsafe { native_keccak256(bytes.as_ptr(), bytes.len(), output.as_mut_ptr()) };
                output.into()
            }
        } else {
            use tiny_keccak::{Hasher, Keccak};

            /// Calls [`tiny-keccak`] when the `tiny-keccak` feature is enabled or
            /// when no particular keccak feature flag is specified.
            ///
            /// [`tiny_keccak`]: https://docs.rs/tiny-keccak/latest/tiny_keccak/
            fn keccak256(bytes: &[u8]) -> FixedBytes<32> {
                let mut hasher = Keccak::v256();
                hasher.update(bytes);
                let mut output = [0; 32];
                hasher.finalize(&mut output);
                output.into()
            }
        }
    }

    keccak256(bytes.as_ref())
}
