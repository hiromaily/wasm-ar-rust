use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

// Ball parameter
const BALL_RADIUS: f64 = 10.0;
const BALL_COLOR: &str = "#0095DD";

const CANVAS_W: f64 = 800.0;
const CANVAS_H: f64 = 600.0;
//const BACKGROUND_COLOR: &str = "#FFFFFF";

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    // Set up console error hook
    console_error_panic_hook::set_once();

    // Setup browser basic DOM element
    let window = window().ok_or("should have a window in this context")?;
    let document = window
        .document()
        .ok_or("should have a document on window")?;
    let body = document.body().ok_or("document should have a body")?;

    // Create canvas element
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_width(CANVAS_W as u32);
    canvas.set_height(CANVAS_H as u32);
    body.append_child(&canvas)?;

    let context = canvas
        .get_context("2d")?
        .ok_or("getContext should return a context")?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    //context.set_fill_style(&JsValue::from_str(BACKGROUND_COLOR));

    // Ball initial parameter
    let mut x = 50.0;
    let mut y = 50.0;
    let mut dx = 2.0;
    let mut dy = 2.0;

    // Create a reference count storage for the callback
    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();

    let window_clone = window.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        // Reset
        context.clear_rect(0.0, 0.0, CANVAS_W, CANVAS_H);

        // Start Animation
        context.begin_path();
        context
            .arc(x, y, BALL_RADIUS, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        context.set_fill_style(&JsValue::from_str(BALL_COLOR));
        context.fill();
        context.close_path();

        if x + dx > CANVAS_W || x + dx < 0.0 {
            dx = -dx;
        }
        if y + dy > CANVAS_H || y + dy < 0.0 {
            dy = -dy;
        }

        x += dx;
        y += dy;

        // Schedule the next frame
        window_clone
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }) as Box<dyn FnMut()>));

    // Initial call to requestAnimationFrame
    window.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;

    Ok(())
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
