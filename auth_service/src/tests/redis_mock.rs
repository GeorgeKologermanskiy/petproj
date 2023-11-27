use redis::ConnectionLike;
use redis_test::{MockCmd, MockRedisConnection};

use crate::redis_adapter::RedisConnectionPool;

pub struct RedisMockConnectionPool {
    connection: MockRedisConnection,
}

pub fn init_mock(mocks: Vec<MockCmd>) -> RedisMockConnectionPool {
    let connection = MockRedisConnection::new(mocks);
    RedisMockConnectionPool { connection }
}

impl RedisConnectionPool for RedisMockConnectionPool {
    fn get_connection(&mut self) -> Option<Box<dyn ConnectionLike>> {
        Some(Box::new(self.connection.clone()))
    }
}
