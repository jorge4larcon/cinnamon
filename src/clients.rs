extern crate serde_json;
extern crate regex;

use std::collections;
use std::net;
use std::fmt;
use crate::ipparser;
use std::convert::TryFrom;

pub struct Client {
    pub ipv4_addr: u32,
    pub port: u16,
    pub username: String,
    pub get_only_by_mac: bool,
    pub drop_votes: u8
}

impl Client {

    pub fn set_drop_votes(&mut self, dv: u8) {
        self.drop_votes = dv;        
    }

    pub fn add_drop_votes(&mut self, dv: u8) -> u8 {
        self.drop_votes += dv;
        self.drop_votes
    }

    pub fn is_valid_username(username: &str) -> bool {
        if username.is_ascii() {
            let username_regex =regex::Regex::new(r"^[a-zA-Z0-9_-]{3,24}$").unwrap();
            return username_regex.is_match(username);
        }
        false
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
        format!(r#"
            {{
                "ipv4_addr": {}, 
                "port": {},
                "username": "{}",
                "get_only_by_mac": {},
                "drop_votes": {}
            }}
        "#, self.ipv4_addr, self.port, self.username, self.get_only_by_mac, self.drop_votes)
    }

    pub fn to_json_string_without_drop_votes(&self) -> String {
        format!(r#"
            {{
                "ipv4_addr": {}, 
                "port": {},
                "username": "{}",
                "get_only_by_mac": {}
            }}
        "#, self.ipv4_addr, self.port, self.username, self.get_only_by_mac)
    }

    pub fn to_json_string_with_mac_without_drop_votes(&self, mac: &ipparser::MacAddress) -> String {
        format!(r#"
            {{
                "mac": {},
                "ipv4_addr": {}, 
                "port": {},
                "username": "{}",
                "get_only_by_mac": {}
            }}
        "#, mac, self.ipv4_addr, self.port, self.username, self.get_only_by_mac)        
    }

    pub fn to_json_string_with_mac(&self, mac: &ipparser::MacAddress) -> String {
        format!(r#"
            {{
                "mac": {},
                "ipv4_addr": {}, 
                "port": {},
                "username": "{}",
                "get_only_by_mac": {},
                "drop_votes": {}
            }}
        "#, mac, self.ipv4_addr, self.port, self.username, self.get_only_by_mac, self.drop_votes)        
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

pub struct ClientsMap {
    clients: collections::BTreeMap<ipparser::MacAddress, Client>
}

impl ClientsMap {
    
}
