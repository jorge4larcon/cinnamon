extern crate serde_json;

use std::net;
use std::fmt;
use crate::ipparser;
use std::convert::TryFrom;

// La funcion tomara un string y lo transformara en un
// Enum de client request

pub enum Request {
    Admin(AdminRequest),
    Client(ClientRequest)
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
                                                                            return Some(Request::Client(ClientRequest::GetByMac { password, mac} ));
                                                                        }
                                                                    }
                                                                }
                                                            },
                                                            "username" => { // ClientRequest::GetByUsername
                                                                if let Some(username) = request.get("username") {
                                                                    if let Some(username) = username.as_str() {
                                                                        if let Some(start_index) = request.get("start_index") {
                                                                            if let Some(start_index) = start_index.as_u64() {
                                                                                let start_index = start_index as usize;
                                                                                let username = String::from(username);
                                                                                return Some(Request::Client(ClientRequest::GetByUsername { password, username, start_index } ))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            },
                                                            _ => return None
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        "drop" => {
                                            if user == "admin" { // AdminDrop
                                                return Some(Request::Admin(AdminRequest::Drop));
                                            } else { // ClientDrop
                                                if let Some(ip) = request.get("ip") {
                                                    if let Some(ip) = ip.as_str() {
                                                        if let Some(ip) = ipparser::str_to_ipv4addr(ip) {
                                                            return Some(Request::Client(ClientRequest::Drop { password, ip } ))
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        "set" => { // Admin only
                                            if user != "admin" { return None; }
                                        }, 
                                        "sign_up" => { // Client only
                                            if user != "client" { return None; }
                                            if let Some(username) = request.get("username") {
                                                if let Some(username) = username.as_str() {
                                                    let username = String::from(username);
                                                    if let Some(mac) = request.get("mac") {
                                                        if let Some(mac) = mac.as_str() {
                                                            if let Some(mac) = ipparser::MacAddress::new_from_str(mac) {
                                                                if let Some(port) = request.get("port") {
                                                                    if let Some(port) = port.as_u64() {
                                                                        if let Ok(port) = u16::try_from(port) {
                                                                            if let Some(gobm) = request.get("get_only_by_mac") {
                                                                                if let Some(get_only_by_mac) = gobm.as_bool() {
                                                                                    return Some(Request::Client(ClientRequest::SignUp { password, username, mac, port, get_only_by_mac } ));
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Request::Admin(admin_request) => {
                match admin_request {
                    AdminRequest::Get => write!(f, "ADMIN Get"),
                    AdminRequest::Drop => write!(f, "ADMIN Drop"),
                    AdminRequest::Set => write!(f, "ADMIN Set")
                }
            },
            Request::Client(client_request) => {
                match client_request {                    
                    ClientRequest::GetByMac { password: _password, mac } => {
                        write!(f, "CLIENT GET {}", mac)
                    },
                    ClientRequest::GetByUsername { password: _password, username, start_index } => {
                        write!(f, "CLIENT GET \"{}\" starting from {}", username, start_index)
                    },
                    ClientRequest::Drop { password: _password, ip } => {
                        write!(f, "CLIENT DROP {}", ip)
                    },
                    ClientRequest::SignUp { password: _password, username, mac, port, get_only_by_mac } => {
                        if *get_only_by_mac {
                            write!(f, "CLIENT SIGN-UP \"{}\" {} PORT: {} MAC-ONLY", username, mac, port)
                        } else {
                            write!(f, "CLIENT SIGN-UP \"{}\" {} PORT: {}", username, mac, port)
                        }
                    }
                }
            }
        }
    }
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
    SignUp {
        password: String,
        username: String,
        mac: ipparser::MacAddress,
        port: u16,
        get_only_by_mac: bool        
    }
}
