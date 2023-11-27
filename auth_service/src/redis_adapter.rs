extern crate redis;
use redis::ConnectionLike;
//use redis_rs::connection::Connection;

pub trait RedisConnectionPool {
    fn get_connection(&mut self) -> Option<Box<dyn ConnectionLike>>;
}

#[derive(Debug)]
pub struct RedisRequestAdapter {
    //address: String,
    connector: redis::Client,
}

impl RedisRequestAdapter {
    pub fn new(address: &String) -> RedisRequestAdapter {
        match redis::Client::open(address.as_str()) {
            Ok(connector) => RedisRequestAdapter {
                //address: address.clone(),
                connector,
            },
            Err(err) => {
                panic!("Error while creating RedisRequestAdapter {:?}", err);
            }
        }
    }
}

impl RedisConnectionPool for RedisRequestAdapter {
    fn get_connection(&mut self) -> Option<Box<dyn ConnectionLike>> {
        // TODO: Impl connection pool..
        match self.connector.get_connection() {
            Ok(conn) => Some(Box::new(conn)),
            Err(err) => {
                eprintln!("{}", err);
                None
            }
        }
    }
}
