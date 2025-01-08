use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Convo",
        options,
        Box::new(|_cc| Box::new(MessageApp::default())),
    )
}

struct MessageApp {
    messages: Vec<String>,
    input_text: String,
}

impl Default for MessageApp {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
            input_text: String::new(),
        }
    }
}

impl eframe::App for MessageApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Convo");

            egui::ScrollArea::vertical().show(ui, |ui| {
                for message in &self.messages {
                    ui.label(message);
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.input_text);
                if ui.button("Send").clicked() {
                    if !self.input_text.trim().is_empty() {
                        self.messages.push(self.input_text.clone());
                        self.input_text.clear();
                    }
                }
            });
        });
    }
}
