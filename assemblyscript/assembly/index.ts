// The entry file of your WebAssembly module.

export function add(a: i32, b: i32): i32 {
  return a + b;
}

export function blah(): void {
  const obj: ref_struct = { a: 1, b: 2 };
  const c = obj.a + obj.b;
}
