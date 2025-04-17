use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

type Clients = Arc<Mutex<HashMap<String, TcpStream>>>;

fn handle_client(mut stream: TcpStream, addr: String, clients: Clients) {
    let reader = BufReader::new(stream.try_clone().unwrap());

    for line in reader.lines() {
        let msg = line.unwrap();
        println!("Received: {}", msg);

        let clients = clients.lock().unwrap();
        for (other_addr, other_stream) in clients.iter() {
            if other_addr != &addr {
                writeln!(other_stream.try_clone().unwrap(), "{}: {}", addr, msg).ok();
            }
        }
    }

    println!("{} disconnected", addr);
    clients.lock().unwrap().remove(&addr);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();
    println!("Server running on 127.0.0.1:4000");

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let addr = stream.peer_addr().unwrap().to_string();
        clients.lock().unwrap().insert(addr.clone(), stream.try_clone().unwrap());

        let clients = Arc::clone(&clients);
        thread::spawn(move || {
            handle_client(stream, addr, clients);
        });
    }
}
