extern crate serde_json;
extern crate regex;

use std::collections;
use std::fmt;
use std::cmp;
use crate::ipparser;
use std::convert::TryFrom;

#[derive(Eq, Clone)]
pub struct Client {
    pub ipv4_addr: u32,
    pub port: u16,
    pub username: String,
    pub get_only_by_mac: bool,
    pub drop_votes: u8
}

impl cmp::Ord for Client {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.ipv4_addr.cmp(&other.ipv4_addr)
    }
}

impl cmp::PartialOrd for Client {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.ipv4_addr == other.ipv4_addr
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.get_only_by_mac {
            write!(f, "{} {} MAC-ONLY PORT: {} DROP-VOTES: {}", self.username, ipparser::u32_to_ipv4(self.ipv4_addr), self.port, self.drop_votes)
        } else {
            write!(f, "{} {} PORT: {} DROP-VOTES: {}", self.username, ipparser::u32_to_ipv4(self.ipv4_addr), self.port, self.drop_votes)
        }
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.get_only_by_mac {
            write!(f, "{} {} MAC-ONLY PORT: {} DROP-VOTES: {}", self.username, ipparser::u32_to_ipv4(self.ipv4_addr), self.port, self.drop_votes)
        } else {
            write!(f, "{} {} PORT: {} DROP-VOTES: {}", self.username, ipparser::u32_to_ipv4(self.ipv4_addr), self.port, self.drop_votes)
        }
    }
}

impl Client {

    pub fn new(ipv4_addr: u32, port: u16, username: &str, get_only_by_mac: bool, drop_votes: u8) -> Option<Client> {
        if Client::is_valid_username(username) {
            return Some(Client { ipv4_addr, port, username: String::from(username), get_only_by_mac, drop_votes });
        }
        None
    }

    pub fn get_ipv4_addr(&self) -> u32 {
        self.ipv4_addr
    }

    pub fn set_drop_votes(&mut self, dv: u8) {
        self.drop_votes = dv;        
    }

    pub fn add_drop_votes(&mut self, dv: u8) -> u8 {
        self.drop_votes += dv;
        self.drop_votes
    }

    pub fn is_valid_username(username: &str) -> bool {
        if username.is_ascii() {
            let username_regex = regex::Regex::new(r"^[a-zA-Z0-9_-]{3,24}$").unwrap();
            return username_regex.is_match(username);
        }
        false
    }

    pub fn username_contains(&self, pattern: &str) -> bool {
        self.username.contains(pattern)
    }

    pub fn username_contains_ignore_case(&self, pattern: &str) -> bool {        
        self.username.to_lowercase().contains(&pattern.to_lowercase())
    }

    pub fn from_json_value_with_no_drop_votes(val: &serde_json::Value) -> Option<Client> {
        // Can be an string
        let username: String;
        if let Some(usrname) = val.get("username") {
            if let Some(usrname) = usrname.as_str() {
                username = String::from(usrname);
            } else { return None; }
        } else { return None; }
        
        // Can be an string ("192.168.1.70") or a number (8974537)
        let ipv4_addr: u32;        
        if let Some(addr) = val.get("ipv4_addr") {
            if let Some(addr) = addr.as_str() {
                if let Some(addr) = ipparser::ipv4_to_u32(addr) {
                    ipv4_addr = addr;
                } else { return None; }
            } else if let Some(addr) = addr.as_u64() {
                if let Ok(addr) = u32::try_from(addr) {
                    ipv4_addr = addr;
                } else { return None; }
            } else { return None; }
        } else { return None; }

        // Can be a number
        let port: u16;
        if let Some(p) = val.get("port") {
            if let Some(p) = p.as_u64() {
                if let Ok(p) = u16::try_from(p) {
                    port = p;
                } else { return None; }
            } else { return None; }
        } else { return None; }

        // Can be a boolean
        let get_only_by_mac: bool;
        if let Some(g) = val.get("get_only_by_mac") {
            if let Some(g) = g.as_bool() {
                get_only_by_mac = g;
            } else { return None; }
        } else { return None; }

        Some(Client { ipv4_addr, port, username, get_only_by_mac, drop_votes: 0 })
    }

