//#![deny(missing_doc)]

use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

pub enum LoopResult<T, E> {
    Continue,
    Break(T),
    Error(E),
}

pub trait Cancellable {
    type Output;
    type Error;
    fn for_each(&mut self) -> LoopResult<Self::Output, ::Error>;

    fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            match self.for_each() {
                LoopResult::Continue => {}
                LoopResult::Break(v) => return Ok(v),
                LoopResult::Error(e) => return Err(e),
            }
        }
    }

    fn spawn(self) -> Handle<Self::Output, Self::Error>
    where
        Self: Send,
        Self::Error: Send,
        Self::Output: Send,
    {
        let keep_running = Arc::new(AtomicBool::new(true));
        {
            let keep_running = keep_running.clone();
            let jh = thread::spawn(move || {});
        }
        let h = Handle::new();
        unimplemented!();
    }
}

#[derive(Clone)]
pub struct Handle<E> {}

impl<E> Handle<E> {
    pub fn wait(self) -> Result<(), E> {
        unimplemented!()
    }

    pub fn terminate(&self) {
        unimplemented!()
    }
}
