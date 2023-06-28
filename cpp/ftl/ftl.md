# FTL, my very own template library
[GitHub link](https://github.com/Zayac-The-Engineer/ftl)

### Features
- `Result<T, E>`, Rust-like Result type
- `Option<T>`, Rust-like Option type
- `str`, a string slice

### TODO
- Add the ability to store references like `Option<T&>` and `Result<T&, E>`.

    Problem here is that you can't have reference members in a union.
    A solution could be to have a partial specialization for `T&` that would
    store `std::reference_wrapper<T>`