    pub fn from_json_value(val: &serde_json::Value) -> Option<Client> {
        // Can be an string
        let username: String;
        if let Some(usrname) = val.get("username") {
            if let Some(usrname) = usrname.as_str() {
                username = String::from(usrname);
            } else { return None; }
        } else { return None; }
        
        // Can be an string ("192.168.1.70") or a number (8974537)
        let ipv4_addr: u32;        
        if let Some(addr) = val.get("ipv4_addr") {
            if let Some(addr) = addr.as_str() {
                if let Some(addr) = ipparser::ipv4_to_u32(addr) {
                    ipv4_addr = addr;
                } else { return None; }
            } else if let Some(addr) = addr.as_u64() {
                if let Ok(addr) = u32::try_from(addr) {
                    ipv4_addr = addr;
                } else { return None; }
            } else { return None; }
        } else { return None; }

        // Can be a number
        let port: u16;
        if let Some(p) = val.get("port") {
            if let Some(p) = p.as_u64() {
                if let Ok(p) = u16::try_from(p) {
                    port = p;
                } else { return None; }
            } else { return None; }
        } else { return None; }

        // Can be a boolean
        let get_only_by_mac: bool;
        if let Some(g) = val.get("get_only_by_mac") {
            if let Some(g) = g.as_bool() {
                get_only_by_mac = g;
            } else { return None; }
        } else { return None; }

        // Can be a number
        let drop_votes: u8;
        if let Some(dv) = val.get("drop_votes") {
            if let Some(dv) = dv.as_u64() {
                if let Ok(dv) = u8::try_from(dv) {
                    drop_votes = dv;
                } else { return None; }
            } else { return None; }
        } else { return None; }

        Some(Client { ipv4_addr, port, username, get_only_by_mac, drop_votes })
    }

