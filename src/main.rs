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
    let a_cmds = a.commands_a();
    a.connect_to_b(b.clone());
    b.connect_to_a(a.events_b());
    a_cmds.say_hello();
    println!("Spawned initial");

    tokio::time::sleep(time::Duration::from_millis(500)).await;
    // std::thread::sleep(time::Duration::from_millis(500));
}
