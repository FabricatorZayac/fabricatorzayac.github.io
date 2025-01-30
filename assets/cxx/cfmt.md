# cfmt formatting library

[GitHub link](https://github.com/FabricatorZayac/cfmt)

cfmt is an stb-style single header library that aims to replace printf
with a rust-style Display trait

## Example

fmt_config.h:
```c
#define FMT_MIXIN \
fmt_mixin(RGB)
```

main.h:
```c
#define FMT_INCLUDE_CONFIG_H
#include "fmt.h"

typedef struct {
    uint8_t r;
    uint8_t g;
    uint8_t b;
} RGB;

// Display trait impl
DISPLAY(RGB)(const RGB *self, FILE *stream) {
    return fmt.format(
        stream,
        "RGB {{ r: {}, g: {}, b: {} }}",
        (int)self->r, (int)self->g, (int)self->b
    );
}

int main() {
    // Calls the display trait for each argument under the hood
    fmt.println("{}", ((RGB){ .r = 5, .g = 255, .b = 8 }));
}

// Insert implementation in the end,
// because it needs to know the types it's working with
#define CFMT_IMPLEMENTATION
#include "fmt.h"
```
Output: `RGB { r: 5, g: 255, b: 8 }`

---

Error handling:
```c
fmt_error err;
if ((err = fmt.print("{}\n"))) {
    fmt.report(err);
}
if ((err = fmt.print("}\n"))) {
    fmt.report(err);
}
if ((err = fmt.print("{\n"))) {
    fmt.report(err);
}
```
Output:
```c
test/src/main.c:40: error: too few arguments
test/src/main.c:44: error: missing opening {
test/src/main.c:48: error: missing closing }
```

## How it works

### Dispatch

In `fmt_mixin` you can specify a custom format functionn instead of defaulting
to the `DISPLAY(T)` implementation like: `fmt_mixin(str_t, str.fmt)`

```c
#define ___FMT_MIXIN_OVERLOAD(_1, _2, NAME, ...) NAME
#define fmt_mixin(...) \
___FMT_MIXIN_OVERLOAD(__VA_ARGS__, ___fmt_mixin2, ___fmt_mixin1)(__VA_ARGS__)
```

The `fmt_mixin` macro has an overload depending on the number of
arguements to select between custom function and default behavior

The custom function needs to have a signature like `fmt_t(const T *)`.

`fmt_t` is an interface consisting of the pointer to the value
and the format function

```c
typedef fmt_error (*fmt_fn)(const void *ctx, FILE *stream);
typedef struct {
    const void *ptr;
    fmt_fn fmt;
} fmt_t;
```


All supported types are listed in the `fmt_type_t` enum, which contains
the default builtin types and gets built upon with `FMT_MIXIN`

```c
typedef enum {
    FMT_TYPE_END,
    FMT_TYPE_DYNAMIC,
    FMT_TYPE_CSTR,
    FMT_TYPE_ERR,
    FMT_TYPE_BOOL,
#undef ___fmt_mixin1
#undef ___fmt_mixin2
#define ___fmt_mixin1(T) FMT_TYPE_##T,
#define ___fmt_mixin2(T, _) FMT_TYPE_##T,
    BUILTIN_TYPES
    FMT_MIXIN
    FMT_TYPES,
} fmt_type_t;
#undef ___fmt_mixin1
#undef ___fmt_mixin2
#define ___fmt_mixin1(T) T: FMT_TYPE_##T,
#define ___fmt_mixin2(T, _) T: FMT_TYPE_##T,
```

The `format` macros insert a type marker before each vararg, so the internal
`vformat` function knows what type it's working on and calls the appropriate
Display implementation for the type

```c
#define _FMT_MARKER(ARG) _Generic((ARG), \
    FMT_MIXIN                            \
    BUILTIN_TYPES                        \
    bool: FMT_TYPE_BOOL,                 \
    char *: FMT_TYPE_CSTR,               \
    fmt_t: FMT_TYPE_DYNAMIC)
```

Inside of vformat, there's a switch statement that looks up the type marker
(known type) so it knows what type to expect in `va_arg`

```c
#undef ___fmt_mixin1
#undef ___fmt_mixin2
#define ___fmt_mixin1(T) \
            case FMT_TYPE_##T: { \
                T value = va_arg(argv, T); \
                display = T##_fmt(&value); \
            } break;
#define ___fmt_mixin2(T, FN) \
            case FMT_TYPE_##T: { \
                T value = va_arg(argv, T); \
                display = FN(&value); \
            } break;

            BUILTIN_TYPES
            FMT_MIXIN
#undef ___fmt_mixin1
#undef ___fmt_mixin2
// set it back
#define ___fmt_mixin1(T) T: FMT_TYPE_##T,
#define ___fmt_mixin2(T, FN) T: FMT_TYPE_##T,
```

### Error handling
```c
// Unfortunately, I can't check the format string for correctness at
// compile time, so we have to rely on runtime errors/crashes
fmt_error (*_format)(FILE *stream, const char *restrict fmt, ...);
void (*_format_or_die)(
    FILE *stream,
    const char *restrict fmt,
    const char *file,
    int line,
    ...
);
```
The `println` macro calls `printu` macro under the hood, which calls
`format_or_die` with `stdout`. `format_or_die` macro calls `_format_or_die`
internal function, passing in the filename and line number, for pretty
error messages on crash.

The `print` macro calls `format` instead, which returns the error instead of
panicking

`format` macros also add a 0 at the end, to check if there are
enough arguments provided

```c
// in internal vformat
// FMT_TYPE_END is 0
arg_typeid = va_arg(argv, fmt_type_t);
if (arg_typeid == FMT_TYPE_END) {
    return FMT_ERR_NOT_ENOUGH_ARGS;
}
```

## TODO
- More default implementations for builtin types
