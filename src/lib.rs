pub struct RawMemoryBox {
    inner: *const usize,
    length: usize,
    capacity: usize,
}

impl RawMemoryBox {
    pub fn new<T>(inner: &T) -> Self {
        let length = std::mem::size_of_val(inner);
        Self {
            inner: inner as *const _ as *const usize,
            length,
            capacity: length,
        }
    }

    pub fn with_capacity<T>(inner: &T, capacity: usize) -> Self {
        Self {
            inner: inner as *const _ as *const usize,
            length: std::mem::size_of_val(inner),
            capacity,
        }
    }

    pub unsafe fn into_inner_with_length(&self) -> Self {
        let usizes = std::slice::from_raw_parts(self.inner, 2);
        let inner_size = usizes
            .get(1)
            .expect("pointer type given should contain a length");

        Self {
            inner: *self.inner as *const usize,
            length: *inner_size,
            capacity: *inner_size,
        }
    }

    pub unsafe fn into_inner_with_length_and_capacity(&self) -> Self {
        let usizes = std::slice::from_raw_parts(self.inner, 3);
        let inner_capacity = usizes
            .get(1)
            .expect("pointer type given should contain a capacity");
        let inner_size = usizes
            .get(2)
            .expect("pointer type given should contain a length");

        Self {
            inner: *self.inner as *const usize,
            length: *inner_size,
            capacity: *inner_capacity,
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        unsafe { std::slice::from_raw_parts(self.inner as *const u8, self.capacity) }.into()
    }

    pub fn used_bytes(&self) -> Vec<u8> {
        unsafe { std::slice::from_raw_parts(self.inner as *const u8, self.length) }.into()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::RawMemoryBox;

    #[test]
    fn new_unsafe_boxed_8bit_number() {
        let value = 5i8;
        let bytes = RawMemoryBox::new(&value).bytes();
        assert_eq!(bytes, [5u8]);
    }

    #[test]
    fn new_unsafe_boxed_8bit_negative_number() {
        let value = -5i8;
        let bytes = RawMemoryBox::new(&value).bytes();
        assert_eq!(bytes, [251u8]);
    }

    #[test]
    fn new_unsafe_boxed_8bit_number_slice() {
        let value = [1i8, 2, 3];
        let bytes = RawMemoryBox::new(&value).bytes();
        assert_eq!(bytes, [1u8, 2, 3]);
    }

    #[test]
    fn new_unsafe_boxed_16bit_number_slice() {
        let value = [1i16, 2, 3];
        let bytes = RawMemoryBox::new(&value).bytes();
        assert_eq!(bytes, [1u8, 0, 2, 0, 3, 0]);
    }

    #[test]
    fn new_unsafe_boxed_8bit_number_vec() {
        let value = vec![1i8, 2, 3];
        let bytes = unsafe { RawMemoryBox::new(&value).into_inner_with_length() }.bytes();
        assert_eq!(bytes, [1u8, 2, 3]);
    }

    #[test]
    fn new_unsafe_boxed_8bit_number_vec_with_capacity() {
        let value = {
            let mut vec = Vec::with_capacity(5);
            vec.push(1i8);
            vec.push(2);
            vec.push(3);
            vec
        };
        let bytes =
            unsafe { RawMemoryBox::new(&value).into_inner_with_length_and_capacity() }.bytes();
        assert_eq!(bytes[0], 1);
        assert_eq!(bytes[1], 2);
        assert_eq!(bytes[2], 3);
        assert_eq!(bytes.len(), 5);
    }

    #[test]
    fn new_unsafe_boxed_string_slice() {
        let value = "abc";
        let bytes = unsafe { RawMemoryBox::new(&value).into_inner_with_length() }.bytes();
        assert_eq!(bytes, ['a' as u8, 'b' as u8, 'c' as u8]);
    }

    #[test]
    fn new_unsafe_boxed_string_with_capacity() {
        let value = {
            let mut s = String::with_capacity(5);
            s.push('a');
            s.push('b');
            s.push('c');
            s
        };
        let bytes =
            unsafe { RawMemoryBox::new(&value).into_inner_with_length_and_capacity() }.bytes();
        assert_eq!(bytes[0], 'a' as u8);
        assert_eq!(bytes[1], 'b' as u8);
        assert_eq!(bytes[2], 'c' as u8);
        assert_eq!(bytes.len(), 5);
    }
}
