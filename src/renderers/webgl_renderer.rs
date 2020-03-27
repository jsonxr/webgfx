use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::*;
use web_sys::{console, HtmlCanvasElement, WebGlProgram, WebGlRenderingContext, WebGlShader};

use super::super::cameras::PerspectiveCamera;
use super::super::scene::Scene;

//struct MyError;

#[wasm_bindgen]
pub struct WebGLRenderer {
  canvas: Option<HtmlCanvasElement>,
  context: Option<WebGlRenderingContext>,
  pub width: u32,
  height: u32,

  secret: u32,
}


// #[derive(Serialize, Deserialize, Debug)]
// struct WebGLRendererOptions<'a> {
//   pub canvas: &'a HtmlCanvasElement,
//   pub callback: &'a js_sys::Function,
// }

#[wasm_bindgen]
impl WebGLRenderer {
  /**
   *
   */
  #[wasm_bindgen(constructor)]
  pub fn new(values: JsValue) -> WebGLRenderer {
    console::log_1(&JsValue::from_str("values..."));
    console::log_1(&values);

    // let value: std::result::Result<WebGLRendererOptions, serde_wasm_bindgen::Error> = serde_wasm_bindgen::from_value(values);
    // let v = value.unwrap();
    // console::log_1(&JsValue::from(v.canvas));
    // console::log_1(&JsValue::from(v.callback));

    WebGLRenderer {
      canvas: None,
      context: None,
      width: 960,
      height: 540,
      secret: 0,
    }
  }

  /**
   *
   */
  pub fn set_size(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
    self.width = width;
    self.height = height;
    Ok(())
  }

  #[wasm_bindgen(js_name = setCanvas)]
  pub fn set_canvas(&mut self, canvas: HtmlCanvasElement) -> Result<(), JsValue> {
    match canvas.dyn_into::<HtmlCanvasElement>() {
      Ok(canvas) => {
        match canvas.get_context("webgl")? {
          Some(context) => {
            console::log_1(&JsValue::from_str("yes, have context"));
            self.canvas = Some(canvas);
            self.context = Some(WebGlRenderingContext::from(JsValue::from(context)));
          }
          None => {
            return Err(JsValue::from_str("Can't get webgl context from canvas"));
          }
        }
        Ok(())
      }
      Err(_) => Err(JsValue::from_str("but i'm not a canvas")),
    }
  }

  /**
   * Sets context
   */
  #[wasm_bindgen(js_name = setContext)]
  pub fn set_context(&mut self, context: WebGlRenderingContext) -> Result<(), JsValue> {
    match context.dyn_into::<WebGlRenderingContext>() {
      Ok(context) => {
        console::log_1(&context);
        self.context = Some(context);
        Ok(())
      }
      Err(_) => Err(JsValue::from_str("but i'm not a context")),
    }
  }

  pub fn render(&self, _scene: &Scene, _camera: &PerspectiveCamera) -> Result<(), JsValue> {
    // let document = web_sys::window().unwrap().document().unwrap();
    // let canvas = document.get_element_by_id("canvas").unwrap();
    // let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    // let context = self.canvas
    //   .get_context("webgl")?
    //   .unwrap()
    //   .dyn_into::<WebGlRenderingContext>()?;
    if let Some(_context) = self.context.as_ref() {
      println!("context is not null");
    } else {
      return Err(JsValue::from_str("canvas not set"));
    }

    let context = self.context.as_ref().unwrap();
    let vert_shader_str = include_str!("shaders/vert.glsl");
    let vert_shader = compile_shader(
      &context,
      WebGlRenderingContext::VERTEX_SHADER,
      vert_shader_str,
    )?;

    let frag_shader_str = include_str!("shaders/frag.glsl");
    let frag_shader = compile_shader(
      &context,
      WebGlRenderingContext::FRAGMENT_SHADER,
      frag_shader_str,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
      let vert_array = js_sys::Float32Array::view(&vertices);

      context.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vert_array,
        WebGlRenderingContext::STATIC_DRAW,
      );
    }

    context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(0);

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(
      WebGlRenderingContext::TRIANGLES,
      0,
      (vertices.len() / 3) as i32,
    );
    Ok(())
  }
}

pub fn compile_shader(
  context: &WebGlRenderingContext,
  shader_type: u32,
  source: &str,
) -> Result<WebGlShader, String> {
  let shader = context
    .create_shader(shader_type)
    .ok_or_else(|| String::from("Unable to create shader object"))?;
  context.shader_source(&shader, source);
  context.compile_shader(&shader);

  if context
    .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(shader)
  } else {
    Err(
      context
        .get_shader_info_log(&shader)
        .unwrap_or_else(|| String::from("Unknown error creating shader")),
    )
  }
}

pub fn link_program(
  context: &WebGlRenderingContext,
  vert_shader: &WebGlShader,
  frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
  let program = context
    .create_program()
    .ok_or_else(|| String::from("Unable to create shader object"))?;

  context.attach_shader(&program, vert_shader);
  context.attach_shader(&program, frag_shader);
  context.link_program(&program);

  if context
    .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(program)
  } else {
    Err(
      context
        .get_program_info_log(&program)
        .unwrap_or_else(|| String::from("Unknown error creating program object")),
    )
  }
}
