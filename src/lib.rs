#![warn(clippy::all, rust_2018_idioms)]

mod fractal_clock;
pub use fractal_clock::FractalClockApp;

// ----------------------------------------------------------------------------
// When compiling for web:

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    eframe::start_web(canvas_id, Box::new(|cc| Box::new(FractalClockApp::new(cc))))
}

pub(crate) fn seconds_since_midnight() -> Option<f64> {
    use chrono::Timelike;
    let time = chrono::Local::now().time();
    let seconds_since_midnight =
        time.num_seconds_from_midnight() as f64 + 1e-9 * (time.nanosecond() as f64);
    Some(seconds_since_midnight)
}
