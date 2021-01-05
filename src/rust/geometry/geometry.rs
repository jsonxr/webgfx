use wasm_bindgen::prelude::*;


#[wasm_bindgen]
#[derive(Debug)]
pub struct Geometry {
  pub id: u32,

}

pub struct Geo<'life> {
  pub id: u32,
  pub vertices: &'life [f32],
}


// extern "C" {
//   #[wasm_bindgen(js_namespace = console)]
//   fn log(msg: &str);
// }

// #[macro_export]
// macro_rules! log {
//   ($($t:tt)*) => (log(&format!($($t)*)));
// } 

#[wasm_bindgen]
impl Geometry {
  #[wasm_bindgen(constructor)]
  pub fn wasm_new(_vertices: Box<[JsValue]>) -> Geometry {
    //log!("The {} is {}", "meaning of life", 42);
    Geometry {
      id: 1,
    }
  }
}
