//use image::GenericImageView;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
#[repr(C)]
#[derive(Serialize)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Export a `generate_image` function from Rust to JavaScript,
// that generates image.
#[wasm_bindgen]
pub fn process_image(data: &[u8]) -> Result<JsValue, JsValue> {
    let img = image::load_from_memory(data).map_err(|e| JsValue::from_str(&e.to_string()))?;

    if img.width() > 100 && img.height() > 100 {
        let vertices = generate_cube_vertices();
        serde_wasm_bindgen::to_value(&vertices).map_err(|e| JsValue::from_str(&e.to_string()))
    } else {
        Ok(JsValue::NULL)
    }
}

fn generate_cube_vertices() -> Vec<Vertex> {
    vec![
        Vertex {
            x: -1.0,
            y: -1.0,
            z: -1.0,
        },
        Vertex {
            x: -1.0,
            y: -1.0,
            z: 1.0,
        },
        Vertex {
            x: -1.0,
            y: 1.0,
            z: -1.0,
        },
        Vertex {
            x: -1.0,
            y: 1.0,
            z: 1.0,
        },
        Vertex {
            x: 1.0,
            y: -1.0,
            z: -1.0,
        },
        Vertex {
            x: 1.0,
            y: -1.0,
            z: 1.0,
        },
        Vertex {
            x: 1.0,
            y: 1.0,
            z: -1.0,
        },
        Vertex {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    ]
}
