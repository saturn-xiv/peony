extern crate env_logger;
extern crate peony;
#[macro_use]
extern crate log;

#[actix_web::main]
async fn main() {
    env_logger::init();
    if let Err(e) = peony::app::launch().await {
        error!("{:?}", e);
    }
}
