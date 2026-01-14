pub mod gossip;

pub enum Request {
    Get { key: Vec<u8> },
    Set { key: Vec<u8>, value: Vec<u8> },
}

pub enum Response {
    Ok { value: Option<Vec<u8>> },
    Error { code: u16, msg: String },
}
