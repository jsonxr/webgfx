use wasm_bindgen::prelude::*;
use web_sys::console;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


mod box_geometry;
pub use box_geometry::BoxGeometry;

mod cameras;
pub use cameras::PerspectiveCamera;

mod mesh;
pub use mesh::Mesh;

mod mesh_basic_material;
pub use mesh_basic_material::MeshBasicMaterial;

mod renderers;
pub use renderers::*;

mod scene;
pub use scene::*;

mod vectors;
pub use vectors::*;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world from rust!"));

    Ok(())
}
