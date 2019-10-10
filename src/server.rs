extern crate log;

use std::io::{
    Read,
    Write
};
use std::net;
use std::process;
use crate::clients;
use crate::config;

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
                    Ok(stream) => {
                        if let Ok(peer_addr) = stream.peer_addr() {
                            log::info!("New connection with {}", peer_addr);
                            handle_client(stream, peer_addr);
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

pub fn handle_client(mut stream: net::TcpStream, peer_addr: net::SocketAddr) {
    let mut buffer = [0; 1024];
    if let Ok(bytes_read) = stream.read(&mut buffer) {
        let _request = String::from_utf8_lossy(&buffer[..bytes_read]);

    } else {
        log::error!("I couldn't read the message of {} :/", peer_addr.ip());
    }

    if let Ok(()) = stream.shutdown(net::Shutdown::Both) {        
    } else { log::error!("I couldn't shutdown the connection with {}", peer_addr.ip()) }
}