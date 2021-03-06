pub mod album;
pub mod cbeta;
pub mod forum;
pub mod iapt;
pub mod nut;
pub mod ops;
pub mod pi;
pub mod survey;
pub mod vip;

use super::orm::migration::New as Migration;

pub trait Plugin {
    fn migrations<'a>() -> Vec<Migration<'a>>;
}
