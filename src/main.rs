extern crate clap;

use cinnamon::config;
use cinnamon::ipparser;
use cinnamon::run_start_command;
use std::process;
use clap::{Arg, App, SubCommand, AppSettings};


fn sock_addr_validator(addr: String) -> Result<(), String> {    
    if ipparser::is_socket_addr_v4(addr.as_str()) {
        return Ok(());
    }
    Err(format!("The address must be <address>:<port>"))
}

fn drop_votes_validator(dv: String) -> Result<(), String> {
    if let Ok(_c) = dv.parse::<u8>() {
       return Ok(()) ;
    }
    Err(format!("The drop votes number must be between [1-255]"))
}

fn password_validator(pass: String) -> Result<(), String> {
    if pass.is_ascii() && pass.len() < 33 {
        return Ok(());
    }    
    Err(format!("Password and key must contain only ascii characters and less than 33 characters"))
}

// Correct values go from 1 to ALL
fn gets_list_limit(l: String) -> Result<(), String> {
    if l.is_ascii() {
        if l.to_uppercase() == "ALL" {
            return Ok(());
        } else {
            if let Ok(v) = l.parse::<u16>() {
                if v < 1 {
                    return Err(format!("This value must be between [1-65535]"));
                } else {
                    return Ok(());
                }
            } else {
                return Err(format!("This value must be between [1-65535]"));
            }
        }
    }
    return Err(format!("This value must be between [1-4294967295]"));
}

fn capacity_validator(c: String) -> Result<(), String> {
    if c.is_ascii() {
        let c = c.to_uppercase();
        if c == "MAX" || c == "MIN" {
            return Ok(());
        } else {
            if let Ok(v) = c.parse::<u16>() {
                if v < 2 {
                    return Err(format!("This value must be between [2-65535]"));
                } else {
                    return Ok(());
                }
            } else {
                return Err(format!("This value must be between [2-65535]"));
            }
        }
    }
    return Err(format!("This value must be between [2-65535]"));
}

fn main() {
    let matches = App::new("MINT Server")
                          .version("1.0")
                          .author("Jorge A. <jorge4larcon@gmail.com>")
                          .about("Connecting clients between LAN")
                          .setting(AppSettings::ArgRequiredElseHelp)
                          .subcommand(SubCommand::with_name("start")
                                       .about("Start the server")
                                       .version("1.0")
                                       .author("Jorge A. <jorge4larcon@gmail.com>")
                                       .arg(Arg::with_name("address")
                                            .short("a")
                                            .long("address")
                                            .value_name("IP_ADDRESS:PORT")
                                            .help("Sets the IP address and port the server will listen to")
                                            .default_value("127.0.0.1:42000")
                                            .takes_value(true)
                                            .required(false)
                                            .number_of_values(1)
                                            .validator(sock_addr_validator))                                        
                                        .arg(Arg::with_name("key")
                                            .short("k")
                                            .long("key")
                                            .value_name("KEY")
                                            .help("Sets the admin's password")
                                            .default_value("admin_secret")
                                            .takes_value(true)
                                            .required(false)
                                            .number_of_values(1)
                                            .validator(password_validator))
                                        .arg(Arg::with_name("drop-votes")
                                            .short("d")
                                            .long("drop-votes")
                                            .value_name("DROP_VOTES")
                                            .help("Sets the number of votes a user must have to be droped from the server")
                                            .default_value("2")
                                            .takes_value(true)
                                            .required(false)
                                            .number_of_values(1)
                                            .validator(drop_votes_validator))
                                        .arg(Arg::with_name("password")
                                            .short("p")
                                            .long("password")
                                            .value_name("PASSWORD")
                                            .help("Sets the password that users must provide in order to register on the server")
                                            .default_value("secret")
                                            .takes_value(true)
                                            .required(false)
                                            .number_of_values(1)
                                            .validator(password_validator))                                        
                                        .arg(Arg::with_name("drop-verification")
                                            .short("D")
                                            .long("drop-verification")
                                            .long_help("When enabled, the server tries to connect to the client that is going to be dropped, if the connection fails the client is dropped, else restarts its drop votes to zero")
                                            .multiple(false)
                                            .required(false))
                                        .arg(Arg::with_name("log-level")
                                            .short("L")
                                            .long("log-level")
                                            .value_name("LOG LEVEL")
                                            .help("Sets the logging level")
                                            .possible_values(&["error", "warning", "info", "debug"])
                                            .default_value("info")
                                            .takes_value(true)
                                            .number_of_values(1)
                                            .required(false))
                                        .arg(Arg::with_name("list-size")
                                            .short("l")
                                            .long("list-size")
                                            .value_name("GET'S LIST SIZE")
                                            .help("Sets how many users the server will send to a GET request")
                                            .default_value("5")
                                            .takes_value(true)
                                            .required(false)
                                            .number_of_values(1)
                                            .validator(gets_list_limit))
                                        .arg(Arg::with_name("capacity")
                                            .short("c")
                                            .long("capacity")
                                            .value_name("CAPACITY")
                                            .help("Sets how many users the server can hold")
                                            .default_value("1024")
                                            .takes_value(true)
                                            .required(false)
                                            .number_of_values(1)                                            
                                            .validator(capacity_validator)))
                          .get_matches();

    let start_command_config: config::StartConfig;
    match config::StartConfig::new(matches) {
        Some(config) => start_command_config = config,
        None => {
            eprintln!("WTF? I didn't understand your command :/");
            process::exit(1);
        }
    }

    if let Err(_) = config::setup_logging(&start_command_config.log_level) {
        eprintln!("Failed to set up logging, maybe there's one logger already...")
    }

    run_start_command(start_command_config);
}
