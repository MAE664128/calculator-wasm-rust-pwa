#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;


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

struct CalcApp {}

impl CalcApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        CalcApp {}
    }
}

impl eframe::App for CalcApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("√").small(),
                ).clicked() {  };
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("C").small(),
                ).clicked() {  };
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("(").small(),
                ).clicked() {  };
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new(")").small(),
                ).clicked() {  };
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("<=").small(),
                ).clicked() {  };
            });
            ui.horizontal(|ui| {
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("sin").small(),
                ).clicked() {  };
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("7").small(),
                ).clicked() {  };
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("8").small(),
                ).clicked() {  };
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("9").small(),
                ).clicked() {  };
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("*").small(),
                ).clicked() {  };
            });
            ui.horizontal(|ui| {
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("cos").small(),
                ).clicked() {  };
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("4").small(),
                ).clicked() {  };
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("5").small(),
                ).clicked() {  };
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("6").small(),
                ).clicked() {  };
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("/").small(),
                ).clicked() {  };
            });
            ui.horizontal(|ui| {
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("tg").small(),
                ).clicked() {  };
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("1").small(),
                ).clicked() {  };
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("2").small(),
                ).clicked() {  };
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("3").small(),
                ).clicked() {  };
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("-").small(),
                ).clicked() {  };
            });
            ui.horizontal(|ui| {
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("ctg").small(),
                ).clicked() {  };
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new(".").small(),
                ).clicked() {  };
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("0").small(),
                ).clicked() {  };
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("=").small(),
                ).clicked() {  };
                if ui.add_sized(
                    [58.0, 48.0],
                    egui::Button::new("+").small(),
                ).clicked() {  };
            });
        });
    }
}