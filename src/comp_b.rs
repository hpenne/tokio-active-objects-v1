use crate::interfaces::{EventsA, EventsB};
use crate::CompA;
use async_trait::async_trait;
use std::sync::{Arc, Weak};
use tokio::sync::Mutex;

pub struct CompB {
    i: Arc<Mutex<CompBImpl>>,
}

impl CompB {
    pub fn new() -> Self {
        Self {
            i: Arc::new(Mutex::new(CompBImpl::new())),
        }
    }

    pub fn connect_to_a(&self, a: Arc<CompA>) {
        let i = self.i.clone();
        tokio::spawn(async move {
            let mut inner = i.lock().await;
            println!("Connecting to a");
            inner.connect_to_a(Arc::downgrade(&a));
        });
    }
}
struct CompBImpl {
    a: Weak<CompA>,
}

impl CompBImpl {
    pub fn new() -> Self {
        Self {
            a: Weak::<CompA>::new(),
        }
    }

    pub fn connect_to_a(&mut self, a: Weak<CompA>) {
        self.a = a;
        println!("Connected to a");
    }
}

#[async_trait]
impl EventsA for CompB {
    fn hello_from_a(&self) {
        let i = self.i.clone();
        tokio::spawn(async move {
            let inner = i.lock().await;
            inner.hello_from_a();
        });
    }
}

#[async_trait]
impl EventsA for CompBImpl {
    fn hello_from_a(&self) {
        println!("->B: Hello from A");
        if let Some(a) = self.a.upgrade() {
            a.hello_from_b();
        } else {
            println!("Failed to find a!")
        }
    }
}
