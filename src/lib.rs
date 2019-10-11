extern crate log;

pub mod ipparser;
pub mod config;
pub mod clients;
pub mod requests;
pub mod replies;
pub mod server;

#[cfg(test)]
mod tests;


pub fn run_start_command(start_config: config::StartConfig) {
    log::info!("{}", start_config);
    let mut server = server::Server::from_start_config(&start_config);
    server.run();
}
