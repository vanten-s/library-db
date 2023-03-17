use websocket;

use websocket::sync::Server;
use websocket::Message;

use std::collections::HashMap;

fn make_books() -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    map.insert(String::from("Mein Kampf"), String::from("kampf.html"));

    return map;
}

//
// Send Message
//

// let message = Message::text("msg");
// _ = client.send_message(&message);

fn main() {
    let books = make_books();

    let server = Server::bind("127.0.0.1:1234").unwrap();

    for connection in server.filter_map(Result::ok) {
        let mut client = connection.accept().unwrap();

        let book = client.recv_dataframe().unwrap().data;
        let book_string = String::from_utf8(book).unwrap();

        let book_filename_result = books.get(&book_string);

        let message = match book_filename_result {
            Some(filename) => filename,
            None           => "E1" 
        };
        let _ = client.send_message(&Message::text(message));
    }
}
