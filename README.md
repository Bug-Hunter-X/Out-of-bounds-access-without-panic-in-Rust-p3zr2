# Out-of-bounds access without panic in Rust

This repository demonstrates a potential security issue in Rust: out-of-bounds array access without a runtime panic. While Rust's ownership and borrowing system prevents many common memory errors, it's important to understand that using `get()` method on a vector does not cause a panic, this can lead to unexpected behavior, especially when not handled appropriately.

The `bug.rs` file shows code which attempts to access an index out of bounds for the vector. This may not panic but return a None. 

The solution handles the potential error case which can make the code more robust and prevents such errors. 
