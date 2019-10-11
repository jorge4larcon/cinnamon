// TODO: to make drop or get the client must be signed up

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

pub fn reply_admin_getbymac(mac: &ipparser::MacAddress, clients_map: &clients::ClientsMap, _guilty: &net::SocketAddr) -> String {
    if let Some(client) = clients_map.get_by_mac(mac) {
        log::info!("{} was sent to the admin", mac);
        format!("{{\"client\":{}}}", client.to_json_string_without_drop_votes_get_only_by_mac())
    } else {
        log::info!("{} doesn't exist, but was requested by the admin", mac);
        format!("{}", ReplyErrCodes::ClientDoesNotExist)
    }
}

pub fn reply_admin_getbyusername(username: &str, clients_map: &clients::ClientsMap, list_size: u16, start_index: usize, _guilty: &net::SocketAddr) -> String {
    if let Ok(list_size) = usize::try_from(list_size) {
        let (clients, end_index) = clients_map.usernames_that_contain_get_by_mac_only(start_index, list_size, username);
        if !clients.is_empty() {
            let clients_len = clients.len();
            let mut json_array = String::from("[");
            for client in clients {
                json_array.push_str(client.to_json_string_without_drop_votes_get_only_by_mac().as_str());
                json_array.push(',');
            }
            json_array.push_str("]");
            log::info!("{} client(s) named like \"{}\" were sent to the admin", clients_len, username);
            return format!("{{\"clients\":{},\"end_index\":{}}}", json_array, end_index);
        } else {
            log::info!("No clients named like \"{}\" were sent to the admin", username);
            return format!("{}", ReplyErrCodes::ClientDoesNotExist);
        }        
    } else {
        log::error!("The admin tried to get a list of usernames, but there was an internal error casting an u16 to an usize and was rejected");
        return format!("{}", ReplyErrCodes::UnsupportedListSize);
    }
}

pub fn reply_admin_getbyindex(start_index: usize, end_index: usize, clients_map: &clients::ClientsMap, _guilty: &net::SocketAddr) -> String {
    let clients_range = clients_map.range(start_index, end_index);
    let list_len = clients_range.len();
    if !clients_range.is_empty() {
        let mut clients_json_array = String::from("[");
        for (mac, client) in clients_range {
            clients_json_array.push_str(&client.to_json_string_with_mac(&mac));
            clients_json_array.push(',');
        }
        clients_json_array.push(']');
        log::info!("The admin requested a list of clients by the range [{}, {}) [{} client(s)]", start_index, end_index, list_len);
        return format!("{{\"clients\":{}}}", clients_json_array);
    } else {
        log::info!("The admin requested a list of clients by the range [{}, {}), but there were not clients in that range", start_index, end_index);
        return format!("{{\"clients\":[]}}");
    }
}

pub fn reply_client_getbymac(mac: &ipparser::MacAddress, clients_map: &clients::ClientsMap, guilty: &net::SocketAddr) -> String {
    if let Some(client) = clients_map.get_by_mac(mac) {
        log::info!("{} was sent to {}", mac, guilty);
        format!("{{\"client\":{}}}", client.to_json_string_without_drop_votes_get_only_by_mac())
    } else {
        log::info!("{} doesn't exist, but was requested by {}", mac, guilty);
        format!("{}", ReplyErrCodes::ClientDoesNotExist)
    }
}

pub fn reply_client_getbyusername(username: &str, clients_map: &clients::ClientsMap, list_size: u16, start_index: usize, guilty: &net::SocketAddr) -> String {
    if let Ok(list_size) = usize::try_from(list_size) {
        let (clients, end_index) = clients_map.usernames_that_contain_get_by_mac_only(start_index, list_size, username);
        if !clients.is_empty() {
            let clients_len = clients.len();
            let mut json_array = String::from("[");
            for client in clients {
                json_array.push_str(client.to_json_string_without_drop_votes_get_only_by_mac().as_str());
                json_array.push(',');
            }
            json_array.push_str("]");
            log::info!("{} client(s) named like \"{}\" were sent to {}", clients_len, username, guilty);
            return format!("{{\"clients\":{},\"end_index\":{}}}", json_array, end_index);
        } else {
            log::info!("No clients named like \"{}\" were sent to {}", username, guilty);
            return format!("{}", ReplyErrCodes::ClientDoesNotExist);
        }        
    } else {
        log::error!("The client {} tried to get a list of usernames, but there was an internal error casting an u16 to an usize and was rejected", guilty);
        return format!("{}", ReplyErrCodes::UnsupportedListSize);
    }
}

pub fn reply_client_drop(ip: &net::Ipv4Addr, clients_map: &mut clients::ClientsMap, max_drop_votes: u8, guilty: &net::SocketAddr) -> String {
    let ipv4 = ipparser::ipv4addr_to_u32(ip);
    if clients_map.exists_by_ipv4(ipv4) {
        if clients_map.drop_vote_by_ipv4(ipv4, 1, max_drop_votes) {
            log::info!("The client {} was dropped out by {}", ip, guilty);
            return format!("{{\"result\":\"Client was dropped out\"}}");
        } else {            
            log::info!("The client {} tried to drop out {}", guilty, ip);
            return format!("{{\"result\":\"Client was not dropped out\"}}");
        }
    } else {
        log::info!("The client {} doesn't exist but {} tried to drop it", ip, guilty);
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
            log::info!("Server capacity ({} clients) if full, client {} was rejected", capaciy, ip);
            return format!("{}", ReplyErrCodes::ServerCapacityIsFull);
        }
    } else {
        log::error!("The client {} tried to sign in, but there was an internal error casting an u16 to an usize and was rejected", ip);
        return format!("{}", ReplyErrCodes::ServerInternalError);
    }
}
