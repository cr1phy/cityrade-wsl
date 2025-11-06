use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Encode, Decode, derive_more::Display)]
pub enum Event {
    #[display("connect")]
    Connect,
    #[display("auth_login<login={login},password={password:?}>")]
    AuthLogin { login: String, password: Vec<u8> },
}
