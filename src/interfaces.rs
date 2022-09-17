use async_trait::async_trait;

#[async_trait]
pub trait CommandsA {
    fn say_hello(&self);
}

#[async_trait]
pub trait EventsA {
    fn hello_from_a(&self);
}

#[async_trait]
pub trait EventsB {
    fn hello_from_b(&self);
}
