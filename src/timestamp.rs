use chrono::Local;

struct MessageApp {
    messages: Vec<(String, String)>,
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
                for (timestamp, message) in &self.messages {
                    ui.label(format!("[{}] {}", timestamp, message));
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.input_text);
                if ui.button("Send").clicked() {
                    if !self.input_text.trim().is_empty() {
                        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                        self.messages.push((timestamp, self.input_text.clone()));
                        self.input_text.clear();
                    }
                }
            });
        });
    }
}
