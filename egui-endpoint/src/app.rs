use eframe::egui;

#[derive(Debug)]
pub struct MyEguiApp {
    name: String,
    age: f64,
}
impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            name: "Ultrabug".to_string(),
            age: 40.,
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let Self { name, age } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust egui application on Spin");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(name).labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(age, 0.0..=120.0).text("age"));
            if ui.button("Click each year").clicked() {
                *age += 1.;
            }
            ui.label(format!("Hello '{name}', age {age}"));
        });
    }
}
