extern crate clap;

use std::net;
use std::fmt;

pub struct StartConfig {
    pub address: net::SocketAddrV4,    
    pub drop_votes: u8,
    pub password: String,
    pub key: String,
    pub capacity: u16,
    pub get_list_limit: u16,
    pub drop_verification: bool
}

impl fmt::Display for StartConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
"MINT running ...
    => address:            {}
    => drop-votes:         {}
    => password:           {}
    => key:                {}
    => capacity:           {} users
    => list-size:          {}
    => drop-verification:  {}", self.address, self.drop_votes, self.password, self.key, self.capacity, self.get_list_limit, self.drop_verification)
    }
}

impl StartConfig {
    pub fn new(matche
    
    
    
    
    s: clap::ArgMatches) -> Option<StartConfig> {
        let address: net::SocketAddrV4;
        if let Some(matches) = matches.subcommand_matches("start") {
            let addr = matches.value_of("address").unwrap();
        }
        None
    }
}

