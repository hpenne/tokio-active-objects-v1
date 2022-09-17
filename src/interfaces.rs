use async_trait::async_trait;

#[async_trait]
pub trait CommandsA {
    async fn say_hello(&self);
}

#[async_trait]
pub trait EventsA {
    async fn hello_from_a(&self);
}

#[async_trait]
pub trait EventsB {
    async fn hello_from_b(&self);
}
