# Rust-like traits in C++

Basically, the way you do it is you make an uninstantiatable template

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
    serialize(RGB &self, S &serializer) { ... }
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

Unfortunately, to have a constraint like `where T: Serialize`
you will need to write a separate concept.
And concepts can't share names with structs,
so we will have to name it differently.

```c++
template<typename T>
concept Serializable = requires(const T &self,
                                archetypes::Serializer &serializer) {
    { Serialize<T>::serialize(self, serializer) }
    -> std::same_as<ftl::Result<typename archetypes::Serializer::Ok,
                                typename archetypes::Serializer::Err>>;
}
```

You may be asking what the hell is an archetype?
It's a fake struct an instance of which would satisfy a concept.

```c++
namespace archetypes {
    struct Serializer {
        using Ok = void;
        using Error = Error;
        using SerializeStruct = SerializeStruct;

        ftl::Result<Ok, Error> serialize_bool(const bool &);

        ftl::Result<Ok, Error> serialize_char(const char &);

        ftl::Result<Ok, Error> serialize_short(const short &);
        ftl::Result<Ok, Error> serialize_int(const int &);
        ftl::Result<Ok, Error> serialize_long(const long &);
        ftl::Result<Ok, Error> serialize_long_long(const long long &);

        ftl::Result<Ok, Error> serialize_float(const float&);
        ftl::Result<Ok, Error> serialize_double(const double&);

        ftl::Result<Ok, Error> serialize_str(const ftl::str &);
        ftl::Result<SerializeStruct *, Error>
        serialize_struct(const ftl::str &, const ftl::usize);
    };
}

// should succeed
static_assert(Serializer<archetypes::Serializer>);
```

The deserializer constraints are currently not implemented,
due to the deserializer being much, much more complicated than the serializer.

