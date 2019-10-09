extern crate clap;

use crate::ipparser;
use std::collections;
use std::net;
use std::fmt;

pub struct StartConfig {
    pub address: net::SocketAddrV4,
    pub managers: collections::BTreeSet<net::IpAddr>,
    pub drop_votes: u8,
    pub password: String,
    pub key: String,
    pub capacity: u16,
    pub get_list_limit: u16,    
    pub drop_verification: bool,
    pub mac_sharing: bool    
}

impl fmt::Display for StartConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
"Address:               {}
Drop votes:            {}
Password:              {}
Key:                   {}
Managers:              {:?}
Capacity:              {} users
List limit:            {}
Mac sharing:           {}
Drop verification:     {}", self.address, self.drop_votes, self.password, self.key, self.managers, self.capacity, self.get_list_limit, self.mac_sharing, self.drop_verification)
    }
}

impl StartConfig {
    pub fn new(matches: clap::ArgMatches) -> Option<StartConfig> {
        // if let Some(matches) = matches.subcommand_matches("start") {
        //     let address = ipparser::AddrContainer::new()
        // }
        return None;
    }
}

