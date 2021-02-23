use std::io;
use std::mem;
use std::net::{self, SocketAddr, ToSocketAddrs, UdpSocket};
use std::time::{Duration, SystemTime};
use std::vec::IntoIter;

use chrono::{DateTime, Local, TimeZone};

/**
 *
 * Radio controlled timepieces
 *
 * 美国，时码代号WWVB，频率为60KHz
 * 德国，时码代号DCF，频率为77.5KHz
 * 英国，时码代号为MSF，频率为60KHz
 * 日本，时码代号为JJY，频率为40KHz
 * 中国，时码代号为BPC，频率为68.5KHz
 *
 *
 */

const MODE_MASK: u8 = 0b0000_0111;
const MODE_SHIFT: u8 = 0;
const VERSION_MASK: u8 = 0b0011_1000;
const VERSION_SHIFT: u8 = 3;
const LI_MASK: u8 = 0b1100_0000;
const LI_SHIFT: u8 = 6;
const NSEC_IN_SEC: u32 = 1_000_000_000;

#[derive(Serialize, Debug, Deserialize)]
struct Packet {
    li_vn_mode: u8,
    stratum: u8,
    poll: i8,
    precision: i8,
    root_delay: u32,
    root_dispersion: u32,
    ref_id: u32,
    ref_timestamp: u64,
    origin_timestamp: u64,
    recv_timestamp: u64,
    tx_timestamp: u64,
}

impl Packet {
    const NTP_TIMESTAMP_DELTA: u32 = 2_208_988_800u32;
    const SNTP_CLIENT_MODE: u8 = 3;
    const SNTP_VERSION: u8 = 4 << 3;

    pub fn new() -> io::Result<Self> {
        let tx_timestamp = Self::get_ntp_timestamp()?;

        Ok(Self {
            li_vn_mode: Self::SNTP_CLIENT_MODE | Self::SNTP_VERSION,
            stratum: 0,
            poll: 0,
            precision: 0,
            root_delay: 0,
            root_dispersion: 0,
            ref_id: 0,
            ref_timestamp: 0,
            origin_timestamp: 0,
            recv_timestamp: 0,
            tx_timestamp,
        })
    }

    fn get_ntp_timestamp() -> io::Result<u64> {
        let now_since_unix = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
        let it = ((now_since_unix.as_secs() + (u64::from(Self::NTP_TIMESTAMP_DELTA))) << 32)
            + u64::from(now_since_unix.subsec_micros());

        Ok(it)
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Response {
    pub sec: u32,
    pub nsec: u32,
    pub roundtrip: u64,
    pub offset: i64,
}

impl From<Response> for DateTime<Local> {
    fn from(val: Response) -> Self {
        Local.timestamp(val.sec as _, val.nsec as _)
    }
}

impl Response {
    pub fn new(sec: u32, nsec: u32, roundtrip: u64, offset: i64) -> Self {
        let residue = nsec / NSEC_IN_SEC;
        let nsec = nsec % NSEC_IN_SEC;
        let sec = sec + residue;

        Self {
            sec,
            nsec,
            roundtrip,
            offset,
        }
    }

    pub fn fetch(pool: &str, port: Option<u16>) -> io::Result<Self> {
        let port = port.unwrap_or(123);
        let socket = net::UdpSocket::bind("0.0.0.0:0")?;
        let dest = format!("{}:{}", pool, port).to_socket_addrs()?;
        socket.set_read_timeout(Some(Duration::new(2, 0)))?;
        let req = Packet::new()?;
        let dest = Self::process_request(dest, &req, &socket)?;
        let mut buf = RawPacket::default();
        let (response, src) = socket.recv_from(buf.0.as_mut())?;
        let recv_timestamp = Packet::get_ntp_timestamp()?;

        if src != dest {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "SNTP response port / address mismatch",
            ));
        }
        if response == mem::size_of::<Packet>() {
            let it = Self::process_response(&req, buf, recv_timestamp)
                .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
            return Ok(it);
        }
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Incorrect NTP packet size read",
        ))
    }

    fn process_request(
        dest: IntoIter<SocketAddr>,
        req: &Packet,
        socket: &UdpSocket,
    ) -> io::Result<SocketAddr> {
        for addr in dest {
            match Self::send_request(&req, &socket, addr) {
                Ok(_) => {
                    return Ok(addr);
                }
                Err(err) => warn!("{}. Try another one", err),
            }
        }
        Err(io::Error::new(
            io::ErrorKind::AddrNotAvailable,
            "SNTP servers not responding",
        ))
    }
    fn send_request(
        req: &Packet,
        socket: &net::UdpSocket,
        dest: net::SocketAddr,
    ) -> io::Result<usize> {
        let buf: RawPacket = req.into();
        socket.send_to(&buf.0, dest)
    }
    fn process_response(
        req: &Packet,
        resp: RawPacket,
        recv_timestamp: u64,
    ) -> Result<Response, &str> {
        const SNTP_UNICAST: u8 = 4;
        const SNTP_BROADCAST: u8 = 5;
        const LI_MAX_VALUE: u8 = 3;
        const MSEC_MASK: u64 = 0x0000_0000_ffff_ffff;
        let shifter = |val, mask, shift| (val & mask) >> shift;
        let mut packet = Packet::from(resp);
        Self::convert_from_network(&mut packet);
        if req.tx_timestamp != packet.origin_timestamp {
            return Err("Incorrect origin timestamp");
        }
        // Shift is 0
        let mode = shifter(packet.li_vn_mode, MODE_MASK, MODE_SHIFT);
        let li = shifter(packet.li_vn_mode, LI_MASK, LI_SHIFT);
        let resp_version = shifter(packet.li_vn_mode, VERSION_MASK, VERSION_SHIFT);
        let req_version = shifter(req.li_vn_mode, VERSION_MASK, VERSION_SHIFT);
        if mode != SNTP_UNICAST && mode != SNTP_BROADCAST {
            return Err("Incorrect MODE value");
        }
        if li > LI_MAX_VALUE {
            return Err("Incorrect LI value");
        }
        if req_version != resp_version {
            return Err("Incorrect response version");
        }
        if packet.stratum == 0 {
            return Err("Incorrect STRATUM headers");
        }
        //    theta = T(B) - T(A) = 1/2 * [(T2-T1) + (T3-T4)]
        //    and the round-trip delay
        //    delta = T(ABA) = (T4-T1) - (T3-T2).
        //    where:
        //      - T1 = client's TX timestamp
        //      - T2 = server's RX timestamp
        //      - T3 = server's TX timestamp
        //      - T4 = client's RX timestamp
        let delta = (recv_timestamp - packet.origin_timestamp) as i64
            - (packet.tx_timestamp - packet.recv_timestamp) as i64;
        let theta = ((packet.recv_timestamp as i64 - packet.origin_timestamp as i64)
            + (recv_timestamp as i64 - packet.tx_timestamp as i64))
            / 2;
        let seconds = (packet.tx_timestamp >> 32) as u32;
        let nsec = (packet.tx_timestamp & MSEC_MASK) as u32;
        let tx_tm = seconds - Packet::NTP_TIMESTAMP_DELTA;
        Ok(Response::new(tx_tm, nsec, delta.abs() as u64, theta))
    }
    fn convert_from_network(packet: &mut Packet) {
        fn ntohl<T: Num>(val: T) -> T::Type {
            val.ntohl()
        }
        packet.root_delay = ntohl(packet.root_delay);
        packet.root_dispersion = ntohl(packet.root_dispersion);
        packet.ref_id = ntohl(packet.ref_id);
        packet.ref_timestamp = ntohl(packet.ref_timestamp);
        packet.origin_timestamp = ntohl(packet.origin_timestamp);
        packet.recv_timestamp = ntohl(packet.recv_timestamp);
        packet.tx_timestamp = ntohl(packet.tx_timestamp);
    }
}

