extern crate minion;
use minion::Cancellable;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    io::{self, prelude::*},
    net, thread,
};

struct Service(net::TcpListener);

impl minion::Cancellable for Service {
    type Error = io::Error;
    fn for_each(&mut self) -> Result<minion::LoopState, Self::Error> {
        let mut stream = self.0.accept()?.0;
        write!(stream, "hello!")?;
        Ok(minion::LoopState::Continue)
    }
}

impl Service {
    fn new() -> Self {
        let listener = net::TcpListener::bind("127.0.0.1:6556").unwrap();
        Service(listener)
    }
}

fn main() {
    let mut s = Service::new();
    s.run().unwrap();
}
