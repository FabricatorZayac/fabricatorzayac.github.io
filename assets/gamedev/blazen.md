# Blazen - Balatro demake for wasm4

[GitHub link](https://github.com/FabricatorZayac/blazen)
[Latest Demo](http://diaco.strangled.net:4444)

This thing is due for a rework, because of an increasing amount of spaghetti
code in the game logic.

## TODO:
- Separate out the engine into a separate crate
- Separate out game components from game logic
- Possibly rewrite in a different language, maybe C++ or Zig

C++ is a great language, but there's a potentially painful part that might
arise from it not having modern tooling. I will have to implement everything
from scratch myself

Zig is a fun language, but I fear writing in it because of the breaking
changes every update due to it not being 1.0. This has bitten me painfully
in the past.
