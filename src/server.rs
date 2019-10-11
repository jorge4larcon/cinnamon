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

    pub fn is_valid_key(key: &str) -> bool {
        key.is_ascii() && key.len() < 33
    }

    pub fn run(&mut self) {
        if let Ok(listener) = net::TcpListener::bind(self.address) {
            log::info!("I'm listening on {}", self.address);
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        if let Ok(peer_addr) = stream.peer_addr() {
                            log::debug!("New connection from {}", peer_addr);
                            let mut buffer = [0; 1024];
                            if let Ok(bytes_read) = stream.read(&mut buffer) {
                                let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);
                                if let Some(request) = requests::Request::from(&request_str) {
                                    let request_type: &str;
                                    match request {
                                        requests::Request::Admin(a_request) => {
                                            match a_request {
                                                // requests::AdminRequest::Get => {},
                                                // requests::AdminRequest::Set => {},
                                                // requests::AdminRequest::Drop => {},
                                                _ => {
                                                    let message = b"Only clients are supported";
                                                    if let Ok(bytes_written) = stream.write(message) {
                                                        if bytes_written == message.len() {
                                                            log::info!("Failure message sent to {}", peer_addr);
                                                        } else {
                                                            log::warn!("I could not write the entire failure message to client {}", peer_addr);
                                                        }
                                                    } else {
                                                        log::warn!("I couln't write to client {}", peer_addr);
                                                    }
                                                }
                                            }
                                        },
                                        requests::Request::Client(c_request) => {                                            
                                            let mut reply: Option<String> = None;
                                            match c_request {
                                                requests::ClientRequest::GetByMac { password: client_password, mac } => {
                                                    request_type = "Client::GetByMac";
                                                    if self.password == client_password {
                                                        reply = Some(replies::reply_client_getbymac(&mac, &self.clients, &peer_addr));
                                                    } else { log::info!("The client {} doesn't know the password", peer_addr); }
                                                },
                                                requests::ClientRequest::GetByUsername { password: client_password, username, start_index } => {
                                                    request_type = "Client::GetByUsername";
                                                    if self.password == client_password {
                                                        reply = Some(replies::reply_client_getbyusername(&username, &self.clients, self.list_size, start_index, &peer_addr));
                                                    } else { log::info!("The client {} doesn't know the password", peer_addr); }
                                                },
                                                requests::ClientRequest::Drop { password: client_password, ip } => {
                                                    request_type = "Client::Drop";
                                                    if self.password == client_password {
                                                        reply = Some(replies::reply_client_drop(&ip, &mut self.clients, self.drop_votes, &peer_addr));
                                                    } else { log::info!("The client {} doesn't know the password", peer_addr); }
                                                    log::debug!("Client's DB:\n{}", self.clients);
                                                },
                                                requests::ClientRequest::SignUp { password: client_password, username, mac, port, get_only_by_mac } => {
                                                    request_type = "Client::SignUp";
                                                    if self.password == client_password {
                                                        match peer_addr {
                                                            net::SocketAddr::V4(sock_addr) => {
                                                                reply = Some(replies::reply_client_signup(&mut self.clients, &username, &mac, &sock_addr.ip(), port, get_only_by_mac, self.capacity));
                                                            },
                                                            _ => log::info!("I only support IPv4, client {} doesn't know that", peer_addr)
                                                        }
                                                    } else { log::info!("The client {} doesn't know the password", peer_addr); }
                                                    log::debug!("Client's DB:\n{}", self.clients);
                                                }
                                            }

                                            if let Some(reply) = reply {
                                                if let Ok(bytes_written) = stream.write(reply.as_bytes()) {
                                                    if bytes_written == reply.as_bytes().len() {
                                                        log::info!("{} from {} Ok!", request_type, peer_addr);
                                                    } else {
                                                        log::warn!("{} from {} Err! I couldn' write the entire reply", request_type, peer_addr);
                                                    }
                                                } else {
                                                    log::warn!("{} from {} Err! I couldn' write anything", request_type, peer_addr);
                                                }                                                
                                            } else {
                                                let message = b"Only IPv4 is supported";
                                                if let Ok(bytes_written) = stream.write(message) {
                                                    if bytes_written == message.len() {
                                                        log::info!("Failure message sent to {}", peer_addr);
                                                    } else {
                                                        log::warn!("I could not write the entire failure message to client {}", peer_addr);
                                                    }
                                                } else {
                                                    log::warn!("I couln't write to client {}", peer_addr);
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    log::info!("I didn't understand the request of {}", peer_addr);
                                    let message = b"I don't understand you";
                                    if let Ok(bytes_written) = stream.write(message) {
                                        if bytes_written == message.len() {
                                            log::info!("Failure message sent to {}", peer_addr);
                                        } else {
                                            log::warn!("I could not write the entire failure message to client {}", peer_addr);
                                        }
                                    } else {
                                        log::warn!("I couln't write to client {}", peer_addr);
                                    }
                                }
                            } else {
                                log::error!("I couldn't read the message of {} :/", peer_addr);
                            }
                            if let Ok(()) = stream.shutdown(net::Shutdown::Both) {        
                            } else { log::error!("I couldn't shutdown the connection with {}", peer_addr) }
                        } else {
                            log::error!("I couldn't get to peer address of a client :/");
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
