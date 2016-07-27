extern crate mysql;

use std::net::TcpStream;
use std::sync::{Arc, Mutex};

pub struct Client<'a>
{
    pub stream: &'a mut TcpStream,
    pub conn: Arc< Mutex<mysql::Pool> >,
    pub client_id: u8
}
