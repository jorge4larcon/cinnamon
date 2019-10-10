use std::fmt;

pub enum Reply {
    Admin(AdminReply),
    Client(ClientReply)
}

pub enum ReplyResult {
    OK,
    ERR
}

impl fmt::Display for ReplyResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReplyResult::OK => write!(f, "Ok"),
            ReplyResult::ERR => write!(f, "Err")
        }
    }
}

pub enum ClientReply {
    GetByMac {}
}

pub enum AdminReply {
}
