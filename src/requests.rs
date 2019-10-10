extern crate serde_json;

use std::net;
use crate::ipparser;

// La funcion tomara un string y lo transformara en un
// Enum de client request

pub enum Request {
    Admin(AdminRequest),
    Client(ClientRequest)
}

pub enum AdminRequest {
    Base
}

pub enum ClientRequest {
    GetByMac {
        // user: String,
        password: String,
        // method: get
        // how: mac
        mac: String
    },
    GetByUsername {
        // user: String,
        password: String,
        // method: get
        // how: username
        username: String,
        start_index: usize
    },
    Drop {
        // user
        password: String,
        ip: net::Ipv4Addr
    },
    SignUP {
        password: String,
        username: String,
        mac: ipparser::MacAddress,
        port: u16,
        get_only_by_mac: bool        
    }
}

impl Request {
    pub fn from(request_str: &str) -> Option<Request> {
        if let Ok(request) = serde_json::from_str::<serde_json::Value>(request_str) {
            if let Some(user) = request.get("user") {
                if let Some(_user) = user.as_str() {
                    return Some(Request::Admin(AdminRequest::Base));
                }
            }
        }
        None
    }
}

pub fn describe(_request: &str, _peer_addr: &net::SocketAddr) {
}

/*

192.168.1.70 says GET 

*/