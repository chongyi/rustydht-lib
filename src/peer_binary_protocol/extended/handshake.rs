use std::collections::HashMap;

use crate::{common::buffer::ByteBuf, peer_binary_protocol::MY_EXTENDED_UT_METADATA};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct ExtendedHandshake {
    // #[serde(bound(deserialize = "ByteBuf: From<&'de [u8]>"))]
    // pub m: HashMap<Vec<u8>, u8>,
    pub m: Dictionary,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", with = "serde_bytes")]
    pub v: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", with = "serde_bytes")]
    pub yourip: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", with = "serde_bytes")]
    pub ipv6: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none", with = "serde_bytes")]
    pub ipv4: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reqq: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_ago: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_only: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Dictionary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ut_metadata: Option<u8>,
}

impl ExtendedHandshake {
    pub fn new() -> Self {
        Self {
            m: Dictionary { ut_metadata: Some(MY_EXTENDED_UT_METADATA) },
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ExtendedHandshake;

    #[test]
    fn test_serialize_handshake_only_ut_metadata() {
        assert_eq!(
            "d1:md11:ut_metadatai1eee",
            serde_bencode::ser::to_string(&ExtendedHandshake {
                m: super::Dictionary { ut_metadata: Some(1) },
                ..Default::default()
            })
            .unwrap()
        )
    }
}
