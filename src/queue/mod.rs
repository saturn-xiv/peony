pub mod paho;
pub mod rabbit;
pub mod zero;

use mime::Mime;

use super::errors::Result;

pub trait Handler: Sync + Send {
    fn handle(&self, id: &str, content_type: &Mime, payload: &[u8]) -> Result<()>;
}

// TODO
// pub trait Queue {
//     async fn publish(
//         &self,
//         queue: &str,
//         id: &str,
//         content_type: &str,
//         payload: Vec<u8>,
//     ) -> Result<()>;
//     async fn consume<H: Handler>(&self, consumer: &str, queue: &str, handler: &H) -> Result<()>;
// }
