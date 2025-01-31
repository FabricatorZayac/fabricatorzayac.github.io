# Zig wasm4 demo

[GitHub link](https://github.com/FabricatorZayac/zig-wasm4)

### What is [wasm4](https://wasm4.org)
Wasm4 is a fantasy console for making retro-style games with a 4 color screen,
160x160px framebuffer, memory mapped I/O and a 64kb limit on memory and cart size.

If you have the bindings, you could theoretically use any compiled language to
write wasm4 games. In my experience, Zig is the most pleasant language for
doing so.

I've tried Rust before, but while I do enjoy the language, unsafe Rust
really sucks to work with. Also it's not particularly good for places where every
byte matters.

### What did I do with it

In it's current state it's a single line which you can move around and
modify it's angular velocity

<iframe src="https://fabricatorzayac.github.io/zig-wasm4" style="height: 35rem; width: 100%"></iframe>
