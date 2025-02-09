use sys_info;
use whoami;

fn get_device_info() -> String {
    let os_name = sys_info::os_type().unwrap_or_else(|_| "Unknown OS".to_string());
    let hostname = whoami::hostname();
    format!("Sent from {} ({})", hostname, os_name)
}

struct Message {
    sender: String,
    content: String,
    device_info: String,
}

fn create_message(sender: &str, content: &str) -> Message {
    Message {
        sender: sender.to_string(),
        content: content.to_string(),
        device_info: get_device_info(),
    }
}
