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
    Get,
    Drop,
    Set
}

pub enum ClientRequest {
    GetByMac {
        // user: String,
        password: String,
        // method: get
        // how: mac
        mac: ipparser::MacAddress
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
            let password;
            let user;
            if let Some(pass) = request.get("password") {
                if let Some(pass) = pass.as_str() {
                    password = String::from(pass);
                    if let Some(usr) = request.get("user") {
                        if let Some(usr) = usr.as_str() {
                            match usr.to_lowercase().as_str() {
                                "admin" | "client" => user = String::from(usr.to_lowercase()),
                                _ => return None
                            }
                            if let Some(method) = request.get("method") {
                                if let Some(method) = method.as_str() {
                                    match method {
                                        "get" => {
                                            if user == "admin" { // AdminGet
                                                return Some(Request::Admin(AdminRequest::Get));
                                            } else { // ClientGet
                                                if let Some(how) = request.get("how") {
                                                    if let Some(how) = how.as_str() {
                                                        match how.to_lowercase().as_str() {
                                                            "mac" => { // ClientRequest::GetByMac
                                                                if let Some(mac) = request.get("mac") {
                                                                    if let Some(mac) = mac.as_str() {
                                                                        if let Some(mac) = ipparser::MacAddress::new_from_str(mac) {
                                                                            return Some(Request::Client(ClientRequest::GetByMac{ password, mac}));
                                                                        }
                                                                    }
                                                                }
                                                            },
                                                            "username" => { // ClientRequest::GetByUsername

                                                            },
                                                            _ => return None
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        "drop" => {
                                            if user == "admin" { // AdminDrop

                                            } else { // ClientDrop

                                            }
                                        },
                                        "set" => { // Admin only
                                            if user != "admin" { return None; }
                                        }, 
                                        "sign_up" => { // Client only
                                            if user != "client" { return None; }
                                        },
                                        _ => return None
                                    }
                                }
                            }
                        }
                    }
                }
            }            
        }
        None
    }
}

pub fn describe(_request: &str, _peer_addr: &net::SocketAddr) {
}
