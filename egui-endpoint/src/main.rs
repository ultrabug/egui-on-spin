use egui_endpoint::MyEguiApp;

/// native local execution
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("starting native runner");

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust egui application on Spin",
        native_options,
        Box::new(|_cc| Box::<MyEguiApp>::default()),
    )
}

/// web execution
#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
#[wasm_bindgen]
pub struct WebHandle {
    runner: eframe::WebRunner,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WebHandle {
    /// Installs a panic hook, then returns.
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            runner: eframe::WebRunner::new(),
        }
    }

    #[wasm_bindgen]
    pub async fn start(&self, canvas_id: &str) -> Result<(), wasm_bindgen::JsValue> {
        tracing::info!("starting wasm runner");
        self.runner
            .start(
                canvas_id,
                eframe::WebOptions::default(),
                Box::new(|_cc| Box::<MyEguiApp>::default()),
            )
            .await
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    tracing_wasm::set_as_global_default();
    tracing::info!("main wasm webapp");

    wasm_bindgen_futures::spawn_local(async {
        let _ = WebHandle::new().start("the_spin_canvas").await;
    });
}
