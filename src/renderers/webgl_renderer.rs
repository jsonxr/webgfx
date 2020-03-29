use wasm_bindgen::prelude::*;
use js_sys::{Reflect};
use web_sys::{console, HtmlCanvasElement, WebGlProgram, WebGlRenderingContext, WebGlShader};

use super::super::cameras::PerspectiveCamera;
use super::super::scene::Scene;

/**
 * Creates a canvas given options as an input. If "canvas" is provided, it uses
 * it, otherwise it will append a canvas to the body.
 */
fn get_or_create_canvas(options: &JsValue) -> Result<HtmlCanvasElement, JsValue> {
  if !options.is_undefined() {
    let ele: JsValue = Reflect::get(&options, &JsValue::from_str("canvas"))?;
    // options = { canvas = null }
    if ele.is_null() {
      return Err(JsValue::from_str("new WebGLRenderer(): canvas can not be null."));
    }

    // options = { canvas: document.getElementById('canvas') }
    if !ele.is_undefined() {
      console::log_1(&JsValue::from_str("Return here!"));
      let canvas = HtmlCanvasElement::from(JsValue::from(ele));
      return Ok(canvas);
    }
  }

  // options = { }
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");
  let canvas_ele = document.create_element("canvas")?;
  body.append_child(&canvas_ele)?;
  let canvas = HtmlCanvasElement::from(JsValue::from(canvas_ele));
  return Ok(canvas);
}

#[wasm_bindgen(inspectable)]
pub struct WebGLRenderer {
  _canvas: HtmlCanvasElement,
  context: WebGlRenderingContext,
  
  #[wasm_bindgen(readonly)]
  pub width: u32,

  #[wasm_bindgen(readonly)]
  pub height: u32,
}

impl WebGLRenderer {
  pub fn new(canvas: HtmlCanvasElement, context: WebGlRenderingContext) -> WebGLRenderer {
    WebGLRenderer {
      _canvas: canvas,
      context,
      width: 10,
      height: 10,
    }
  }
}


#[wasm_bindgen]
impl WebGLRenderer {
  /**
   *
   */
  #[wasm_bindgen(constructor)]
  pub fn wasm_new(options: &JsValue) -> Result<WebGLRenderer, JsValue> {
    let canvas: HtmlCanvasElement = get_or_create_canvas(&options)?;
    let context = canvas.get_context("webgl")?;
    let context = WebGlRenderingContext::from(JsValue::from(context));
    return Ok(WebGLRenderer::new(canvas, context));
  }

  pub fn render(&self, _scene: &Scene, _camera: &PerspectiveCamera) -> Result<(), JsValue> {
    let context = &self.context;
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
