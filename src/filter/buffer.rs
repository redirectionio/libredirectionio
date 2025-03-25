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

    pub fn from_vec(mut vec: Vec<u8>) -> Self {
        if vec.is_empty() {
            return Self::default();
        }

        let len = vec.len();
        let data = vec.as_mut_ptr();

        core::mem::forget(vec);

        Buffer { data, len }
    }

    pub fn from_string(mut str: String) -> Self {
        if str.is_empty() {
            return Self::default();
        }

        let len = str.len();
        let data = str.as_mut_ptr();

        core::mem::forget(str);

        Buffer { data, len }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        if self.data.is_null() || self.len == 0 {
            return Vec::new();
        }

        let mut target: Vec<u8> = Vec::new();
        // Safety: data is a valid pointer to a buffer of length len aligned
        // This is guaranteed by exposed API to construct this struct
        let buffer = unsafe { std::slice::from_raw_parts(self.data, self.len) };

        target.clone_from_slice(buffer);
        target
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

#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_api_buffer_drop(buffer: Buffer) {
    buffer.into_vec();
}
