pub mod ipparser;
pub mod config;
pub mod clients;
pub mod replies;
pub mod server;


pub enum START_COMMAND_ERROR_CODES {
    UNKNOWN_ERROR
}

pub fn run_start_command(start_config: config::StartConfig) -> START_COMMAND_ERROR_CODES {
    START_COMMAND_ERROR_CODES::UNKNOWN_ERROR
}
