//use crate::auth_service::redis_adapter;
/*
use redis::ConnectionLike;
use redis_test::{MockCmd, MockRedisConnection};

pub struct RedisMockConnectionPool {
    connection: MockRedisConnection,
}

pub fn init_mock(mocks: Vec<MockCmd>) -> RedisMockConnectionPool {
    let connection = MockRedisConnection::new(mocks);
    RedisMockConnectionPool {
        connection: connection,
    } 
}

impl auth_service::redis_adapter::RedisConnectionPool for RedisMockConnectionPool {
    fn get_connection(&mut self) -> Option<Box<dyn ConnectionLike>> {
        Some(Box::new(&self.connection))
    } 
}
*/