extern crate serde_json;
extern crate log;

use std::net;
use std::fmt;
use crate::ipparser;
use crate::clients;
use crate::server;
use std::convert::TryFrom;


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
                    log::debug!("Request::from - password obtained ({})", password);
                    if let Some(usr) = request.get("user") {
                        if let Some(usr) = usr.as_str() {
                            match usr.to_lowercase().as_str() {
                                "admin" | "client" => user = String::from(usr.to_lowercase()),
                                _ => {
                                    log::info!("Request::from - incorrect user type ({})", usr);
                                    return None;
                                }
                            }
                            log::debug!("Request::from - user obtained ({})", user);
                            if let Some(method) = request.get("method") {
                                if let Some(method) = method.as_str() {
                                    match method {
                                        "get" => {
                                            log::debug!("Request::from - method obtained (get)");
                                            if user == "admin" { // ======================================================= AdminGet
                                                log::debug!("Request::from - parsing request AdminRequest::Get");                                                
                                                if let Some(how) = request.get("how") {
                                                    if let Some(how) = how.as_str() {
                                                        match how.to_lowercase().as_str() {
                                                            "mac" => { // AdminRequest::GetByMac
                                                                log::debug!("Request::from - parsing request: AdminRequest::GetByMac how obtained (mac)");
                                                                if let Some(mac) = request.get("mac") {
                                                                    if let Some(mac) = mac.as_str() {
                                                                        if let Some(mac) = ipparser::MacAddress::new_from_str(mac) {
                                                                            log::debug!("Request::from - parsing request: AdminRequest::GetByMac mac obtained ({})", mac);
                                                                            log::debug!("Request::from - parsed request: AdminRequest::GetByMac");
                                                                            return Some(Request::Admin(AdminRequest::GetByMac { password, mac} ));
                                                                        } else { log::debug!("Request::from - parsing request: AdminRequest::GetByMac incorrect mac ({})", mac) };
                                                                    } else { log::debug!("Request::from - parsing request: AdminRequest::GetByMac mac not obtained") };
                                                                } else { log::debug!("Request::from - parsing request: AdminRequest::GetByMac mac not obtained") };
                                                            },
                                                            "username" => { // AdminRequest::GetByUsername
                                                                log::debug!("Request::from - parsing request: AdminRequest::GetByUsername how obtained (username)");
                                                                if let Some(username) = request.get("username") {
                                                                    if let Some(username) = username.as_str() {
                                                                        if clients::Client::is_valid_username(username) {
                                                                            log::debug!("Request::from - parsing request: AdminRequest::GetByUsername username obtained ({})", username);
                                                                            if let Some(start_index) = request.get("start_index") {
                                                                                if let Some(start_index) = start_index.as_u64() {
                                                                                    if let Ok(start_index) = usize::try_from(start_index) {
                                                                                        log::debug!("Request::from - parsing request: AdminRequest::GetByUsername start_index obtained ({})", start_index);
                                                                                        let username = String::from(username);
                                                                                        log::debug!("Request::from - parsed request: AdminRequest::GetByUsername");
                                                                                        return Some(Request::Admin(AdminRequest::GetByUsername { password, username, start_index } ))
                                                                                    } else { log::debug!("Request::from - parsing request: AdminRequest::GetByUsername incorrect start_index ({})", start_index); }
                                                                                } else { log::debug!("Request::from - parsing request: AdminRequest::GetByUsername start_index not obtained"); }
                                                                            } else { log::debug!("Request::from - parsing request: AdminRequest::GetByUsername start_index not obtained"); }
                                                                        } else { log::debug!("Request::from - parsing request: AdminRequest::GetByUsername incorrect username ({})", username); }
                                                                    } else { log::debug!("Request::from - parsing request: AdminRequest::GetByUsername username not obtained"); }
                                                                } else { log::debug!("Request::from - parsing request: AdminRequest::GetByUsername username not obtained"); }
                                                            },
                                                            "index" => { // AdminRequest::GetByUsername
                                                                log::debug!("Request::from - parsing request: AdminRequest::GetByIndex how obtained (index)");
                                                                if let Some(start_index) = request.get("start_index") {
                                                                    if let Some(start_index) = start_index.as_u64() {
                                                                        if let Ok(start_index) = usize::try_from(start_index) {
                                                                            if let Some(end_index) = request.get("end_index") {
                                                                                if let Some(end_index) = end_index.as_u64() {
                                                                                    if let Ok(end_index) = usize::try_from(end_index) {
                                                                                        log::debug!("Request::from - parsed request: AdminRequest::GetByIndex");
                                                                                        return Some(Request::Admin(AdminRequest::GetByIndex { password, start_index, end_index } ))
                                                                                    } else { log::debug!("Request::from - parsing request: AdminRequest::GetByIndex incorrect end_index ({})", end_index); }
                                                                                } else { log::debug!("Request::from - parsing request: AdminRequest::GetByIndex end_index not obtained"); }
                                                                            } else { log::debug!("Request::from - parsing request: AdminRequest::GetByIndex end_index not obtained"); }
                                                                        } else { log::debug!("Request::from - parsing request: AdminRequest::GetByIndex incorrect start_index ({})", start_index); }
                                                                    } else { log::debug!("Request::from - parsing request: AdminRequest::GetByIndex start_index not obtained"); }
                                                                } else { log::debug!("Request::from - parsing request: AdminRequest::GetByIndex start_index not obtained"); }
                                                            },
                                                            _ => {
                                                                log::debug!("Request::from - parsing request: AdminRequest::Get incorrect how type ({})", how);
                                                                return None;
                                                            }
                                                        }
                                                    } else { log::debug!("Request::from - parsing request: AdminRequest::Get how not obtained"); }
                                                } else { log::debug!("Request::from - parsing request: AdminRequest::Get how not obtained"); }
                                            } else { // ======================================================= ClientGet
                                                log::debug!("Request::from - parsing request ClientRequest::Get");
                                                if let Some(how) = request.get("how") {
                                                    if let Some(how) = how.as_str() {
                                                        match how.to_lowercase().as_str() {
                                                            "mac" => { // ClientRequest::GetByMac
                                                                log::debug!("Request::from - parsing request: ClientRequest::GetByMac how obtained (mac)");
                                                                if let Some(mac) = request.get("mac") {
                                                                    if let Some(mac) = mac.as_str() {
                                                                        if let Some(mac) = ipparser::MacAddress::new_from_str(mac) {
                                                                            log::debug!("Request::from - parsing request: ClientRequest::GetByMac mac obtained ({})", mac);
                                                                            log::debug!("Request::from - parsed request: ClientRequest::GetByMac");
                                                                            return Some(Request::Client(ClientRequest::GetByMac { password, mac} ));
                                                                        } else { log::debug!("Request::from - parsing request: ClientRequest::GetByMac incorrect mac ({})", mac) };
                                                                    } else { log::debug!("Request::from - parsing request: ClientRequest::GetByMac mac not obtained") };
                                                                } else { log::debug!("Request::from - parsing request: ClientRequest::GetByMac mac not obtained") };
                                                            },
                                                            "username" => { // ClientRequest::GetByUsername
                                                                log::debug!("Request::from - parsing request: ClientRequest::GetByUsername how obtained (username)");
                                                                if let Some(username) = request.get("username") {
                                                                    if let Some(username) = username.as_str() {
                                                                        if clients::Client::is_valid_username(username) {
                                                                            log::debug!("Request::from - parsing request: ClientRequest::GetByUsername username obtained ({})", username);
                                                                            if let Some(start_index) = request.get("start_index") {
                                                                                if let Some(start_index) = start_index.as_u64() {
                                                                                    if let Ok(start_index) = usize::try_from(start_index) {
                                                                                        log::debug!("Request::from - parsing request: ClientRequest::GetByUsername start_index obtained ({})", start_index);
                                                                                        let username = String::from(username);
                                                                                        log::debug!("Request::from - parsed request: ClientRequest::GetByUsername");
                                                                                        return Some(Request::Client(ClientRequest::GetByUsername { password, username, start_index } ));
                                                                                    } else { log::debug!("Request::from - parsing request: ClientRequest::GetByUsername incorrect start_index ({})", start_index); }
                                                                                } else { log::debug!("Request::from - parsing request: ClientRequest::GetByUsername start_index not obtained"); }
                                                                            } else { log::debug!("Request::from - parsing request: ClientRequest::GetByUsername start_index not obtained"); }
                                                                        } else { log::debug!("Request::from - parsing request: ClientRequest::GetByUsername incorrect username ({})", username); }
                                                                    } else { log::debug!("Request::from - parsing request: ClientRequest::GetByUsername username not obtained"); }
                                                                } else { log::debug!("Request::from - parsing request: ClientRequest::GetByUsername username not obtained"); }
                                                            },
                                                            _ => {
                                                                log::debug!("Request::from - parsing request: ClientRequest::Get incorrect how type ({})", how);
                                                                return None;
                                                            }
                                                        } 
                                                    } else { log::debug!("Request::from - parsing request: ClientRequest::Get how not obtained"); }
                                                } else { log::debug!("Request::from - parsing request: ClientRequest::Get how not obtained"); }
                                            }
                                        },
                                        "drop" => {
                                            log::debug!("Request::from - method obtained (drop)");
                                            if user == "admin" { // ======================================================= AdminDrop
                                                if let Some(ip) = request.get("ip") {
                                                    if let Some(ip) = ip.as_str() {
                                                        if let Some(ip) = ipparser::str_to_ipv4addr(ip) {
                                                            log::debug!("Request::from - parsing request: AdminRequest::Drop ip obtained ({})", ip);
                                                            log::debug!("Request::from - parsed request: AdminRequest::Drop");
                                                            return Some(Request::Admin(AdminRequest::Drop { password, ip } ));
                                                        } else { log::debug!("Request::from - parsing request: AdminRequest::Drop incorrect ip ({})", ip); }
                                                    } else { log::debug!("Request::from - parsing request: AdminRequest::Drop ip not obtained"); }
                                                } else { log::debug!("Request::from - parsing request: AdminRequest::Drop ip not obtained"); }
                                            } else { // ======================================================= ClientDrop
                                                if let Some(ip) = request.get("ip") {
                                                    if let Some(ip) = ip.as_str() {
                                                        if let Some(ip) = ipparser::str_to_ipv4addr(ip) {
                                                            log::debug!("Request::from - parsing request: ClientRequest::Drop ip obtained ({})", ip);
                                                            log::debug!("Request::from - parsed request: ClientRequest::Drop");
                                                            return Some(Request::Client(ClientRequest::Drop { password, ip } ));
                                                        } else { log::debug!("Request::from - parsing request: ClientRequest::Drop incorrect ip ({})", ip); }
                                                    } else { log::debug!("Request::from - parsing request: ClientRequest::Drop ip not obtained"); }
                                                } else { log::debug!("Request::from - parsing request: ClientRequest::Drop ip not obtained"); }
                                            }
                                        },
                                        "set" => { // ======================================================= Admin only
                                            log::debug!("Request::from - method obtained (set)");
                                            if user != "admin" { return None; }
                                            log::debug!("Request::from - parsed request: AdminRequest::Set");
                                            if let Some(what) = request.get("what") {
                                                if let Some(what) = what.as_str() {
                                                    match what.to_lowercase().as_str() {
                                                        "key" => {
                                                            if let Some(key) = request.get("key") {
                                                                if let Some(key) = key.as_str(){
                                                                    if server::Server::is_valid_key(key) {
                                                                        log::debug!("Request::from - parsing request: AdminRequest::Set key obtained ({})", key);
                                                                        log::debug!("Request::from - parsed request: AdminRequest::SetKey");
                                                                        return Some(Request::Admin(AdminRequest::SetKey { password, key: String::from(key) } ));
                                                                    } else { log::debug!("Request::from - parsing request: AdminRequest::Set incorrect key ({})", key); }
                                                                } else { log::debug!("Request::from - parsing request: AdminRequest::Set key not obtained"); }
                                                            } else { log::debug!("Request::from - parsing request: AdminRequest::Set key not obtained"); }
                                                        },
                                                        "password" => {
                                                            if let Some(new_password) = request.get("new_password") {
                                                                if let Some(new_password) = new_password.as_str(){
                                                                    if server::Server::is_valid_key(new_password) {
                                                                        log::debug!("Request::from - parsing request: AdminRequest::Set new_password obtained ({})", new_password);
                                                                        log::debug!("Request::from - parsed request: AdminRequest::SetPassword");
                                                                        return Some(Request::Admin(AdminRequest::SetPassword { password, new_password: String::from(new_password) } ));
                                                                    } else { log::debug!("Request::from - parsing request: AdminRequest::Set incorrect new_password ({})", new_password); }
                                                                } else { log::debug!("Request::from - parsing request: AdminRequest::Set new_password not obtained"); }
                                                            } else { log::debug!("Request::from - parsing request: AdminRequest::Set new_password not obtained"); }
                                                        },
                                                        "capacity" => {
                                                            if let Some(capacity) = request.get("capacity") {
                                                                if let Some(capacity) = capacity.as_u64(){
                                                                    if let Ok(capacity) = u16::try_from(capacity) {
                                                                        log::debug!("Request::from - parsing request: AdminRequest::Set capacity obtained ({})", capacity);
                                                                        log::debug!("Request::from - parsed request: AdminRequest::SetCapacity");
                                                                        return Some(Request::Admin(AdminRequest::SetCapacity { password, capacity } ));
                                                                    } else { log::debug!("Request::from - parsing request: AdminRequest::Set incorrect capacity ({})", capacity); }
                                                                } else { log::debug!("Request::from - parsing request: AdminRequest::Set capacity not obtained"); }
                                                            } else { log::debug!("Request::from - parsing request: AdminRequest::Set capacity not obtained"); }
                                                        },
                                                        "list_size" => {},
                                                        "drop_verification" => {},
                                                        _ => {
                                                            log::debug!("Request::from - parsing request: AdminRequest::Set incorrect what type ({})", what);
                                                            return None;
                                                        }
                                                    }
                                                } else { log::debug!("Request::from - parsing request: AdminRequest::Set what not obtained"); }
                                            } else { log::debug!("Request::from - parsing request: AdminRequest::Set what not obtained"); }
                                        }, 
                                        "sign_up" => { // ======================================================= Client only
                                        log::debug!("Request::from - method obtained (sign_up)");
                                            if user != "client" { return None; }
                                            if let Some(username) = request.get("username") {
                                                if let Some(username) = username.as_str() {
                                                    if clients::Client::is_valid_username(username) {
                                                        let username = String::from(username);
                                                        log::debug!("Request::from - parsing request: ClientRequest::SignUp username obtained ({})", username);
                                                        if let Some(mac) = request.get("mac") {
                                                            if let Some(mac) = mac.as_str() {
                                                                if let Some(mac) = ipparser::MacAddress::new_from_str(mac) {
                                                                    log::debug!("Request::from - parsing request: ClientRequest::SignUp mac obtained ({})", mac);
                                                                    if let Some(port) = request.get("port") {
                                                                        if let Some(port) = port.as_u64() {
                                                                            if let Ok(port) = u16::try_from(port) {
                                                                                log::debug!("Request::from - parsing request: ClientRequest::SignUp port obtained ({})", port);
                                                                                if let Some(gobm) = request.get("get_only_by_mac") {
                                                                                    if let Some(get_only_by_mac) = gobm.as_bool() {
                                                                                        log::debug!("Request::from - parsing request: ClientRequest::SignUp get_only_by_mac obtained ({})", get_only_by_mac);
                                                                                        log::debug!("Request::from - parsed request: ClientRequest::SignUp");
                                                                                        return Some(Request::Client(ClientRequest::SignUp { password, username, mac, port, get_only_by_mac } ));
                                                                                    } else { log::debug!("Request::from - parsing request: ClientRequest::SignUp get_only_by_mac not obtained"); }
                                                                                } else { log::debug!("Request::from - parsing request: ClientRequest::SignUp get_only_by_mac not obtained"); }
                                                                            } else { log::debug!("Request::from - parsing request: ClientRequest::SignUp incorrect port ({})", port); }
                                                                        } else { log::debug!("Request::from - parsing request: ClientRequest::SignUp port not obtained"); }
                                                                    } else { log::debug!("Request::from - parsing request: ClientRequest::SignUp port not obtained"); }
                                                                } else { log::debug!("Request::from - parsing request: ClientRequest::SignUp incorrect mac ({})", mac); }
                                                            } else { log::debug!("Request::from - parsing request: ClientRequest::SignUp username not obtained"); }
                                                        } else { log::debug!("Request::from - parsing request: ClientRequest::SignUp mac not obtained"); }
                                                    } else { log::debug!("Request::from - parsing request: ClientRequest::SignUp invalid username ({})", username); }
                                                } else { log::debug!("Request::from - parsing request: ClientRequest::SignUp username not obtained"); }
                                            } else { log::debug!("Request::from - parsing request: ClientRequest::SignUp username not obtained"); }
                                        },
                                        _ => {
                                            log::debug!("Request::from - incorrect method type ({})", method);
                                            return None;
                                        }
                                    }
                                } else { log::debug!("Request::from - method not obtained"); }
                            } else { log::debug!("Request::from - method not obtained"); }
                        } else { log::debug!("Request::from - user not obtained"); }
                    } else { log::debug!("Request::from - user not obtained"); }
                } else { log::debug!("Request::from - password not obtained"); }
            } else { log::debug!("Request::from - password not obtained"); }
        } else { 
            log::debug!("Request::from - failed to serde_json::from_str, request:\n{}", request_str);            
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

#[derive(Clone)]
pub enum AdminRequest {
    GetByIndex {
        password: String,
        start_index: usize,
        end_index: usize
    },
    GetByMac {
        password: String,
        mac: ipparser::MacAddress
    },
    GetByUsername {        
        password: String,        
        username: String,
        start_index: usize
    },
    Drop {
        password: String,
        ip: net::Ipv4Addr
    },
    SetKey {
        password: String,
        key: String
    },
    SetPassword {
        password: String,
        new_password: String
    },
    SetCapacity {
        password: String,
        capacity: u16
    },
    SetListSize {
        password: String,
        list_size: u16
    },
    SetDropVerification {
        password: String,
        drop_verification: bool
    }
}

impl fmt::Display for AdminRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdminRequest::GetByIndex { password: _password, start_index, end_index } => {
                write!(f, "AdminRequest::GetByIndex [{}, {})", start_index, end_index)
            },
            AdminRequest::GetByMac { password: _password, mac } => {
                write!(f, "AdminRequest::GetByMac {}", mac)
            },
            AdminRequest::GetByUsername { password: _password, username, start_index } {
                write!(f, "AdminRequest::GetByUsername \"{}\" starting from {}", username, start_index)
            },
            AdminRequest::Drop { password: _password, ip } => {
                write!(f, "AdminRequest::Drop {}", ip)
            },
            AdminRequest::SetKey { password: _password, key } => {
                write!(f, "AdminRequest::SetKey {}", key)
            },
            AdminRequest::SetPassword { password: _password, new_password } => {
                write!(f, "AdminRequest::SetPassword {}", new_password)
            },
            AdminRequest::SetCapacity { password: _password, capacity } => {
                write!(f, "AdminRequest::SetCapacity {}", capacity)
            },
            AdminRequest::SetListSize { password: _password, list_size } => {
                write!(f, "AdminRequest::SetListSize {}", list_size)
            },
            AdminRequest::SetDropVerification { password: _password, drop_verification } => {
                write!(f, "AdminRequest::SetDropVerification {}", drop_verification)
            }
        }
    }
}

#[derive(Clone)]
pub enum ClientRequest {
    GetByMac {        
        password: String,
        mac: ipparser::MacAddress
    },
    GetByUsername {        
        password: String,        
        username: String,
        start_index: usize
    },
    Drop {
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

impl fmt::Display for ClientRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientRequest::GetByMac { password: _password, mac } => {
                write!(f, "ClientRequest::GetByMac {}", mac)
            },
            ClientRequest::GetByUsername { password: _password, username, start_index } => {
                write!(f, "ClientRequest::GetByUsername \"{}\" starting from {}", username, start_index)
            },
            ClientRequest::Drop { password: _password, ip } => {
                write!(f, "ClientRequest::Drop {}", ip)
            },
            ClientRequest::SignUp { password: _password, username, mac, port, get_only_by_mac } => {
                if *get_only_by_mac {
                    write!(f, "ClientRequest::SignUp {} \"{}\" PORT: {} MAC-ONLY", mac, username, port)
                } else {
                    write!(f, "ClientRequest::SignUp {} \"{}\" PORT: {}", mac, username, port)
                }
            }                        
        }
    }
}
