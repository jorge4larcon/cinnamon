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

    pub fn run(&mut self) {
        if let Ok(listener) = net::TcpListener::bind(self.address) {
            log::info!("I'm listening on {}", self.address);
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        if let Ok(peer_addr) = stream.peer_addr() {
                            log::info!("New connection with {}", peer_addr);                            
                            let mut buffer = [0; 1024];
                            if let Ok(bytes_read) = stream.read(&mut buffer) {
                                let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);
                                if let Some(request) = requests::Request::from(&request_str) {
                                    log::debug!("I could parse the request");
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
                                            log::debug!("Repliying a client request");
                                            let mut reply: Option<String> = None;
                                            let copy_c_request = c_request.clone();
                                            log::debug!("I could clone the request");
                                            match c_request {
                                                requests::ClientRequest::GetByMac { password: client_password, mac } => {
                                                    log::debug!("It's a ClientRequest::GetByMac");
                                                    if self.password == client_password {
                                                        reply = Some(replies::reply_client_getbymac(&mac, &self.clients));
                                                        log::debug!("I could obtain the reply");
                                                    } else { log::info!("The client {} doesn't know the password", peer_addr); }
                                                },
                                                requests::ClientRequest::GetByUsername { password: client_password, username, start_index } => {
                                                    log::debug!("It's a ClientRequest::GetByUsername");
                                                    if self.password == client_password {
                                                        reply = Some(replies::reply_client_getbyusername(&username, &self.clients, self.list_size, start_index));
                                                        log::debug!("I could obtain the reply");
                                                    } else { log::info!("The client {} doesn't know the password", peer_addr); }
                                                },
                                                requests::ClientRequest::Drop { password: client_password, ip } => {
                                                    log::debug!("It's a ClientRequest::Drop");
                                                    if self.password == client_password {
                                                        reply = Some(replies::reply_client_drop(&ip, &mut self.clients, self.drop_votes));
                                                        log::debug!("I could obtain the reply");
                                                    } else { log::info!("The client {} doesn't know the password", peer_addr); }
                                                    log::info!("Clients database:\n{}", self.clients);
                                                },
                                                requests::ClientRequest::SignUp { password: client_password, username, mac, port, get_only_by_mac } => {
                                                    log::debug!("It's a ClientRequest::SignUp");
                                                    if self.password == client_password {
                                                        log::debug!("The passwords match");
                                                        match peer_addr {
                                                            net::SocketAddr::V4(sock_addr) => {
                                                                log::debug!("I'll try to get the reply...");
                                                                reply = Some(replies::reply_client_signup(&mut self.clients, &username, &mac, &sock_addr.ip(), port, get_only_by_mac, self.capacity));
                                                                log::debug!("I could obtain the reply");
                                                            },
                                                            _ => log::info!("I only support IPv4, client {} doesn't know that", peer_addr)                                                            
                                                        }
                                                    } else { log::info!("The client {} doesn't know the password", peer_addr); }
                                                    log::info!("Clients database:\n{}", self.clients);
                                                }
                                            }

                                            if let Some(reply) = reply {
                                                if let Ok(bytes_written) = stream.write(reply.as_bytes()) {
                                                    if bytes_written == reply.as_bytes().len() {
                                                        log::info!("{} from {} Ok!", copy_c_request, peer_addr);
                                                    } else {
                                                        log::warn!("{} from {} Err! I couldn' write the entire reply", copy_c_request, peer_addr);
                                                    }
                                                } else {
                                                    log::warn!("{} from {} Err! I couldn' write anything", copy_c_request, peer_addr);
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
                                    log::info!("I didn't unserstand the request of {}", peer_addr);
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
