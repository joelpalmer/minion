//#![deny(missing_doc)]

use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

pub enum LoopResult<E> {
    Continue,
    Break,
    Error(E),
}

pub trait Cancellable {
    type Error;
    fn for_each(&mut self) -> LoopResult<Self::Error>;

    fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            match self.for_each() {
                LoopResult::Continue => {}
                LoopResult::Break => break,
                LoopResult::Error(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn spawn(self) -> Handle<Self::Error>
    where
        Self: Send,
        Self::Error: Send,
    {
        let keep_running = Arc::new(AtomicBool::new(true));
        let jh = {
            let keep_running = keep_running.clone();
            thread::spawn(move || {
                while keep_running.load(Ordering::SeqCst) {
                    match self.for_each() {
                        LoopResult::Continue => {}
                        LoopResult::Break => break,
                        LoopResult::Error(e) => return Err(e),
                    }
                }
                Ok(())
            })
        };
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
