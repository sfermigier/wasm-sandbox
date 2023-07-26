# Wasm sandbox

## Notes

### Zig

Compile examples with something like:

```bash
zig build-lib zig-src/math2.zig -O ReleaseSmall -target wasm32-freestanding -dynamic
```

### C

Compile examples with:

```bash

zig cc -shared -target wasm32-freestanding -o add.wasm add.c

# or

emcc -O3 main.c
```
