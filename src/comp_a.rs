use crate::interfaces::EventsA;
use crate::interfaces::{CommandsA, EventsB};
use crate::CompB;
use async_trait::async_trait;
use std::sync::Arc;
use std::sync::Weak;
use tokio::sync::Mutex;

pub struct CompA {
    i: Arc<Mutex<CompAImpl>>,
}

impl CompA {
    pub fn new() -> Self {
        Self {
            i: Arc::new(Mutex::new(CompAImpl::new())),
        }
    }

    pub fn connect_to_b(&self, b: Arc<CompB>) {
        let i = self.i.clone();
        tokio::spawn(async move {
            let mut inner = i.lock().await;
            println!("Connecting to b");
            inner.connect_to_b(Arc::downgrade(&b));
        });
    }
}

struct CompAImpl {
    b: Weak<CompB>,
}

impl CompAImpl {
    pub fn new() -> Self {
        Self {
            b: Weak::<CompB>::new(),
        }
    }

    pub fn connect_to_b(&mut self, b: Weak<CompB>) {
        self.b = b;
        println!("Connected to b");
    }
}

#[async_trait]
impl CommandsA for CompA {
    fn say_hello(&self) {
        let i = self.i.clone();

        // Using spawn here is problematic, because spawn creates a new task, and tasks
        // can be executed in arbitrary order, so say_hello will sometimes be executed before
        // connect_to_b, in which case hello fails.
        tokio::spawn(async move {
            let inner = i.lock().await;
            inner.say_hello();
        });
    }
}

#[async_trait]
impl EventsB for CompA {
    fn hello_from_b(&self) {
        let i = self.i.clone();
        tokio::spawn(async move {
            let inner = i.lock().await;
            inner.hello_from_b();
        });
    }
}

#[async_trait]
impl CommandsA for CompAImpl {
    fn say_hello(&self) {
        println!("CompA says hello");
        if let Some(b) = self.b.upgrade() {
            b.hello_from_a();
        } else {
            println!("Failed to find b!")
        }
    }
}

#[async_trait]
impl EventsB for CompAImpl {
    fn hello_from_b(&self) {
        println!("->A: Hello from B");
    }
}
