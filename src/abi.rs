// Generated by antelope-abi2rs 0.3.0 - eosio::abi/1.2

use serde::{Deserialize, Serialize};

type Name = String;
type Bool = bool;
type Uint64 = u64;
type Float32 = String;


macro_rules! impl_try_from_str {
    ($type:ty) => {
        impl TryFrom<&str> for $type {
            type Error = serde_json::Error;
            #[inline]
            fn try_from(str: &str) -> Result<Self, Self::Error> {
                serde_json::from_str(str)
            }
        }
    };
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Action {
    pub device_id: Uint64,
    pub r#type: String,
    pub state: String,
}
impl_try_from_str!(Action);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Deldevice {
    pub device_id: Uint64,
}
impl_try_from_str!(Deldevice);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DevicesRow {
    pub device_id: Uint64,
    pub authority: Name,
}
impl_try_from_str!(DevicesRow);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Location {
    pub device_id: Uint64,
    pub x: Float32,
    pub y: Float32,
    pub z: Option<Float32>,
}
impl_try_from_str!(Location);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Setdevice {
    pub device_id: Uint64,
    pub authority: Name,
}
impl_try_from_str!(Setdevice);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Status {
    pub device_id: Uint64,
    pub battery: Float32,
    pub connected: Bool,
}
impl_try_from_str!(Status);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Temperature {
    pub device_id: Uint64,
    pub temperature: Float32,
}
impl_try_from_str!(Temperature);
