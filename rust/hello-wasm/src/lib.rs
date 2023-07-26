use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }


#[wasm_bindgen]
pub fn f(x: &str) -> usize {
    x.len()
}


// #[wasm_bindgen]
// pub fn g() -> usize {
//     "xxx".len()
// }
