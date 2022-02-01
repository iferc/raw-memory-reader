use raw_memory_ref::RawMemoryRef;

fn main() {
    let value = vec![5i32];
    let boxed =
        unsafe { RawMemoryRef::new(&value).into_inner_with_length(std::mem::size_of::<i32>()) };

    // should be fine here
    assert_eq!(boxed.allocated_bytes(), vec![5u8, 0, 0, 0]);

    // original value dropped
    drop(value);

    // cannot compil due to read after drop
    assert_eq!(boxed.allocated_bytes(), vec![5u8, 0, 0, 0]);
}
