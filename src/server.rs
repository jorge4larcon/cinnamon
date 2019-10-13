extern crate log;

use std::io::{
    Read,
    Write
};
use std::net;
use std::process;
use crate::clients;
use crate::config;
use crate::requests;
use crate::replies;
use std::fmt;

pub struct Server {
    pub clients: clients::ClientsMap,
    pub address: net::SocketAddrV4,
    pub key: String,
    pub password: String,
    pub drop_votes: u8,
    pub capacity: u16,
    pub list_size: u16,
    pub drop_verification: bool
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.drop_verification {
            write!(f,"MINT server listening on {}\\n=> key:        {}\\n=> password:   {}\\n=> drop-votes: {}\\n=> list-size:  {}\\n=> capacity:   up to {} user(s)\\n=> drop-verification is enabled\\n=> {} user(s) are logged in", self.address, self.key, self.password, self.drop_votes, self.list_size, self.capacity, self.clients.len())
        } else {
            write!(f,"MINT server listening on {}\\n=> key:        {}\\n=> password:   {}\\n=> drop-votes: {}\\n=> list-size:  {}\\n=> capacity:   up to {} user(s)\\n=> drop-verification is disabled\\n=> {} user(s) are logged in", self.address, self.key, self.password, self.drop_votes, self.list_size, self.capacity, self.clients.len())
        }        
    }
}

impl Server {
    pub fn from_start_config(start_config: &config::StartConfig) -> Server {
        Server {
            clients: clients::ClientsMap::new(),
            address: start_config.address.clone(),
            key: start_config.key.clone(),
            password: start_config.password.clone(),
            drop_votes: start_config.drop_votes,
            capacity: start_config.capacity,
            list_size: start_config.list_size,
            drop_verification: start_config.drop_verification
        }
    }

