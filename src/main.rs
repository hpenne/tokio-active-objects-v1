mod comp_a;
mod comp_b;
mod interfaces;

use crate::comp_b::CompB;
use comp_a::CompA;
use interfaces::CommandsA;
use std::sync::Arc;
use std::time;

#[tokio::main]
async fn main() {
    let a = Arc::new(CompA::new());
    let b = Arc::new(CompB::new());
    a.connect_to_b(b.clone()).await;
    b.connect_to_a(a.clone()).await;
    a.say_hello().await;
    println!("Spawned initial");

    tokio::time::sleep(time::Duration::from_millis(500)).await;
    // std::thread::sleep(time::Duration::from_millis(500));
}
