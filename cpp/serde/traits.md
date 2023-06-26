# Rust-like traits in C++

Basically, the way you do it is you make an un-instantiatable template

```c++
template<typename T>
struct Serialize;
```

and an equivalent to

```rust
impl Serialize for i32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Err>
    where S: Serializer { ... }
}
```

would be a specialization like

```c++
template<>
struct Serialize<int> {
    template<Serializer S>
    static ftl::Result<typename S::Ok, typename S::Err>
    serialize(RGB &self, S serializer) { ... }
}
```

Unfortunately, we can't have the nice call syntax like in rust.
Instead of
```rust
let foo: i32 = 69;
foo.serialize(some_serializer);
```
you will have to do
```c++
int foo = 69;
Serialize<int>::serialize(foo, some_serializer);
```
