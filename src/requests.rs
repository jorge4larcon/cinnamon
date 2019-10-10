extern crate serde_json;

use std::net;

// La funcion tomara un string y lo transformara en un
// Enum de client request

pub enum Request {
    Admin(AdminRequest),
    Client(ClientRequest)
}

pub enum AdminRequest {
}

pub enum ClientRequest {
    GET_BY_MAC {
        // user: String,
        password: String,
        // method: get
        // how: mac
        mac: String
    },
    GET_BY_USERNAME {
        // user: String,
        password: String,
        // method: get
        // how: username
        username: String,
        start_index: usize
    },
    DROP {
        
    },
    SIGN_UP {}
}

pub fn describe(request: &str, peer_addr: &net::SocketAddr) -> String {
    if let Ok(request) = serde_json::from_str::<serde_json::Value>(request) {

    } else {

    }
}

/*

192.168.1.70 says GET 

*/