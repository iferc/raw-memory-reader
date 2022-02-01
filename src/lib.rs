/// RawMemoryRef holds onto a raw pointer for the purpose of reading its raw bytes of memory.
pub struct RawMemoryRef<'a> {
    inner: *const usize,
    length: usize,
    capacity: usize,
    phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> RawMemoryRef<'a> {
    /// Create a new raw memory reference for some given data.
    pub fn new<T>(inner: &'a T) -> Self {
        let length = std::mem::size_of_val(inner);
        Self {
            inner: inner as *const _ as *const usize,
            length,
            capacity: length,
            phantom: std::marker::PhantomData,
        }
    }

    /// Create a new raw memory reference for some given data with some
    /// explicit capacity size.
    ///
    /// This method is relies on the caller knowing the correctly allocated
    /// capacity of the referenced data. If incorrect, the process may self
    /// terminate with the error `(signal: 11, SIGSEGV: invalid memory reference)`.
    ///
    /// Most likely, you want to use [`new`][RawMemoryRef::new] which can detect
    /// the allocated size of the data referenced.
    pub unsafe fn with_capacity<T>(inner: &'a T, capacity: usize) -> Self {
        Self {
            inner: inner as *const _ as *const usize,
            length: std::mem::size_of_val(inner),
            capacity,
            phantom: std::marker::PhantomData,
        }
    }

    pub unsafe fn into_inner(&self, inner_size_bytes: usize) -> Self {
        Self {
            inner: *self.inner as *const usize,
            length: inner_size_bytes,
            capacity: inner_size_bytes,
            phantom: std::marker::PhantomData,
        }
    }

    pub unsafe fn into_inner_with_length(&self, inner_size_bytes: usize) -> Self {
        let usizes = std::slice::from_raw_parts(self.inner, 2);
        let inner_size = usizes
            .get(1)
            .expect("pointer type given should contain a length");

        Self {
            inner: *self.inner as *const usize,
            length: *inner_size * inner_size_bytes,
            capacity: *inner_size * inner_size_bytes,
            phantom: std::marker::PhantomData,
        }
    }

    /// Returns a new RawMemoryRef of contained pointer, capacity, and length values.
    ///
    /// This is useful for stepping into types such as [`String`] or [`Vec`]. Any type that
    /// explicitly contains these three values as the first words of data in that order.
    /// ```
    /// let mut greeting = String::with_capacity(25);
    /// greeting.push_str("Hello");
    /// greeting.push(',');
    /// greeting.push(' ');
    /// greeting.push_str("world!");
    ///
    /// let dataref = unsafe {
    ///     raw_memory_ref::RawMemoryRef::new(&greeting)
    ///         .into_inner_with_length_and_capacity(std::mem::size_of::<u8>())
    /// };
    /// let bytes = dataref.initialized_bytes();
    ///
    /// assert_eq!(std::str::from_utf8(&bytes), Ok("Hello, world!"));
    /// assert_eq!(bytes.len(), 13);
    /// ```
    ///
    /// ```
    /// let mut booleans = Vec::with_capacity(5);
    /// booleans.push(true);
    /// booleans.push(false);
    /// booleans.push(false);
    ///
    /// let dataref = unsafe {
    ///     raw_memory_ref::RawMemoryRef::new(&booleans)
    ///         .into_inner_with_length_and_capacity(std::mem::size_of::<bool>())
    /// };
    /// let bytes = dataref.initialized_bytes();
    ///
    /// assert_eq!(bytes[..3], [true as u8, false as u8, false as u8]);
    /// assert_eq!(bytes.len(), 3);
    /// ```
    ///
    /// Worth noting that capacity is ususally in the same slot as length for types where
    /// there is no stored capacity. This means that
    /// [`into_inner_with_length`][RawMemoryRef::into_inner_with_length]
    /// should work in place of
    /// [`into_inner_with_length_and_capacity`][RawMemoryRef::into_inner_with_length_and_capacity]
    /// if there isn't a need for skipping unintialized bytes.
    pub unsafe fn into_inner_with_length_and_capacity(&self, inner_size_bytes: usize) -> Self {
        let usizes = std::slice::from_raw_parts(self.inner, 3);
        let inner_capacity = usizes
            .get(1)
            .expect("pointer type given should contain a capacity");
        let inner_size = usizes
            .get(2)
            .expect("pointer type given should contain a length");

        Self {
            inner: *self.inner as *const usize,
            length: *inner_size * inner_size_bytes,
            capacity: *inner_capacity * inner_size_bytes,
            phantom: std::marker::PhantomData,
        }
    }

    /// Returns a slice of bytes of the referenced data. The bytes may
    /// or may not be initialized bytes.
    ///
    /// Refer to [`initialized_bytes`][RawMemoryRef::initialized_bytes] if you want to skip uninitialized tail bytes.
    ///
    /// For example, a [`Vec`] might have a larger capacity than actual size
    /// which would contain uninitialized bytes.
    /// ```
    /// let mut numbers = Vec::with_capacity(5);
    /// numbers.push(1u8);
    /// numbers.push(2);
    /// numbers.push(3);
    ///
    /// let dataref = unsafe {
    ///     raw_memory_ref::RawMemoryRef::new(&numbers)
    ///         .into_inner_with_length_and_capacity(std::mem::size_of::<u8>())
    /// };
    /// let bytes = dataref.allocated_bytes();
    ///
    /// assert_eq!(bytes[..], [1u8, 2, 3, 0, 0]);
    /// assert_eq!(bytes.len(), 5);
    /// ```
    pub fn allocated_bytes(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts(self.inner as *const u8, self.capacity) }
    }

    /// Returns a slice of bytes of the referenced data. The bytes should
    /// most likely be initialized bytes only.
    ///
    /// For example, a [`Vec`] might have a larger capacity than actual size
    /// which would contain uninitialized bytes.
    /// ```
    /// let mut numbers = Vec::with_capacity(5);
    /// numbers.push(1i16);
    /// numbers.push(2);
    /// numbers.push(3);
    /// numbers.push(i16::MAX);
    ///
    /// let dataref = unsafe {
    ///     raw_memory_ref::RawMemoryRef::new(&numbers)
    ///         .into_inner_with_length_and_capacity(std::mem::size_of::<i16>())
    /// };
    /// let bytes = dataref.initialized_bytes();
    ///
    /// assert_eq!(bytes, &[1u8, 0, 2, 0, 3, 0, 255, 127]);
    /// assert_eq!(bytes.len(), 8);
    /// ```
    pub fn initialized_bytes(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts(self.inner as *const u8, self.length) }
    }

    /// Number of bytes allocated that should be initialized of the referenced data.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Number of bytes allocated that may or may not be initialized of the referenced data.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::RawMemoryRef;

    #[test]
    fn lifetime_of_ref_should_match_source_data() {
        trybuild::TestCases::new().compile_fail("tests/no_compile/valid_lifetime.rs");
    }

    #[test]
    fn new_unsafe_boxed_8bit_number() {
        let value = 5i8;
        let bytes = RawMemoryRef::new(&value).allocated_bytes();
        assert_eq!(bytes, [5u8]);
    }

    #[test]
    fn new_unsafe_boxed_8bit_negative_number() {
        let value = -5i8;
        let bytes = RawMemoryRef::new(&value).allocated_bytes();
        assert_eq!(bytes, [251u8]);
    }

    #[test]
    fn new_unsafe_boxed_8bit_number_slice() {
        let value = [1i8, 2, 3];
        let bytes = RawMemoryRef::new(&value).allocated_bytes();
        assert_eq!(bytes, [1u8, 2, 3]);
    }

    #[test]
    fn new_unsafe_boxed_16bit_number_slice() {
        let value = [1i16, 2, 3, i16::MAX];
        let bytes = RawMemoryRef::new(&value).allocated_bytes();
        assert_eq!(bytes, [1u8, 0, 2, 0, 3, 0, 255, 127]);
    }

    #[test]
    fn new_unsafe_boxed_8bit_number_vec() {
        let value = vec![1i8, 2, 3];
        let bytes =
            unsafe { RawMemoryRef::new(&value).into_inner_with_length(std::mem::size_of::<i8>()) }
                .allocated_bytes();
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
        let bytes = unsafe {
            RawMemoryRef::new(&value).into_inner_with_length_and_capacity(std::mem::size_of::<i8>())
        }
        .allocated_bytes();
        assert_eq!(bytes[0], 1);
        assert_eq!(bytes[1], 2);
        assert_eq!(bytes[2], 3);
        assert_eq!(bytes.len(), 5);
    }

    #[test]
    fn new_unsafe_boxed_string_slice() {
        let value = "abc";
        let bytes =
            unsafe { RawMemoryRef::new(&value).into_inner_with_length(std::mem::size_of::<u8>()) }
                .allocated_bytes();
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
        let bytes = unsafe {
            RawMemoryRef::new(&value).into_inner_with_length_and_capacity(std::mem::size_of::<u8>())
        }
        .allocated_bytes();
        assert_eq!(bytes[0], 'a' as u8);
        assert_eq!(bytes[1], 'b' as u8);
        assert_eq!(bytes[2], 'c' as u8);
        assert_eq!(bytes.len(), 5);
    }
}
