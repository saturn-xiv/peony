pub mod g786;

// use std::fmt::{Debug, Display};
// use std::io::{prelude::*, Error as IoError, ErrorKind as IoErrorKind};
// use std::result::Result as StdResult;
// use std::str::Utf8Error as StrUtf8Error;
use std::sync::Mutex;
// use std::thread;
// use std::time::Duration;

// use bytes::BytesMut;
// use chrono::Utc;
use serialport::{SerialPortBuilder, TTYPort};

// use super::super::errors::Result;

pub struct Queue {
    pub port: Mutex<TTYPort>,
}

impl Queue {
    pub const ORAGNTE_PI_UART1: &'static str = "/dev/ttyS1";
    pub const ORAGNTE_PI_UART2: &'static str = "/dev/ttyS2";
    pub const USB0: &'static str = "/dev/ttyUSB0";
    pub const USB1: &'static str = "/dev/ttyUSB1";
    pub const RASPBERRY_PI_UART1: &'static str = "/dev/serial0";
    pub const RETRY: u8 = 32;
    pub const MAX_BUFFER_LEN: usize = 1 << 16;
}

// impl Queue {

//     pub fn open<T: AsRef<str>>(name: T) -> StdResult<Self, IoError> {
//         let name = name.as_ref();
//         Self::new(serialport::new(name, 9600).timeout(Duration::from_millis(10)))
//     }

//     pub fn new(builder: SerialPortBuilder) -> StdResult<Self, IoError> {
//         info!("open serial port {:?}", builder);
//         let mut port = builder.open_native()?;
//         port.set_exclusive(true)?;
//         Ok(Self {
//             port: Mutex::new(port),
//         })
//     }

//     pub fn write(&self, buf: &[u8]) -> Result<()> {
//         let now = Utc::now();
//         let mut i = 1;
//         loop {
//             if let Ok(mut port) = self.port.lock() {
//                 match port.write_all(buf) {
//                     Ok(_) => {
//                         debug!(
//                             "try {} times for write tty {}, spend {}",
//                             i + 1,
//                             buf.len(),
//                             (Utc::now() - now)
//                         );
//                         return Ok(());
//                     }
//                     Err(ref e) if e.kind() == IoErrorKind::TimedOut => {
//                         thread::sleep(Duration::from_micros(100));
//                     }
//                     Err(e) => {
//                         error!("{:?}", e);
//                         i += 1;
//                         if i >= Self::RETRY {
//                             return Err(e.into());
//                         }
//                     }
//                 };
//             }
//         }
//     }

//     pub fn listen<H>(&self, port: u16, handler: &H) -> StdResult<(), IoError>
//     where
//         H: Fn(&str) -> Option<(String, usize)>,
//     {
//         let mut line = String::new();
//         loop {
//             if let Ok(mut port) = self.port.lock() {
//                 let mut buf: Vec<u8> = vec![0; 1 << 4];
//                 match port.read(buf.as_mut_slice()) {
//                     Ok(len) => {
//                         match std::str::from_utf8(&buf) {
//                             Ok(buf) => {
//                                 debug!("receive {} bytes: {:?}", len, buf);
//                                 line.push_str(buf);
//                                 if let Some((it, at)) = handler(&line) {
//                                     // debug!("match {}", it);
//                                     // debug!("split to: {:?}", std::str::from_utf8(&buf));
//                                     // return handler.handle(&it);
//                                 }
//                                 if line.len() >= Self::MAX_BUFFER_LEN {
//                                     warn!("buffer is full, will clear");
//                                     line.clear();
//                                 }
//                             }
//                             Err(e) => {
//                                 error!("{:?}, clear message: {}", e, line);
//                                 line.clear();
//                             }
//                         }
//                     }
//                     Err(ref e) if e.kind() == IoErrorKind::TimedOut => {}
//                     Err(e) => {
//                         return Err(e.into());
//                     }
//                 }
//             }
//             thread::sleep(Duration::from_micros(100));
//         }
//     }
// }