    pub fn run(&mut self) {
        if let Ok(listener) = net::TcpListener::bind(self.address) {
            log::info!("I'm listening on {}", self.address);
            let mut reply;
            let mut request_type: String = "UnparsedRequest".to_string();
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {                        
                        if let Ok(peer_addr) = stream.peer_addr() {
                            if let net::SocketAddr::V4(peer_addr) = peer_addr {
                                let mut buffer = [0; 1024];
                                if let Ok(bytes_read) = stream.read(&mut buffer) {
                                    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
                                    if let Some(request) = requests::Request::from(&request) {
                                        request_type = request.to_string();
                                        match request {
                                            requests::Request::Admin(a_request) => {
                                                if peer_addr.ip() == self.address.ip() {
                                                    match a_request {
                                                        requests::AdminRequest::Drop { password, ip } => {
                                                            if self.key == password {
                                                                reply = replies::reply_admin_drop(&ip, &mut self.clients);
                                                            } else { 
                                                                log::info!("The admin {} forgot the password", peer_addr);
                                                                reply = replies::ReplyErrCodes::WrongPassword.to_string();
                                                            }
                                                        },
                                                        requests::AdminRequest::GetByIndex { password, start_index, end_index } => {
                                                            if self.key == password {
                                                                reply = replies::reply_admin_getbyindex(start_index, end_index, &self.clients);
                                                            } else { 
                                                                log::info!("The admin {} forgot the password", peer_addr);
                                                                reply = replies::ReplyErrCodes::WrongPassword.to_string();
                                                            }
                                                        },
                                                        requests::AdminRequest::GetByMac { password, mac } => {
                                                            if self.key == password {
                                                                reply = replies::reply_admin_getbymac(&mac, &self.clients);
                                                            } else { 
                                                                log::info!("The admin {} forgot the password", peer_addr);
                                                                reply = replies::ReplyErrCodes::WrongPassword.to_string();
                                                            }
                                                        },
                                                        requests::AdminRequest::GetByUsername { password, username, start_index } => {
                                                            if self.key == password {
                                                                reply = replies::reply_admin_getbyusername(&username, &self.clients, self.list_size, start_index);
                                                            } else { 
                                                                log::info!("The admin {} forgot the password", peer_addr);
                                                                reply = replies::ReplyErrCodes::WrongPassword.to_string();
                                                            }
                                                        },
                                                        requests::AdminRequest::GetRunningConfiguration { password } => {
                                                            if self.key == password {
                                                                reply = replies::reply_admin_getrunningconfiguration(&self);
                                                            } else { 
                                                                log::info!("The admin {} forgot the password", peer_addr);
                                                                reply = replies::ReplyErrCodes::WrongPassword.to_string();
                                                            }
                                                        },
                                                        requests::AdminRequest::SetCapacity { password, capacity } => {
                                                            if self.key == password {
                                                                reply = replies::reply_admin_setcapacity(capacity, &mut self.capacity, self.clients.len());
                                                            } else { 
                                                                log::info!("The admin {} forgot the password", peer_addr);
                                                                reply = replies::ReplyErrCodes::WrongPassword.to_string();
                                                            }
                                                        },
                                                        requests::AdminRequest::SetDropVerification { password, drop_verification } => {
                                                            if self.key == password {
                                                                reply = replies::reply_admin_setdropverification(drop_verification, &mut self.drop_verification);
                                                            } else { 
                                                                log::info!("The admin {} forgot the password", peer_addr);
                                                                reply = replies::ReplyErrCodes::WrongPassword.to_string();
                                                            }
                                                        },
                                                        requests::AdminRequest::SetDropVotes { password, drop_votes } => {
                                                            if self.key == password {
                                                                reply = replies::reply_admin_setdropvotes(drop_votes, &mut self.drop_votes, &mut self.clients);
                                                            } else { 
                                                                log::info!("The admin {} forgot the password", peer_addr);
                                                                reply = replies::ReplyErrCodes::WrongPassword.to_string();
                                                            }
                                                        },
                                                        requests::AdminRequest::SetKey { password, key } => {
                                                            if self.key == password {
                                                                reply = replies::reply_admin_setkey(&key, &mut self.key);
                                                            } else { 
                                                                log::info!("The admin {} forgot the password", peer_addr);
                                                                reply = replies::ReplyErrCodes::WrongPassword.to_string();
                                                            }
                                                        },
                                                        requests::AdminRequest::SetListSize { password, list_size } => {
                                                            if self.key == password {
                                                                reply = replies::reply_admin_setlistsize(list_size, &mut self.list_size);
                                                            } else { 
                                                                log::info!("The admin {} forgot the password", peer_addr);
                                                                reply = replies::ReplyErrCodes::WrongPassword.to_string();
                                                            }
                                                        },
                                                        requests::AdminRequest::SetPassword { password, new_password } => {
                                                            if self.key == password {
                                                                reply = replies::reply_admin_setpassword(&new_password, &mut self.password);
                                                            } else { 
                                                                log::info!("The admin {} forgot the password", peer_addr);
                                                                reply = replies::ReplyErrCodes::WrongPassword.to_string();
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    log::warn!("{} tried to administrate the server", peer_addr);
                                                    reply = replies::ReplyErrCodes::RemoteAdminIsNotAllowed.to_string();
                                                }
                                            },
                                            requests::Request::Client(c_request) => {
                                                match c_request {
                                                    requests::ClientRequest::GetByMac { password: client_password, mac } => {
                                                        if self.password == client_password {
                                                            reply = replies::reply_client_getbymac(&mac, &self.clients, &peer_addr);
                                                        } else { 
                                                            log::info!("The client {} doesn't know the password", peer_addr);
                                                            reply = replies::ReplyErrCodes::WrongPassword.to_string();
                                                        }
                                                    },
                                                    requests::ClientRequest::GetByUsername { password: client_password, username, start_index } => {
                                                        if self.password == client_password {
                                                            reply = replies::reply_client_getbyusername(&username, &self.clients, self.list_size, start_index, &peer_addr);
                                                        } else { 
                                                            log::info!("The client {} doesn't know the password", peer_addr);
                                                            reply = replies::ReplyErrCodes::WrongPassword.to_string();
                                                        }
                                                    },
                                                    requests::ClientRequest::Drop { password: client_password, ip } => {
                                                        if self.password == client_password {
                                                            reply = replies::reply_client_drop(&ip, &mut self.clients, self.drop_votes, &peer_addr);
                                                        } else { 
                                                            log::info!("The client {} doesn't know the password", peer_addr);
                                                            reply = replies::ReplyErrCodes::WrongPassword.to_string();
                                                        }
                                                        log::debug!("Client's DB:\n{}", self.clients);
                                                    },
                                                    requests::ClientRequest::SignUp { password: client_password, username, mac, port, get_only_by_mac } => {
                                                        if self.password == client_password {
                                                            reply = replies::reply_client_signup(&mut self.clients, &username, &mac, &peer_addr.ip(), port, get_only_by_mac, self.capacity);
                                                        } else { 
                                                            log::info!("The client {} doesn't know the password", peer_addr);
                                                            reply = replies::ReplyErrCodes::WrongPassword.to_string();
                                                        }
                                                        log::debug!("Client's DB:\n{}", self.clients);
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        log::info!("I couldn't parse the request of {}", peer_addr);
                                        reply = replies::ReplyErrCodes::UnparsableRequest.to_string();
                                    }
                                } else {
                                    log::error!("I couldn't read the request of {}", peer_addr);
                                    reply = replies::ReplyErrCodes::ServerInternalError.to_string();
                                }
                            } else {
                                log::info!("Host {} tried to use IPv6, but it's not supported", peer_addr);
                                reply = replies::ReplyErrCodes::OnlyIpv4Supported.to_string();
                            }

                            if let Ok(bytes_written) = stream.write(reply.as_bytes()) {
                                if bytes_written == reply.len() {
                                    log::info!("{} from {} Ok!", request_type, peer_addr);
                                } else {
                                    log::error!("{} Err! I sent {} of {} bytes to {}", request_type, bytes_written, reply.as_bytes().len(), peer_addr);
                                }
                            } else {
                                log::error!("{} Err! I couldn't sent the reply to {}", request_type, peer_addr);
                            }
                        } else {
                            log::error!("I couldn't get to peer address of a client");
                        }                        
                    },
                    Err(e) => {
                        log::error!("{}", e);
                    }
                }
            }
        } else {
            log::error!("I couldn't bind to {} :/", self.address);
            process::exit(1);
        }
    }

}

pub fn is_valid_key(key: &str) -> bool {
    key.is_ascii() && key.len() < 33
}
