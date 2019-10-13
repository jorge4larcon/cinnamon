// TODO: to make drop or get the client must be signed up

extern crate log;

use crate::clients;
use crate::ipparser;
use std::fmt;
use std::convert::TryFrom;
use std::net;
use crate::server;

pub enum ReplyErrCodes {
    ClientDoesNotExist,
    UnsupportedListSize,
    ServerCapacityIsFull,
    ServerInternalError,
    WrongPassword,
    OnlyIpv4Supported,
    UnparsableRequest,
    RemoteAdminIsNotAllowed
}

impl fmt::Display for ReplyErrCodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReplyErrCodes::ClientDoesNotExist => write!(f, "{{\"error\":1,\"name\":\"ClientDoesNotExist\"}}"),
            ReplyErrCodes::UnsupportedListSize => write!(f, "{{\"error\":2,\"name\":\"UnsupportedListSize\"}}"),
            ReplyErrCodes::ServerCapacityIsFull => write!(f, "{{\"error\":3,\"name\":\"ServerCapacityIsFull\"}}"),
            ReplyErrCodes::ServerInternalError => write!(f, "{{\"error\":4,\"name\":\"ServerInternalError\"}}"),
            ReplyErrCodes::WrongPassword => write!(f, "{{\"error\":5,\"name\":\"WrongPassword\"}}"),
            ReplyErrCodes::OnlyIpv4Supported => write!(f, "{{\"error\":6,\"name\":\"OnlyIPv4Supported\"}}"),
            ReplyErrCodes::UnparsableRequest => write!(f, "{{\"error\":7,\"name\":\"UnparsableRequest\"}}"),
            ReplyErrCodes::RemoteAdminIsNotAllowed => write!(f, "{{\"error\":8,\"name\":\"RemoteAdminIsNotAllowed\"}}")
        }
    }
}

pub fn reply_admin_drop(ip: &net::Ipv4Addr, clients_map: &mut clients::ClientsMap) -> String {
    let ipv4 = ipparser::ipv4addr_to_u32(ip);
    if clients_map.exists_by_ipv4(ipv4) {
        if clients_map.drop_by_ipv4(ipv4) {
            log::info!("The admin dropped out the client {}", ip);
            return format!("{{\"result\":\"Client was dropped out\"}}");
        } else {            
            log::error!("The admin wanted to drop the client {}, the client exists but was not dropped out", ip);
            return format!("{{\"result\":\"Client was not dropped out\"}}");
        }
    } else {
        log::info!("The client {} doesn't exist but the admin tried to drop it", ip);
        return format!("{}", ReplyErrCodes::ClientDoesNotExist);
    }
}

pub fn reply_admin_setdropvotes(new_dv: u8, server_dv: &mut u8, clients_map: &mut clients::ClientsMap) -> String {
    *server_dv = new_dv;
    let dropped_clients = clients_map.drop_amount(*server_dv);
    let mut list_of_dropped_clients = String::default();
    for (index, (mac, client)) in dropped_clients.iter().enumerate() {
        list_of_dropped_clients.push_str(&format!("[{}] {} {}\n", index, mac, client));
    }
    list_of_dropped_clients.push_str(&format!("{} client(s) were dropped out", list_of_dropped_clients.len()));

    let mut clients_json_array = String::from("[");
    for (mac, client) in dropped_clients {
        clients_json_array.push_str(&client.to_json_string_with_mac(&mac));
        clients_json_array.push(',');
    }
    clients_json_array.push(']');

    log::warn!("The admin set the drop-votes to {}, any client with an equal or greater amount will be dropped out\n{}", server_dv, list_of_dropped_clients);
    format!("{{\"result\":\"The drop-votes has been set to {}, any client with an equal or greater amount has been dropped out\",\"dropped_clients\":{}}}", server_dv, clients_json_array)
}

pub fn reply_admin_setdropverification(new_dv: bool, server_dv: &mut bool) -> String {
    *server_dv = new_dv;
    if *server_dv {
        log::info!("The admin enabled the drop-verification");        
    } else {
        log::info!("The admin disabled the drop-verification");
    }
    format!("{{\"result\":\"The drop-verification has been set to {}\"}}", server_dv)
}

pub fn reply_admin_setlistsize(new_list_size: u16, server_list_size: &mut u16) -> String {
    *server_list_size = new_list_size;

    if *server_list_size == 0 {
        log::warn!("The admin set the list size to {}, no clients will be sended when ClientRequest::GetByUsername", server_list_size);
        format!("{{\"result\":\"The list size has been changed to {}, no clients will be sended when ClientRequest::GetByUsername\"}}", server_list_size)
    } else {
        log::info!("The admin set the list size to {}", server_list_size);
        format!("{{\"result\":\"The list size has been changed to {}\"}}", server_list_size)
    }
}