// pub trait Decoder {
//     type Item: Debug;
//     type Error: Debug;
//     fn decode(buf: &[u8]) -> StdResult<Option<(Self::Item, usize)>, Self::Error>;
// }

// pub trait Handler {
//     type Item: Debug;
//     type Error: From<StrUtf8Error> + From<IoError> + Debug;
//     fn handle(&mut self, msg: &Self::Item) -> StdResult<(), Self::Error>;
// }

// pub struct Tty {
//     pub port: TTYPort,
//     line: BytesMut,
// }

// impl Tty {
//     pub const ORAGNTE_PI_UART1: &'static str = "/dev/ttyS1";
//     pub const ORAGNTE_PI_UART2: &'static str = "/dev/ttyS2";
//     pub const USB0: &'static str = "/dev/ttyUSB0";
//     pub const USB1: &'static str = "/dev/ttyUSB1";
//     pub const RASPBERRY_PI_UART1: &'static str = "/dev/serial0";
//     pub const RETRY: u8 = 32;
//     pub const MAX_BUFFER_LEN: usize = 1 << 16;

//     pub fn open(name: &str) -> StdResult<Self, IoError> {
//         Self::new(serialport::new(name, 9600).timeout(Duration::from_millis(10)))
//     }

//     pub fn new(builder: SerialPortBuilder) -> StdResult<Self, IoError> {
//         info!("open serial port {:?}", builder);
//         let mut port = builder.open_native()?;
//         port.set_exclusive(true)?;
//         Ok(Self {
//             port,
//             line: BytesMut::new(),
//         })
//     }

//     pub fn ports() -> Result<Vec<String>> {
//         let items = serialport::available_ports()?
//             .iter()
//             .map(|x| x.port_name.clone())
//             .collect();
//         Ok(items)
//     }

//     pub fn send<Q: Display>(&mut self, query: &Q) -> Result<()> {
//         let query = query.to_string();
//         info!("send {}", query);
//         self.write(query.as_bytes())
//     }

//     pub fn write(&mut self, buf: &[u8]) -> Result<()> {
//         let now = Utc::now();
//         let mut i = 1;
//         loop {
//             match self.port.write_all(buf) {
//                 Ok(_) => {
//                     debug!(
//                         "try {} times for write tty {}, spend {}",
//                         i + 1,
//                         buf.len(),
//                         (Utc::now() - now)
//                     );
//                     return Ok(());
//                 }
//                 Err(ref e) if e.kind() == IoErrorKind::TimedOut => {
//                     thread::sleep(Duration::from_micros(100));
//                 }
//                 Err(e) => {
//                     error!("{:?}", e);
//                     i += 1;
//                     if i >= Self::RETRY {
//                         return Err(e.into());
//                     }
//                 }
//             };
//         }
//     }

//     pub fn read<D, ED, H, EH>(&mut self, handler: &mut H) -> StdResult<(), EH>
//     where
//         D: Decoder<Item = D, Error = ED>,
//         ED: From<IoError> + Debug,
//         H: Handler<Item = D, Error = EH>,
//         EH: From<ED> + From<IoError> + Debug,
//     {
//         let mut buf: Vec<u8> = vec![0; 1 << 4];
//         match self.port.read(buf.as_mut_slice()) {
//             Ok(len) => {
//                 debug!("receive {} bytes", len);
//                 // log::trace!("{:?}", std::str::from_utf8(&buf[..len]));
//                 self.line.extend_from_slice(&buf[..len]);
//                 if let Some((it, at)) = D::decode(&self.line)? {
//                     let buf = self.line.split_to(at);
//                     debug!("split to: {:?}", std::str::from_utf8(&buf));
//                     return handler.handle(&it);
//                 }
//                 if self.line.len() >= Self::MAX_BUFFER_LEN {
//                     warn!("buffer is full, will clear");
//                     self.line.clear();
//                 }
//                 Ok(())
//             }
//             Err(ref e) if e.kind() == IoErrorKind::TimedOut => Ok(()),
//             Err(e) => Err(e.into()),
//         }
//     }
// }
