fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 200.0)),
        ..Default::default()
    };

    eframe::run_native("Tea", options, Box::new(|_cc| Box::<MyApp>::default()));
}

struct MyApp {
    base: u16,
    add: u16,
    infusion: u16,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            base: 10,
            add: 5,
            infusion: 1,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tea steeping tracker:");
            ui.add(egui::Slider::new(&mut self.base, 0..=300).text("First infusion"));
            ui.add(egui::Slider::new(&mut self.add, 0..=60).text("Time per infusion"));
            ui.add(egui::Slider::new(&mut self.infusion, 1..=25).text("Next infusion Nr."));
            let infusion_time = self.base + self.add * (self.infusion - 1);
            if ui.button("Next infusion ->").clicked() {
                self.infusion += 1;
            }
            ui.label(format!(
                "Your next infusion, number {}, takes {}s.",
                self.infusion, infusion_time
            ));
        });
    }
}