pub fn reply_admin_setcapacity(new_capacity: u16, server_capacity: &mut u16, clients_map_len: usize) -> String {
    *server_capacity = new_capacity;
    if let Ok(clients_map_len) = u16::try_from(clients_map_len) {
        if new_capacity < clients_map_len {
            log::info!("The admin tried to set the capacity to {}, but there are {} clients signed up in the server, the request was rejected", new_capacity, clients_map_len);
            format!("{{\"result\":\"There are {} clients signed up in the server, fist drop some and then set the capacity\"}}", clients_map_len)
        } else {
            log::info!("The admin set capacity to {}", server_capacity);
            format!("{{\"result\":\"The capacity has been changed to {}\"}}", server_capacity)
        }
    } else {
        log::error!("The admin tried to set the capacity, but there was an internal error casting an u16 to an usize and was rejected");
        format!("{}", ReplyErrCodes::ServerInternalError)
    }
}

pub fn reply_admin_setpassword(new_password: &str, server_password: &mut String) -> String {
    server_password.clear();
    server_password.push_str(new_password);
    log::info!("The admin set the password to {}", server_password);
    format!("{{\"result\":\"The password has been changed to {}\"}}", server_password)
}

pub fn reply_admin_setkey(new_key: &str, server_key: &mut String) -> String {
    server_key.clear();
    server_key.push_str(new_key);
    log::info!("The admin set the key to {}", server_key);
    format!("{{\"result\":\"The key has been changed to {}\"}}", server_key)
}

pub fn reply_admin_getbymac(mac: &ipparser::MacAddress, clients_map: &clients::ClientsMap) -> String {
    if let Some(client) = clients_map.get_by_mac(mac) {
        log::info!("{} was sent to the admin", mac);
        format!("{{\"client\":{}}}", client.to_json_string_with_mac(&mac))
    } else {
        log::info!("{} doesn't exist, but was requested by the admin", mac);
        format!("{}", ReplyErrCodes::ClientDoesNotExist)
    }
}

pub fn reply_admin_getbyusername(username: &str, clients_map: &clients::ClientsMap, list_size: u16, start_index: usize) -> String {
    if let Ok(list_size) = usize::try_from(list_size) {
        let (clients, end_index) = clients_map.usernames_that_contain_with_macs(start_index, list_size, username);
        if !clients.is_empty() { // This ensures that the vector at least contains 1 element
            let clients_len = clients.len();
            let mut json_array = String::from("[");
            for (mac, client) in clients {
                json_array.push_str(client.to_json_string_with_mac(&mac).as_str());
                json_array.push(',');
            }
            json_array.pop(); // So we can act safely
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

pub fn reply_admin_getrunningconfiguration(server: &server::Server) -> String {
    log::info!("The admin asked for the server configuration");
    format!("{{\"config\": \"{}\"}}", server)
}

pub fn reply_admin_getbyindex(start_index: usize, end_index: usize, clients_map: &clients::ClientsMap) -> String {
    let clients_range = clients_map.range(start_index, end_index);
    let list_len = clients_range.len();
    if !clients_range.is_empty() { // This ensures that at least the vector contains 1 element
        let mut clients_json_array = String::from("[");
        for (mac, client) in clients_range.iter() {
            clients_json_array.push_str(&client.to_json_string_with_mac(&mac));
            clients_json_array.push(',');
        }
        clients_json_array.pop(); // So we can act safely
        clients_json_array.push(']');
        log::info!("The admin requested a list of clients by the range [{}, {}) [{} client(s)]", start_index, end_index, list_len);
        return format!("{{\"clients\":{}}}", clients_json_array);
    } else {
        log::info!("The admin requested a list of clients by the range [{}, {}), but there were no clients in that range", start_index, end_index);
        return format!("{{\"clients\":[]}}");
    }
}

pub fn reply_client_getbymac(mac: &ipparser::MacAddress, clients_map: &clients::ClientsMap, guilty: &net::SocketAddrV4) -> String {
    if let Some(client) = clients_map.get_by_mac(mac) {
        log::info!("{} was sent to {}", mac, guilty);
        format!("{{\"client\":{}}}", client.to_json_string_without_drop_votes_get_only_by_mac())
    } else {
        log::info!("{} doesn't exist, but was requested by {}", mac, guilty);
        format!("{}", ReplyErrCodes::ClientDoesNotExist)
    }
}

pub fn reply_client_getbyusername(username: &str, clients_map: &clients::ClientsMap, list_size: u16, start_index: usize, guilty: &net::SocketAddrV4) -> String {
    if let Ok(list_size) = usize::try_from(list_size) {
        let (clients, end_index) = clients_map.usernames_that_contain_get_by_mac_only(start_index, list_size, username);
        if !clients.is_empty() { // This ensures that at least the vector contains 1 element
            let clients_len = clients.len();
            let mut json_array = String::from("[");
            for client in clients {
                json_array.push_str(client.to_json_string_without_drop_votes_get_only_by_mac().as_str());
                json_array.push(',');
            }
            json_array.pop(); // So we can act safely
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

pub fn reply_client_drop(ip: &net::Ipv4Addr, clients_map: &mut clients::ClientsMap, max_drop_votes: u8, guilty: &net::SocketAddrV4) -> String {
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
                    log::info!("New client: {} = {}", mac, client);
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
