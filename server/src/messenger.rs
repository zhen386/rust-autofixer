use lsp_server::{Connection, IoThreads, Message};
use anyhow;


pub struct Messenger { }

impl Messenger {

    pub fn new () -> (Connection, IoThreads) {
        Connection::stdio()
    }

}

/*
connection.sender.send(lsp_server::Message::Notification(not)).unwrap();
*/

