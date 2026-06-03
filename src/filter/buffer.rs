use core::ptr::null_mut;

#[repr(C)]
pub struct Buffer {
    data: *mut u8,
    len: usize,
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer { data: null_mut(), len: 0 }
    }
}

impl Clone for Buffer {
    fn clone(&self) -> Self {
        Self::from_vec(self.to_vec())
    }
}

impl Buffer {
    /// # Safety
    ///
    /// Duplicate this buffer, existing one will still existing in memory
    pub fn duplicate(&self) -> Self {
        Self::from_vec(self.to_vec())
    }

    pub fn from_vec(vec: Vec<u8>) -> Self {
        if vec.is_empty() {
            return Self::default();
        }

        // Convert to a boxed slice so the allocation size matches `len` exactly.
        // `into_vec` later deallocates assuming capacity == len, so we must not
        // keep any spare capacity here (doing so would dealloc with the wrong
        // layout, which is undefined behavior).
        let boxed = vec.into_boxed_slice();
        let len = boxed.len();
        let data = Box::into_raw(boxed) as *mut u8;

        Buffer { data, len }
    }

    pub fn from_string(str: String) -> Self {
        Self::from_vec(str.into_bytes())
    }

    pub fn to_vec(&self) -> Vec<u8> {
        if self.data.is_null() || self.len == 0 {
            return Vec::new();
        }

        // Safety: data is a valid pointer to a buffer of length len aligned
        // This is guaranteed by exposed API to construct this struct
        let buffer = unsafe { std::slice::from_raw_parts(self.data, self.len) };

        buffer.to_vec()
    }

    pub fn into_vec(self) -> Vec<u8> {
        if self.data.is_null() || self.len == 0 {
            return Vec::new();
        }

        // Safety: data is a valid pointer to a buffer of length len aligned
        // This is guaranteed by exposed API to construct this struct
        let owned = unsafe {
            let buffer = std::slice::from_raw_parts_mut(self.data, self.len);
            Box::from_raw(buffer)
        };

        owned.into_vec()
    }
}

/// Create a buffer owning a copy of `len` bytes read from `data`.
///
/// The returned buffer is allocated with Rust's allocator, so it must be
/// reclaimed by Rust (either by passing it to a function that consumes it, such
/// as `redirectionio_action_body_filter_filter`, or with
/// `redirectionio_api_buffer_drop`). The memory pointed to by `data` is only
/// read; ownership is not taken, so the caller remains responsible for freeing
/// it.
///
/// # Safety
///
/// `data` must point to at least `len` readable bytes, or be null when `len` is 0.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn redirectionio_api_buffer_create(data: *const u8, len: usize) -> Buffer {
    if data.is_null() || len == 0 {
        return Buffer::default();
    }

    // Safety: data points to len readable bytes as guaranteed by the caller
    let slice = unsafe { std::slice::from_raw_parts(data, len) };

    Buffer::from_vec(slice.to_vec())
}

#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_api_buffer_drop(buffer: Buffer) {
    buffer.into_vec();
}

#[cfg(test)]
mod tests {
    use super::Buffer;

    #[test]
    fn to_vec_returns_buffer_contents() {
        let buffer = Buffer::from_vec(vec![1, 2, 3, 4]);
        assert_eq!(buffer.to_vec(), vec![1, 2, 3, 4]);
        assert_eq!(buffer.into_vec(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn duplicate_preserves_contents_and_leaves_original_usable() {
        let buffer = Buffer::from_vec(vec![5, 6, 7]);
        let copy = buffer.duplicate();
        assert_eq!(copy.to_vec(), vec![5, 6, 7]);
        assert_eq!(buffer.to_vec(), vec![5, 6, 7]);
        buffer.into_vec();
        copy.into_vec();
    }

    #[test]
    fn round_trip_with_spare_capacity_matches_allocation_size() {
        // A Vec with capacity > len must still round-trip without a
        // mismatched-layout deallocation.
        let mut vec = Vec::with_capacity(64);
        vec.extend_from_slice(b"hello");
        let buffer = Buffer::from_vec(vec);
        assert_eq!(buffer.into_vec(), b"hello".to_vec());
    }

    #[test]
    fn from_string_round_trips() {
        let buffer = Buffer::from_string("héllo".to_string());
        assert_eq!(buffer.into_vec(), "héllo".as_bytes().to_vec());
    }
}
