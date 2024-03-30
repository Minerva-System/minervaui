pub mod products;
pub mod user;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref MINERVAHOST: Mutex<String> =
        Mutex::new("http://192.168.3.6:30000/api/v1".into());
}
