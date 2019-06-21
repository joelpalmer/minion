use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Handle;
use std::{net, thread};

trait Cancellable {
    type Error;
    fn run(keep_running: &AtomicBool) -> Self::Error;

    /// the `!` means that this will `never` return unless there is an error
    fn run_here() -> Result<!, Self::Error> {}

    fn run_in_bg() -> Handle {}
}

impl Cancellable for Foo {
    fn run(keep_running: &AtomicBool) {
        let listener = net::TcpListener::bind();
        while keep_running.load(Ordering::SeqCst) {
            let stream = listener.accept()?;

            thread::spawn(move || {
                do_work(stream);
            });
        }
    }
}

fn main() {
    let foo: Foo;
    //foo.run_here();
    let h = foo.run_in_bg(); // h: Clone
    thread::spawn(|| {
        h.end(); // set keep_running = false
    });
    h.wait(); // return Result<(), Self::Error>
}
