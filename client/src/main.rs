use websocket::ClientBuilder;
use websocket::Message;

fn main() {
    let mut app = ClientBuilder::new("ws://localhost:8765")
    .unwrap()
    .connect_insecure()
    .unwrap();
    // The app is created
    
}