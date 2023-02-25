#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "calculator-wasm-rust-pwa",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(CalcApp::new(cc))),
    )
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
            ui.label(r#"  
            Это вымышленный калькулятор.

            Чтобы воспользоваться калькулятором вам необходимо воспользоваться воображением.
Давайте представим себе любой интерфейс похожий на интерфейс калькулятора и наберите в нем математическое выражение, после чего нажмем на кнопку '='.


            Увидели результат, да? - Поздравляю, ваш калькулятор работает хорошо.

            "#);
        });
    }
}