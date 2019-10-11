extern crate log;

use crate::clients;
use crate::ipparser;
use std::fmt;
use std::convert::TryFrom;
use std::net;

pub enum ReplyErrCodes {
    ClientDoesNotExist,
    UnsupportedListSize,
    ServerCapacityIsFull,
    ServerInternalError
}

impl fmt::Display for ReplyErrCodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReplyErrCodes::ClientDoesNotExist => write!(f, "{{\"err_code\":1,\"name\":\"ClientDoesNotExist\"}}"),
            ReplyErrCodes::UnsupportedListSize => write!(f, "{{\"err_code\":2,\"name\":\"UnsupportedListSize\"}}"),
            ReplyErrCodes::ServerCapacityIsFull => write!(f, "{{\"err_code\":3,\"name\":\"ServerCapacityIsFull\"}}"),
            ReplyErrCodes::ServerInternalError => write!(f, "{{\"err_code\":4,\"name\":\"ServerInternalError\"}}")
        }
    }
}

pub fn reply_client_getbymac(mac: &ipparser::MacAddress, clients_map: &clients::ClientsMap) -> String {
    if let Some(client) = clients_map.get_by_mac(mac) {
        format!("{{\"client\":{}}}", client.to_json_string_without_drop_votes())
    } else {
        format!("{}", ReplyErrCodes::ClientDoesNotExist)
    }
}

pub fn reply_client_getbyusername(username: &str, clients_map: &clients::ClientsMap, list_size: u16, start_index: usize) -> String {
    if let Ok(list_size) = usize::try_from(list_size) {
        let (clients, end_index) = clients_map.usernames_that_contain_get_by_mac_only(start_index, list_size, username);
        if !clients.is_empty() {
            let mut json_array = String::from("[");
            for client in clients {
                json_array.push_str(client.to_json_string_without_drop_votes().as_str());
                json_array.push(',');
            }
            json_array.push_str("]");
            return format!("{{\"clients\":{},\"end_index\":{}}}", json_array, end_index);
        } else {
            return format!("{}", ReplyErrCodes::ClientDoesNotExist);
        }
    } else {
        return format!("{}", ReplyErrCodes::UnsupportedListSize);
    }
}

pub fn reply_client_drop(ip: &net::Ipv4Addr, clients_map: &mut clients::ClientsMap, max_drop_votes: u8) -> String {
    let ipv4 = ipparser::ipv4addr_to_u32(ip);
    if clients_map.exists_by_ipv4(ipv4) {
        if clients_map.drop_vote_by_ipv4(ipv4, 1, max_drop_votes) {
            log::info!("Client {} was dropped out", ip);
            return format!("{{\"result\":\"Client was dropped out\"}}");
        } else {
            return format!("{{\"result\":\"Client was not dropped out\"}}");
        }
    } else {
        return format!("{}", ReplyErrCodes::ClientDoesNotExist);
    }
}

pub fn reply_client_signup(clients_map: &mut clients::ClientsMap, username: &str, mac: &ipparser::MacAddress, ip: &net::Ipv4Addr, port: u16, get_only_by_mac: bool, capaciy: u16) -> String {
    let client = clients::Client { ipv4_addr: ipparser::ipv4addr_to_u32(ip), port, username: String::from(username), get_only_by_mac, drop_votes: 0 };    
    if let Ok(capaciy) = usize::try_from(capaciy) {        
        if clients_map.len() < capaciy {
            match clients_map.insert(mac, &client) {
                clients::InsertionType::Insert => {
                    log::info!("We have a new client:\n{} = {}", mac, client);
                    return format!("{{\"result\": \"You have been registered\"}}");
                },
                clients::InsertionType::Update => {
                    log::info!("The client {} has been updated", mac);
                    return format!("{{\"result\": \"Your data has been updated\"}}");
                },
                clients::InsertionType::Replace { client_mac_replaced } => {
                    log::info!("The client {} was replaced by {} {}", client_mac_replaced, mac, ip);
                    return format!("{{\"result\": \"You have been registered\"}}");
                }
            }
        } else {
            return format!("{}", ReplyErrCodes::ServerCapacityIsFull);
        }
    } else {
        return format!("{}", ReplyErrCodes::ServerInternalError);
    }
}
