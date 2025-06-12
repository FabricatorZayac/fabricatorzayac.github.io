# Writing a garbage collector in C++

[GitHub link](https://github.com/FabricatorZayac/gcpp)

Inspired by [Cheating the Reaper in Go](https://mcyoung.xyz/2025/04/21/go-arenas/),
I have decided to try to do the same thing, but in reverse and in C++.
At first I wanted to make the garbage collector satisfy the `Allocator` concept.
To do garbage collection, I need to walk the object graph starting from the roots.
How do I know which allocations are roots? A language with native GC would
have a mechanism of walking the globals and stack, and upon finding a pointer,
following it into the heap.
In C++ I need to instead manually store a list of living roots.
```c++
struct GC {
    // ...
private:
    std::vector<Root> roots; // reference counted root pointers
    std::vector<DynAlloc *> allocations; // all allocations
public:
    template<typename T>
    auto bind(T *ptr) -> RootPtr<T> {
        roots.push_back({
            .refcount = 1,
            .data = ptr,
        });

        return RootPtr<T>(roots.size() - 1, *this);
    }

    template<typename T>
    auto create() -> RootPtr<T> {
        return bind(allocate<T>());
    }
};
```
`RootPtr<T>` holds a `GC &` and an index of it's corresponding root, with the
root holding a refcount and a pointer to an allocated object
