use deadpool_redis::redis;
use deadpool_redis::redis::{Cmd as RedisCmd, RedisError, Value};
use redis_test::{MockCmd as MockRedisCmd, MockRedisConnection};

#[tokio::test(flavor = "multi_thread")]
async fn hash_is_succeed() {
    let mut conn = MockRedisConnection::new(vec![MockRedisCmd::new(
        RedisCmd::hset("aaa", "bbb", "ccc"),
        Ok(Value::Int(1)),
    )]);

    let query: Result<(), RedisError> = RedisCmd::hset("aaa", "bbb", "ccc")
        .query_async(&mut conn)
        .await;

    assert!(query.is_ok())
}

#[tokio::test(flavor = "multi_thread")]
async fn hash_with_pipe_is_succeed() {
    let mut conn = MockRedisConnection::new(vec![MockRedisCmd::new(
        redis::pipe().hset("aaa", "bbb", "ccc"),
        Ok(Value::Int(0)),
    )]);

    let query: Result<(), RedisError> = RedisCmd::hset("aaa", "bbb", "ccc")
        .query_async(&mut conn)
        .await;

    assert!(query.is_ok())
}

#[tokio::test(flavor = "multi_thread")]
async fn hash_with_pipe_is_succeed2() {
    let mut conn = MockRedisConnection::new(vec![MockRedisCmd::new(
        redis::pipe().hset("aaa", "bbb", "ccc"),
        // Ok(Value::Bulk(vec![Value::Int(0)])),
        Ok(Value::Int(0)),
    )]);

    let query: Result<(), RedisError> = redis::pipe()
        .hset("aaa", "bbb", "ccc")
        .query_async(&mut conn)
        .await;

    assert!(query.is_ok())
}
