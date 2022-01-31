# Raw Memory Reader

Utility crate for reading raw bytes of memory from a given value. Can unsafely step into an inner reference when the inner reference is the first usize in the initially given reference.

Primarily intended for teaching purposes at this time to show the internal values of various types such as slices, `str`, `String`, and `Vec`.

See the [tests](./src/lib.rs) for example usage.
