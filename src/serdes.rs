#![forbid(unsafe_code)]

// These constants help with sending and receiving utf-8 string slices serialized as [u16]
// across the COM bus for COM verbs that take string arguments.
// Serialized [u16] buffer format is:
// - u16_buf[0]: length of packed string in bytes
// - u16_buf[1..N]: little-endian packed utf-8 string
pub const STR_32_WORDS: usize = 17; // 1 length + 16 data (max utf-8 length 32 bytes)
pub const STR_64_WORDS: usize = 33; // 1 length + 32 data (max utf-8 length 64 bytes)
pub const STR_32_U8_SIZE: usize = 2 * (STR_32_WORDS - 1);
pub const STR_64_U8_SIZE: usize = 2 * (STR_64_WORDS - 1);

/// Error codes related to COM bus protocol serialization/deserialization
#[derive(Debug)]
pub enum SerdesError {
    StrLenTooBig = 1,
    Utf8Decode = 2,
}

/// Serialized (convertable to &[u16]) string of max-length 2*(U16_LEN-1) bytes.
/// This uses const generics which require rustc version 1.51 or greater.
/// See https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html
pub struct StringSer<const U16_LEN: usize> {
    u16_buf: [u16; U16_LEN],
}
impl<const U16_LEN: usize> StringSer<U16_LEN> {
    pub fn new() -> Self {
        Self {
            u16_buf: [0; U16_LEN],
        }
    }

    /// Serialize a string slice into a u16 slice for sending over the COM bus.
    pub fn encode(&mut self, s: &str) -> Result<&[u16; U16_LEN], SerdesError> {
        let str_len = s.len();
        let u8_max_len = 2 * (U16_LEN - 1);
        if str_len > u8_max_len {
            return Err(SerdesError::StrLenTooBig);
        }
        // Using iterators here instead of C-style indexing saves a lot of bounds checks
        // See https://docs.rust-embedded.org/book/c-tips/index.html#iterators-vs-array-access
        let mut dest_it = self.u16_buf.iter_mut();
        if let Some(length) = dest_it.next() {
            *length = str_len as u16;
        }
        // Using chunks_exact() and remainder() should avoid both panics and bounds checks
        // See https://doc.rust-lang.org/std/primitive.slice.html#method.chunks_exact
        let mut src_chunks = s.as_bytes().chunks_exact(2);
        let src_rem = src_chunks.remainder();
        for dest in dest_it {
            if let Some(src) = src_chunks.next() {
                *dest = u16::from_le_bytes([src[0], src[1]]);
            } else if !src_rem.is_empty() {
                *dest = u16::from_le_bytes([src_rem[0], 0]);
                break;
            } else {
                break;
            }
        }
        Ok(self.as_u16_slice())
    }

    /// Return a u16 slice of this structs's serialized string.
    pub fn as_u16_slice(&self) -> &[u16; U16_LEN] {
        &self.u16_buf
    }
}

/// Deserialized (convertable to &str) COM protocol string of max-length 32 bytes
/// This uses const generics (see comment for StringSer).
pub struct StringDes<const U16_LEN: usize, const U8_LEN: usize> {
    len: usize,
    u8_buf: [u8; U8_LEN],
}
impl<const U16_LEN: usize, const U8_LEN: usize> StringDes<U16_LEN, U8_LEN> {
    pub fn new() -> Self {
        Self {
            len: 0,
            u8_buf: [0; U8_LEN],
        }
    }

    /// Deserialize a string packed as [u16] into a length and utf-8 byte buffer.
    pub fn decode_u16(&mut self, u16_buf: &[u16; U16_LEN]) -> Result<&str, SerdesError> {
        let mut src_it = u16_buf.iter();
        if let Some(length) = src_it.next() {
            if (*length as usize) <= U8_LEN {
                self.len = *length as usize;
            } else {
                return Err(SerdesError::StrLenTooBig);
            }
        }
        let mut dest_it = self.u8_buf.iter_mut();
        for src in src_it {
            let b = src.to_le_bytes();
            if let Some(dest) = dest_it.next() {
                *dest = b[0];
            }
            if let Some(dest) = dest_it.next() {
                *dest = b[1];
            }
        }
        self.as_str()
    }

    /// Convert this struct's byte buffer and length into a string slice.
    pub fn as_str(&self) -> Result<&str, SerdesError> {
        let str_len = self.len;
        let u8_max_len = 2 * (U16_LEN - 1);
        if str_len > u8_max_len {
            return Err(SerdesError::StrLenTooBig);
        }
        match core::str::from_utf8(&self.u8_buf[..str_len]) {
            Ok(s) => Ok(s),
            _ => Err(SerdesError::Utf8Decode),
        }
    }
}

/// Serdes Unit Tests.
/// If you run this as a submodule of betrusted-ec, `cargo test` alone won't work right
/// because ../.cargo/config sets an RV32 build target. The solution is to add a --target
/// switch to cargo test, like:
///   `cargo test --target="x86_64-unknown-linux-gnu"`
/// To check targets, try:
///   `rustc --print target-list`
///
#[cfg(test)]
mod tests {
    use super::*;

    /// Pack two ASCII chars as a u16
    fn c2u16(c1: char, c2: char) -> u16 {
        u16::from_le_bytes([c1 as u8, c2 as u8])
    }

    #[test]
    fn serialize_short_str() {
        const U16_LEN: usize = 4;
        let src = "short";
        let encoded = &[5, c2u16('s', 'h'), c2u16('o', 'r'), 't' as u16];
        let mut ser = StringSer::<U16_LEN>::new();
        assert_eq!(encoded, ser.encode(&src).unwrap());
    }

    #[test]
    fn deserialize_short_str() {
        const U16_LEN: usize = 4;
        const U8_LEN: usize = 2 * (U16_LEN - 1);
        let src = "short";
        let encoded = &[5, c2u16('s', 'h'), c2u16('o', 'r'), 't' as u16];
        let mut des = StringDes::<U16_LEN, U8_LEN>::new();
        assert_eq!(src, des.decode_u16(encoded).unwrap());
    }

    #[test]
    fn round_trip_short_str() {
        const U16_LEN: usize = 4;
        const U8_LEN: usize = 2 * (U16_LEN - 1);
        let src = "short";
        let mut ser = StringSer::<U16_LEN>::new();
        let mut des = StringDes::<U16_LEN, U8_LEN>::new();
        assert_eq!(src, des.decode_u16(ser.encode(&src).unwrap()).unwrap());
    }
}
