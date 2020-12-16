extern crate env_logger;
extern crate peony;
#[macro_use]
extern crate log;

fn main() {
    env_logger::init();
    if let Err(e) = peony::app::launch() {
        error!("{:?}", e);
    }
}
