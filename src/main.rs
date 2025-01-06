use fltk::{app, button::Button, frame::Frame, input::Input, prelude::*, text::TextBuffer, text::TextDisplay, window::Window};
use gstreamer::prelude::*;
use std::sync::{Arc, Mutex};

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

win.set_color(fltk::enums::Color::White);
text_display.set_text_color(fltk::enums::Color::Black);
send_button.set_color(fltk::enums::Color::Green);

use std::fs::OpenOptions;
use std::io::{Read, Write};

fn save_messages(buffer: &TextBuffer) {
    let mut file = OpenOptions::new().write(true).create(true).open("messages.txt").unwrap();
    file.write_all(buffer.text().as_bytes()).unwrap();
}

fn load_messages(buffer: &mut TextBuffer) {
    let mut file = OpenOptions::new().read(true).open("messages.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    buffer.set_text(&content);
}

use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

async fn send_message_to_server(message: &str) {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    stream.write_all(message.as_bytes()).await.unwrap();
}

async fn receive_messages_from_server() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let mut buffer = vec![0; 1024];
    loop {
        let n = stream.read(&mut buffer).await.unwrap();
        if n == 0 {
            break;
        }
        let message = String::from_utf8_lossy(&buffer[..n]);
        println!("Received: {}", message);
    }
}

fn main() {
    
    gstreamer::init().unwrap();

    let app = app::App::default();
    let mut win = Window::new(100, 100, 640, 480, "Video Call");
    let mut video_frame = Frame::new(10, 10, 620, 460, "");

    win.end();
    win.show();

    
    let buffer = Arc::new(Mutex::new(vec![0u8; 640 * 480 * 3]));

    
    let pipeline = gstreamer::parse_launch(
        "videotestsrc ! video/x-raw,format=RGB,width=640,height=480 ! appsink name=sink",
    )
    .unwrap();
    let appsink = pipeline
        .dynamic_cast::<gstreamer::Pipeline>()
        .unwrap()
        .get_by_name("sink")
        .unwrap()
        .dynamic_cast::<gstreamer::AppSink>()
        .unwrap();

    
    let buffer_clone = buffer.clone();
    appsink.set_callbacks(
        gstreamer::AppSinkCallbacks::builder()
            .new_sample(move |sink| {
                if let Some(sample) = sink.pull_sample().ok() {
                    let buffer = sample.buffer().unwrap();
                    let map = buffer.map_read().unwrap();

                    let mut shared_buffer = buffer_clone.lock().unwrap();
                    shared_buffer.copy_from_slice(&map);
                }
                Ok(gstreamer::FlowSuccess::Ok)
            })
            .build(),
    );

    pipeline.set_state(gstreamer::State::Playing).unwrap();

    
    app::add_idle(move || {
        let buffer = buffer.lock().unwrap();
        video_frame.draw(move |f| {
            let img = fltk::image::RgbImage::new(&buffer, 640, 480, fltk::enums::ColorDepth::Rgb8)
                .unwrap();
            img.draw(f.x(), f.y(), f.w(), f.h());
        });
    });

    app.run().unwrap();
}
