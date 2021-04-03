use std::collections::BTreeMap;
use std::default::Default;
use std::ops::Deref;
use std::process::Command;

use actix_web::{get, post, web, HttpResponse, Responder};
use eui48::MacAddress;

use super::super::super::super::{
    cache::{redis::Pool as DbPool, Kv},
    errors::Result,
    sys::network::{
        ip4 as get_ip4, is_on, mac as get_mac,
        systemd::{Dhcp, Static, Wifi, Wpa},
    },
};
use super::CurrentUser;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub wifi: Option<Wifi>,
    pub ether: Ether,
}

impl Form {
    pub const KEY: &'static str = "network";

    const ETH: &'static str = "eth0";
    const WLAN: &'static str = "wlan0";
    pub fn is_on() -> bool {
        (is_on(Self::WLAN) && get_ip4(Self::WLAN).is_some())
            || (is_on(Self::ETH) && get_ip4(Self::ETH).is_some())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Ether {
    Dhcp,
    Static {
        address: String,
        netmask: String,
        gateway: String,
        dns1: String,
        dns2: String,
    },
}

impl Default for Ether {
    fn default() -> Self {
        Self::Dhcp
    }
}

impl Form {
    #[cfg(debug_assertions)]
    pub fn mac(&self) -> Result<MacAddress> {
        get_mac("wlp3s0")
    }
    #[cfg(not(debug_assertions))]
    pub fn mac(&self) -> Result<MacAddress> {
        get_mac(Self::ETH)
    }

    fn save(&self, name: &str) -> Result<()> {
        debug!("save network interfaces {:?}", self);
        {
            let metric = 50;
            match self.ether {
                Ether::Static {
                    ref address,
                    ref netmask,
                    ref gateway,
                    ref dns1,
                    ref dns2,
                } => Static::new(
                    Self::ETH,
                    metric,
                    address,
                    netmask,
                    gateway,
                    dns1,
                    Some(dns2),
                )?
                .save(name)?,
                Ether::Dhcp => Dhcp {
                    name: Self::ETH.to_string(),
                    metric,
                    options: vec![Dhcp::WWW],
                }
                .save(name)?,
            };
        }

        {
            match self.wifi {
                Some(ref it) => {
                    it.save(Self::WLAN)?;
                    Dhcp {
                        name: Self::WLAN.to_string(),
                        metric: 200,
                        options: vec![Dhcp::WWW],
                    }
                    .save(name)?;
                    Wpa.save(Self::WLAN)?;
                }
                None => {
                    Wifi::remove(Self::WLAN)?;
                }
            }
        }

        Ok(())
    }
}

#[get("/status")]
async fn status(_user: CurrentUser) -> Result<impl Responder> {
    let mut items = BTreeMap::new();
    items.insert(Form::ETH, get_ip4(Form::ETH));
    items.insert(Form::WLAN, get_ip4(Form::WLAN));
    Ok(HttpResponse::Ok().json(items))
}

#[post("/ping/{host}")]
async fn ping(_user: CurrentUser, path: web::Path<String>) -> Result<impl Responder> {
    // let mut ping = oping::Ping::new();
    // ping.set_timeout(2.0)?;
    // ping.add_host(&path.0)?;
    // let items: Vec<String> = ping
    //     .send()?
    //     .map(|it| {
    //         if it.dropped > 0 {
    //             format!("No response from host: {}", it.hostname)
    //         } else {
    //             format!(
    //                 "Response from host {} (address {}): latency {} ms",
    //                 it.hostname, it.address, it.latency_ms
    //             )
    //         }
    //     })
    //     .collect();

    // Ok(HttpResponse::Ok().json(items))

    let out = Command::new("ping")
        .arg("-c")
        .arg(4.to_string())
        .arg("-i")
        .arg(1.to_string())
        .arg(&path.0)
        .output()?;
    let out = String::from_utf8(if out.status.success() {
        out.stdout
    } else {
        out.stderr
    })?;
    Ok(HttpResponse::Ok().json(out))
}

#[get("/")]
async fn get(_user: CurrentUser, db: web::Data<DbPool>) -> Result<impl Responder> {
    let db = db.deref();
    let db = db.deref();
    let it: Form = Kv::get(db, &Form::KEY.to_string()).unwrap_or_default();
    Ok(HttpResponse::Ok().json(it))
}

#[post("/")]
async fn set(
    _user: CurrentUser,
    form: web::Json<Form>,
    db: web::Data<DbPool>,
) -> Result<impl Responder> {
    let db = db.deref();
    let db = db.deref();
    let form = form.deref();
    form.save("pi")?;
    Kv::set(db, &Form::KEY.to_string(), form)?;
    Ok(HttpResponse::Ok().json(()))
}