    pub fn to_json_string(&self) -> String {
        let ipv4_addr = ipparser::u32_to_ipv4(self.ipv4_addr);
        format!(r#"
            {{
                "ipv4_addr": "{}",
                "port": {},
                "username": "{}",
                "get_only_by_mac": {},
                "drop_votes": {}
            }}
        "#, ipv4_addr, self.port, self.username, self.get_only_by_mac, self.drop_votes)
    }

    pub fn to_json_string_without_drop_votes(&self) -> String {
        let ipv4_addr = ipparser::u32_to_ipv4(self.ipv4_addr);
        format!(r#"
            {{
                "ipv4_addr": "{}", 
                "port": {},
                "username": "{}",
                "get_only_by_mac": {}
            }}
        "#, ipv4_addr, self.port, self.username, self.get_only_by_mac)
    }

    pub fn to_json_string_without_drop_votes_get_only_by_mac(&self) -> String {
        let ipv4_addr = ipparser::u32_to_ipv4(self.ipv4_addr);
        format!(r#"
            {{
                "ipv4_addr": "{}", 
                "port": {},
                "username": "{}"
            }}
        "#, ipv4_addr, self.port, self.username)        
    }

    pub fn to_json_string_with_mac_without_drop_votes(&self, mac: &ipparser::MacAddress) -> String {
        let ipv4_addr = ipparser::u32_to_ipv4(self.ipv4_addr);
        format!(r#"
            {{
                "mac": "{}",
                "ipv4_addr": "{}", 
                "port": {},
                "username": "{}",
                "get_only_by_mac": {}
            }}
        "#, mac, ipv4_addr, self.port, self.username, self.get_only_by_mac)        
    }

    pub fn to_json_string_with_mac(&self, mac: &ipparser::MacAddress) -> String {
        let ipv4_addr = ipparser::u32_to_ipv4(self.ipv4_addr);
        format!(r#"
            {{
                "mac": "{}",
                "ipv4_addr": "{}",
                "port": {},
                "username": "{}",
                "get_only_by_mac": {},
                "drop_votes": {}
            }}
        "#, mac, ipv4_addr, self.port, self.username, self.get_only_by_mac, self.drop_votes)        
    }
}

pub struct ClientsMap {
    clients: collections::BTreeMap<ipparser::MacAddress, Client>
}

impl fmt::Display for ClientsMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.clients.is_empty() {
            write!(f, "This ClientsMap is empty")
        } else {
            let mut clients_map = String::default();
            for (i, (mac, client)) in self.clients.iter().enumerate() {
                clients_map.push_str(&format!("[{}] {} {}\n", i+1, mac, client));
            }
            if self.clients.len() > 1 {
                clients_map.push_str(&format!("{} clients", self.clients.len()));
            } else {
                clients_map.push_str("Just 1 client :/");
            }
            write!(f, "{}", clients_map)
        }
    }  
}

pub enum InsertionType {
    Update,
    Replace { client_mac_replaced: ipparser::MacAddress},
    Insert
}

impl ClientsMap {
    pub fn new() -> ClientsMap {
        ClientsMap { clients: collections::BTreeMap::new() }
    }

    pub fn insert(&mut self, mac: &ipparser::MacAddress, client: &Client) -> InsertionType {
        if let Some(existing_client) = self.clients.get(&mac) { // MAC exists
            if *existing_client == *client { // IPv4 also exists
                // Do nothing... I think I should do an update here, maybe the client changed his name
                self.clients.insert(mac.clone(), client.clone());
                return InsertionType::Update;
            } else { // IPv4 does not exist
                // Do an update
                self.clients.insert(mac.clone(), client.clone());
                return InsertionType::Update;
            }
        } else { // MAC does not exist            
            let repl_mac: ipparser::MacAddress;
            if let Some((mac_key, _client_value)) = self.clients.iter().find(|(_mac_addr, existing_client)| **existing_client == *client) { // IPv4 exists
                // Replace the existing client with the new one
                repl_mac = mac_key.clone(); // ------------- The code continues in `self.clients.remove()`
            } else { // IPv4 neither exists                 
                // Insert the new client
                self.clients.insert(mac.clone(), client.clone());
                return InsertionType::Insert;
            }            
            self.clients.remove(&repl_mac);
            self.clients.insert(mac.clone(), client.clone());
            return InsertionType::Replace { client_mac_replaced: repl_mac };
        }
    }

    pub fn len(&self) -> usize {
        self.clients.len()
    }

    pub fn get_by_mac(&self, mac: &ipparser::MacAddress) -> Option<Client> {
        match self.clients.get(mac) {
            Some(client) => Some(client.clone()),
            None => None
        }
    }

    pub fn exists_by_ipv4(&self, ipv4: u32) -> bool {
        if let Some((_mac_key, _client_value)) = self.clients.iter().find(|(_mac, client)| client.get_ipv4_addr() == ipv4) {
            return true;
        }
        false
    }

    pub fn exists_by_mac(&self, mac: &ipparser::MacAddress) -> bool {
        self.clients.contains_key(mac)
    }

    pub fn range(&self, start_index: usize, end_index: usize) -> Vec<(ipparser::MacAddress, Client)> {
        let mut clients_range: Vec<(ipparser::MacAddress, Client)> = Vec::new();
        if start_index >= self.clients.len() /*|| end_index > self.clients.len() */|| start_index == end_index || start_index > end_index {
            return clients_range;
        }

        let mut new_end_index = end_index;
        if end_index > self.clients.len() {
            new_end_index = self.clients.len();
        }

        for (index, (mac, client)) in self.clients.iter().enumerate() {
            if index >= start_index && index < new_end_index {
                clients_range.push((mac.clone(), client.clone()));
            } else if index >= new_end_index {
                break;
            }
        }
        return clients_range;
    }

    pub fn usernames_that_contain(&self, start_index: usize, size: usize, pattern: &str) -> (Vec<Client>, usize) {
        let mut clients: Vec<Client> = Vec::new();
        for (index, client) in self.clients.values().enumerate() {
            if index >= start_index {
                if clients.len() == size {
                    return (clients, index-1);
                } else if client.username_contains_ignore_case(pattern) {
                    clients.push(client.clone());
                }                
            }
        }
        if self.clients.len() > 0 {
            return (clients, self.clients.len()-1);
        } else {
            return (clients, self.clients.len());
        }
    }

    pub fn usernames_that_contain_with_macs(&self, start_index: usize, size: usize, pattern: &str) -> (Vec<(ipparser::MacAddress, Client)>, usize) {
        let mut clients: Vec<(ipparser::MacAddress, Client)> = Vec::new();
        for (index, (mac, client)) in self.clients.iter().enumerate() {
            if index >= start_index {
                if clients.len() == size {
                    return (clients, index-1);
                } else if client.username_contains_ignore_case(pattern) {
                    clients.push((mac.clone(), client.clone()));
                }
            }
        }
        if self.clients.len() > 0 {
            return (clients, self.clients.len()-1);
        } else {
            return (clients, self.clients.len());
        }
    }

    pub fn usernames_that_contain_get_by_mac_only(&self, start_index: usize, size: usize, pattern: &str) -> (Vec<Client>, usize) {
        let mut clients: Vec<Client> = Vec::new();
        for (index, client) in self.clients.values().enumerate() {
            if index >= start_index {
                if clients.len() == size {
                    return (clients, index);
                } else if client.username_contains_ignore_case(pattern) {
                    if !client.get_only_by_mac {
                        clients.push(client.clone());
                    }                    
                }                
            }
        }
        if self.clients.len() > 0 {
            return (clients, self.clients.len()-1);
        } else {
            return (clients, self.clients.len());
        }
    }

    pub fn drop_vote_by_mac(&mut self, mac: &ipparser::MacAddress, drop_votes: u8, max_drop_votes: u8) -> bool {
        let actual_drop_votes;
        if let Some(client) = self.clients.get_mut(mac) {
            actual_drop_votes = client.add_drop_votes(drop_votes);
        } else { return false; }
        
        if actual_drop_votes >= max_drop_votes {
            self.clients.remove(mac);
            return true;
        }
        false
    }

    pub fn drop_vote_by_ipv4(&mut self, ipv4: u32, drop_votes: u8, max_drop_votes: u8) -> bool {
        let mac;
        if let Some((mac_key, _client_value)) = self.clients.iter().find(|(_mac, client)| client.get_ipv4_addr() == ipv4) {
            mac = mac_key.clone();
        } else {  return false; }
        return self.drop_vote_by_mac(&mac, drop_votes, max_drop_votes);
    }

    pub fn drop_by_ipv4(&mut self, ipv4: u32) -> bool {
        let mac;
        if let Some((mac_k, _client)) = self.clients.iter().find(|(_mac, client)| client.get_ipv4_addr() == ipv4) {
            mac = mac_k.clone();
        } else { return false; }
        
        if let Some(_client) = self.clients.remove(&mac) {
            return true;
        } else {
            log::error!("clients::ClientsMap::drop_by_ipv4: client {} was not removed", mac);
            return false;
        }
    }

    pub fn drop_amount(&mut self, max_drop_votes: u8) -> Vec<(ipparser::MacAddress, Client)> {
        let mut clients: Vec<(ipparser::MacAddress, Client)> = Vec::new();
        for (mac, client) in self.clients.iter() {
            if client.drop_votes >= max_drop_votes {
                clients.push((mac.clone(), client.clone()));       
            }
        }
        for (mac, _client) in clients.iter() {
            if let Some(_c) = self.clients.remove(&mac) {
            } else {
                log::error!("clients::ClientsMap::drop_amount: client {} was not removed", mac);
            }
        }
        clients
    }
}
