#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

use calculator_wasm_rust_pwa::keyboard;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    eframe::run_native(
        "calculator-wasm-rust-pwa",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(CalcApp::new(cc))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Убедитесь, что паника регистрируется с помощью `console.error`.
    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "calculator-wasm-rust-pwa",
            eframe::WebOptions::default(),
            Box::new(|cc| Box::new(CalcApp::new(cc))),
        )
            .await
            .expect("failed to start calculator-wasm-rust-pwa");
    });
}

struct CalcApp {
    math_exp: Vec<String>,
}

impl CalcApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        CalcApp {
            math_exp: Vec::new(),
        }
    }
}

impl eframe::App for CalcApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            keyboard::CalcKeyboard::from_buffer(&mut self.math_exp).show(ui)
        });
    }
}