trait Num {
    type Type;

    fn ntohl(&self) -> Self::Type;
}

impl Num for u32 {
    type Type = u32;

    fn ntohl(&self) -> Self::Type {
        self.to_be()
    }
}
impl Num for u64 {
    type Type = u64;

    fn ntohl(&self) -> Self::Type {
        self.to_be()
    }
}

#[derive(Debug)]
struct RawPacket([u8; mem::size_of::<Packet>()]);

impl Default for RawPacket {
    fn default() -> Self {
        Self([0u8; mem::size_of::<Packet>()])
    }
}

impl From<RawPacket> for Packet {
    fn from(val: RawPacket) -> Self {
        let to_array_u32 = |x: &[u8]| {
            let mut buf = [0u8; mem::size_of::<u32>()];
            buf.copy_from_slice(x);
            buf
        };
        let to_array_u64 = |x: &[u8]| {
            let mut buf = [0u8; mem::size_of::<u64>()];
            buf.copy_from_slice(x);
            buf
        };

        Self {
            li_vn_mode: val.0[0],
            stratum: val.0[1],
            poll: val.0[2] as i8,
            precision: val.0[3] as i8,
            root_delay: u32::from_le_bytes(to_array_u32(&val.0[4..8])),
            root_dispersion: u32::from_le_bytes(to_array_u32(&val.0[8..12])),
            ref_id: u32::from_le_bytes(to_array_u32(&val.0[12..16])),
            ref_timestamp: u64::from_le_bytes(to_array_u64(&val.0[16..24])),
            origin_timestamp: u64::from_le_bytes(to_array_u64(&val.0[24..32])),
            recv_timestamp: u64::from_le_bytes(to_array_u64(&val.0[32..40])),
            tx_timestamp: u64::from_le_bytes(to_array_u64(&val.0[40..48])),
        }
    }
}

impl From<&Packet> for RawPacket {
    fn from(val: &Packet) -> Self {
        let mut buf = [0u8; mem::size_of::<Packet>()];

        buf[0] = val.li_vn_mode;
        buf[1] = val.stratum;
        buf[2] = val.poll as u8;
        buf[3] = val.precision as u8;
        buf[4..8].copy_from_slice(&val.root_delay.to_be_bytes());
        buf[8..12].copy_from_slice(&val.root_dispersion.to_be_bytes());
        buf[12..16].copy_from_slice(&val.ref_id.to_be_bytes());
        buf[16..24].copy_from_slice(&val.ref_timestamp.to_be_bytes());
        buf[24..32].copy_from_slice(&val.origin_timestamp.to_be_bytes());
        buf[32..40].copy_from_slice(&val.recv_timestamp.to_be_bytes());
        buf[40..48].copy_from_slice(&val.tx_timestamp.to_be_bytes());

        Self(buf)
    }
}
