
use bytes::Bytes;
use tokio::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use mini_redis::{Connection, Frame};
use mini_redis::Command::{Get, Set};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;


async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self};

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("Unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap()
    }
}


#[tokio::main]
async fn main() {
    // Bind listener to address
    let listener = TcpListener::bind("127.0.0.1:6667").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let db = db.clone();

        println!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}
