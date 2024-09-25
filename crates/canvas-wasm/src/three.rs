// use stl_io::{read_stl, IndexedMesh};
// use wasm_bindgen::prelude::*;
// use web_sys::{console, window, WebGlBuffer, WebGlRenderingContext};

// include!(concat!(env!("OUT_DIR"), "/poi.rs"));

// #[wasm_bindgen(start)]
// pub fn start_three_impl() -> Result<(), JsValue> {
//     // WebページのDOMからcanvas要素を取得
//     let document = window().unwrap().document().unwrap();
//     let canvas = document
//         .get_element_by_id("canvas")
//         .unwrap()
//         .dyn_into::<web_sys::HtmlCanvasElement>()?;

//     let gl = canvas
//         .get_context("webgl")?
//         .unwrap()
//         .dyn_into::<WebGlRenderingContext>()?;

//     // STLデータをパース
//     let mesh = parse_stl(POI_MESH_STL).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

//     console::log_1(&JsValue::from_str(&format!(
//         "Parsed mesh with {} vertices and {} triangles",
//         mesh.vertices.len(),
//         mesh.faces.len()
//     )));

//     // WebGLバッファの準備
//     let vertex_buffer = gl.create_buffer().ok_or("Failed to create buffer")?;
//     gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

//     let vertices: Vec<f32> = mesh
//         .vertices
//         .iter()
//         .flat_map(|v| vec![v[0], v[1], v[2]])
//         .collect();
//     unsafe {
//         let vert_array = js_sys::Float32Array::view(&vertices);
//         gl.buffer_data_with_array_buffer_view(
//             WebGlRenderingContext::ARRAY_BUFFER,
//             &vert_array,
//             WebGlRenderingContext::STATIC_DRAW,
//         );
//     }

//     // 簡単なシェーダーの作成
//     let vert_shader = compile_shader(
//         &gl,
//         WebGlRenderingContext::VERTEX_SHADER,
//         r#"
//         attribute vec3 position;
//         void main() {
//             gl_Position = vec4(position, 1.0);
//         }
//         "#,
//     )?;
//     let frag_shader = compile_shader(
//         &gl,
//         WebGlRenderingContext::FRAGMENT_SHADER,
//         r#"
//         void main() {
//             gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
//         }
//         "#,
//     )?;
//     let program = link_program(&gl, &vert_shader, &frag_shader)?;
//     gl.use_program(Some(&program));

//     // 属性バインディング
//     let position_attrib_location = gl.get_attrib_location(&program, "position") as u32;
//     gl.enable_vertex_attrib_array(position_attrib_location);
//     gl.vertex_attrib_pointer_with_i32(
//         position_attrib_location,
//         3,
//         WebGlRenderingContext::FLOAT,
//         false,
//         0,
//         0,
//     );

//     // 描画
//     gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
//     gl.draw_arrays(
//         WebGlRenderingContext::TRIANGLES,
//         0,
//         (vertices.len() / 3) as i32,
//     );

//     Ok(())
// }

// fn parse_stl(stl_data: &[u8]) -> Result<IndexedMesh, stl_io::Error> {
//     let mut reader = std::io::Cursor::new(stl_data);
//     read_stl(&mut reader)
// }

// fn compile_shader(
//     gl: &WebGlRenderingContext,
//     shader_type: u32,
//     source: &str,
// ) -> Result<web_sys::WebGlShader, String> {
//     let shader = gl
//         .create_shader(shader_type)
//         .ok_or_else(|| String::from("Unable to create shader object"))?;
//     gl.shader_source(&shader, source);
//     gl.compile_shader(&shader);

//     if gl
//         .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
//         .as_bool()
//         .unwrap_or(false)
//     {
//         Ok(shader)
//     } else {
//         Err(gl
//             .get_shader_info_log(&shader)
//             .unwrap_or_else(|| String::from("Unknown error creating shader")))
//     }
// }

// fn link_program(
//     gl: &WebGlRenderingContext,
//     vert_shader: &web_sys::WebGlShader,
//     frag_shader: &web_sys::WebGlShader,
// ) -> Result<web_sys::WebGlProgram, String> {
//     let program = gl
//         .create_program()
//         .ok_or_else(|| String::from("Unable to create shader program"))?;

//     gl.attach_shader(&program, vert_shader);
//     gl.attach_shader(&program, frag_shader);
//     gl.link_program(&program);

//     if gl
//         .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
//         .as_bool()
//         .unwrap_or(false)
//     {
//         Ok(program)
//     } else {
//         Err(gl
//             .get_program_info_log(&program)
//             .unwrap_or_else(|| String::from("Unknown error creating program object")))
//     }
// }
