extern crate clap;
extern crate fern;
extern crate chrono;
extern crate log;

use std::net;
use std::fmt;
use crate::ipparser;

pub struct StartConfig {
    pub address: net::SocketAddrV4,    
    pub drop_votes: u8,
    pub password: String,
    pub key: String,
    pub capacity: u16,
    pub list_size: u16,
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
    => drop-verification:  {}", self.address, self.drop_votes, self.password, self.key, self.capacity, self.list_size, self.drop_verification)
    }
}

impl StartConfig {
    pub fn new(matches: clap::ArgMatches) -> Option<StartConfig> {
        println!("Here 1");
        if let Some(matches) = matches.subcommand_matches("start") {            
            let address: net::SocketAddrV4;
            if let Some(addr) = matches.value_of("address") {
                if let Some(addr) = ipparser::sockaddrv4str_to_sockaddrv4(addr) {
                    address = addr;
                } else { return None; }
            } else { return None; }
            println!("Here 2");

            let drop_votes: u8;
            if let Some(dv) = matches.value_of("drop-votes") {
                println!("Here 2.1");
                if let Ok(dv) = dv.parse::<u8>() {
                    drop_votes = dv;
                } else { return None; }
            } else { return None; }
            println!("Here 3");

            let password: String;
            if let Some(pass) = matches.value_of("password") {
                password = String::from(pass);
            } else { return None; }
            println!("Here 4");

            let key: String;
            if let Some(k) = matches.value_of("key") {
                key = String::from(k);
            } else { return None; }
            println!("Here 5");

            let capacity: u16;
            if let Some(c) = matches.value_of("capacity") {
                if let Ok(c) = c.parse::<u16>() {
                    capacity = c;
                }  else { return None; }
            } else { return None; }
            println!("Here 6");
            
            let list_size: u16;
            if let Some(ls) = matches.value_of("list-size") {
                if let Ok(ls) = ls.parse::<u16>() {
                    list_size = ls;
                } else { return None; }
            } else { return None; }
            println!("Here 7");

            let drop_verification = matches.is_present("drop-verification");
            println!("Here 8");

            return Some( StartConfig { address, drop_votes, password, key, capacity, list_size, drop_verification } );
        }
        None
    }
}

pub fn setup_logging() -> Result<(), fern::InitError> {
    let colors = fern::colors::ColoredLevelConfig::new().info(fern::colors::Color::Green)
                                                        .warn(fern::colors::Color::Yellow)
                                                        .error(fern::colors::Color::Red)
                                                        .debug(fern::colors::Color::Blue)
                                                        .trace(fern::colors::Color::Magenta);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())        
        .apply()?;        
    Ok(())
}
