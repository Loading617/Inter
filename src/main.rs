use fltk::{app, button::Button, frame::Frame, input::Input, prelude::*, text::TextBuffer, text::TextDisplay, window::Window};

fn main() {
    let app = app::App::default();
    let mut win = Window::new(100, 100, 400, 600, "Convo");

    
    let mut text_display = TextDisplay::new(10, 10, 380, 450, "");
    let mut text_buffer = TextBuffer::default();
    text_display.set_buffer(Some(text_buffer.clone()));

    
    let mut input = Input::new(10, 480, 300, 30, "");

    
    let mut send_button = Button::new(320, 480, 70, 30, "Send");

    
    send_button.set_callback({
        let mut buffer = text_buffer.clone();
        let mut input_clone = input.clone();
        move |_| {
            if !input_clone.value().trim().is_empty() {
                let current_text = buffer.text();
                let new_message = format!("{}\nYou: {}", current_text, input_clone.value());
                buffer.set_text(&new_message);
                input_clone.set_value(""); 
            }
        }
    });

    
    win.end();
    win.show();
    app.run().unwrap();
}

text_display.wrap_mode(fltk::text::WrapMode::AtBounds, 0);

std::thread::spawn({
    let mut buffer = text_buffer.clone();
    move || loop {
        std::thread::sleep(std::time::Duration::from_secs(5));
        app::lock().unwrap();
        let current_text = buffer.text();
        let new_message = format!("{}\nFriend: Hello!", current_text);
        buffer.set_text(&new_message);
        app::unlock();
        app::awake();
    }
});
