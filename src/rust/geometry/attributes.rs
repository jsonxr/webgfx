use std::mem;

pub trait Attribute {
  fn get_size(&self) -> usize;
  fn get_len(&self) -> usize;
  unsafe fn get_ptr(&self) -> *const u32;
}
pub type BoxedAttribute = Box<dyn Attribute>;

// -----------------------------------------------------------------------------
// TypedAttribute<T>
// -----------------------------------------------------------------------------

pub struct TypedAttribute<T: Clone + std::fmt::Debug> {
  pub size: usize,
  pub values: Vec<T>,
}

impl<T: Clone + std::fmt::Debug> TypedAttribute<T> {
  pub fn new(array: &[T], size: usize) -> TypedAttribute<T> {
    TypedAttribute::<T> { 
      size: size,
      values: array.iter().cloned().collect(),
    }
  }
}

impl<T: Clone+ std::fmt::Debug> Attribute for TypedAttribute<T> {
  // fn get_ptr();
  fn get_len(&self) -> usize {
    return self.values.len();
  }
  fn get_size(&self) -> usize {
    return self.size;
  }
  // This is a pointer for javascript
  unsafe fn get_ptr(&self) -> *const u32 {
    let ptr = mem::transmute::<*const T, *const u32>(self.values.as_ptr());
    return ptr;
  }
}

pub type Int8Attribute = TypedAttribute<i8>;
pub type Uint8Attribute = TypedAttribute<u8>;
//Uint8ClampedArray();
pub type Int16Attribute = TypedAttribute<i16>;
pub type Uint16Attribute = TypedAttribute<u16>;
pub type Int32Attribute = TypedAttribute<i32>;
pub type Uint32Attribute = TypedAttribute<u32>;
pub type Float32Attribute = TypedAttribute<f32>;
pub type Float64Attribute = TypedAttribute<f64>;


// -----------------------------------------------------------------------------
// WasmBufferAttribute
// -----------------------------------------------------------------------------

#[wasm_bindgen(js_name=BufferAttribute)]
pub struct WasmBufferAttribute {
  attribute: BoxedAttribute,
}
#[wasm_bindgen(js_class=BufferAttribute)]
impl WasmBufferAttribute {
  
  fn from_i8(js_values: JsValue, size: usize) -> WasmBufferAttribute {
    let values: Vec<i8> = Int8Array::from(js_values).to_vec();
    let attribute = Box::new(Int8Attribute { values, size });
    return WasmBufferAttribute { attribute };
  }

  fn from_u8(js_values: JsValue, size: usize) -> WasmBufferAttribute {
    let values: Vec<u8> = Uint8Array::from(js_values).to_vec();
    let attribute = Box::new(Uint8Attribute { values, size });
    return WasmBufferAttribute { attribute };
  }

  fn from_i16(js_values: JsValue, size: usize) -> WasmBufferAttribute {
    let values: Vec<i16> = Int16Array::from(js_values).to_vec();
    let attribute = Box::new(Int16Attribute { values, size });
    return WasmBufferAttribute { attribute };
  }

  fn from_u16(js_values: JsValue, size: usize) -> WasmBufferAttribute {
    let values: Vec<u16> = Uint16Array::from(js_values).to_vec();
    let attribute = Box::new(Uint16Attribute { values, size });
    return WasmBufferAttribute { attribute };
  }

  fn from_i32(js_values: JsValue, size: usize) -> WasmBufferAttribute {
    let values: Vec<i32> = Int32Array::from(js_values).to_vec();
    let attribute = Box::new(Int32Attribute { values, size });
    return WasmBufferAttribute { attribute };
  }

  fn from_u32(js_values: JsValue, size: usize) -> WasmBufferAttribute {
    let values: Vec<u32> = Uint32Array::from(js_values).to_vec();
    let attribute = Box::new(Uint32Attribute { values, size });
    return WasmBufferAttribute { attribute };
  }

  fn from_f32(js_values: JsValue, size: usize) -> WasmBufferAttribute {
    let values: Vec<f32> = Float32Array::from(js_values).to_vec();
    let attribute = Box::new(Float32Attribute { values, size });
    return WasmBufferAttribute { attribute };
  }

  fn from_f64(js_values: JsValue, size: usize) -> WasmBufferAttribute {
    let values: Vec<f64> = Float64Array::from(js_values).to_vec();
    let attribute = Box::new(Float64Attribute { values, size });
    return WasmBufferAttribute { attribute };
  }
  
  #[wasm_bindgen(constructor)]
  pub fn wasm_new(js_values: JsValue, size: usize) -> Result<WasmBufferAttribute, JsValue> {
    if js_values.has_type::<Int8Array>() {
      return Ok(WasmBufferAttribute::from_i8(js_values, size));
    } else if js_values.has_type::<Uint8Array>() {
      return Ok(WasmBufferAttribute::from_u8(js_values, size));
    } else if js_values.has_type::<Int16Array>() {
      return Ok(WasmBufferAttribute::from_i16(js_values, size));
    } else if js_values.has_type::<Uint16Array>() {
      return Ok(WasmBufferAttribute::from_u16(js_values, size));
    } else if js_values.has_type::<Int32Array>() {
      return Ok(WasmBufferAttribute::from_i32(js_values, size));
    } else if js_values.has_type::<Uint32Array>() {
      return Ok(WasmBufferAttribute::from_u32(js_values, size));
    } else if js_values.has_type::<Float32Array>() {
      return Ok(WasmBufferAttribute::from_f32(js_values, size));
    } else if js_values.has_type::<Float64Array>() {
      return Ok(WasmBufferAttribute::from_f64(js_values, size));
    }

    return Err(JsValue::from_str("new BufferAttribute(...What are you passing in?)"));
  }

  #[wasm_bindgen(js_name=getPtr)]
  pub fn get_ptr(&self) -> *const u32 {
    unsafe { return self.attribute.get_ptr() }
  }
  #[wasm_bindgen(js_name=getCount)]
  pub fn get_len(&self) -> usize {
    return self.attribute.get_len();
  }
  #[wasm_bindgen(js_name=getSize)]
  pub fn get_size(&self) -> usize {
    return self.attribute.get_size();
  }
}

