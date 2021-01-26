use core::ptr::null_mut;

#[repr(C)]
pub struct Buffer {
    pub data: *mut u8,
    pub len: usize,
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer { data: null_mut(), len: 0 }
    }
}

impl Clone for Buffer {
    fn clone(&self) -> Self {
        unsafe { Self::from_vec(self.to_vec()) }
    }
}

impl Buffer {
    pub unsafe fn from_vec(mut vec: Vec<u8>) -> Self {
        if vec.is_empty() {
            return Self::default();
        }

        let len = vec.len();
        let data = vec.as_mut_ptr();

        core::mem::forget(vec);

        Buffer { data, len }
    }

    pub unsafe fn from_string(mut str: String) -> Self {
        if str.is_empty() {
            return Self::default();
        }

        let len = str.len();
        let data = str.as_mut_ptr();

        core::mem::forget(str);

        Buffer { data, len }
    }

    pub unsafe fn to_vec(&self) -> Vec<u8> {
        let mut target: Vec<u8> = Vec::new();
        let buffer = std::slice::from_raw_parts(self.data, self.len);

        target.clone_from_slice(buffer);
        target
    }

    pub unsafe fn into_vec(self) -> Vec<u8> {
        let buffer = std::slice::from_raw_parts_mut(self.data, self.len);
        let owned = Box::from_raw(buffer);

        owned.into_vec()
    }
}

#[no_mangle]
pub unsafe extern "C" fn redirectionio_api_buffer_drop(buffer: Buffer) {
    buffer.into_vec();
}
