use bincode::Options;
use byteorder::{BE, ByteOrder};
use serde::{Deserialize, Serialize};

use crate::{common::{lengths::CHUNK_SIZE, Id}, errors::MessageDeserializeError};

pub mod extended;

const INTEGER_LEN: usize = 4;
const MSGID_LEN: usize = 1;
const PREAMBLE_LEN: usize = INTEGER_LEN + MSGID_LEN;
const PIECE_MESSAGE_PREAMBLE_LEN: usize = PREAMBLE_LEN + INTEGER_LEN * 2;
pub const PIECE_MESSAGE_DEFAULT_LEN: usize = PIECE_MESSAGE_PREAMBLE_LEN + CHUNK_SIZE as usize;

const NO_PAYLOAD_MSG_LEN: usize = PREAMBLE_LEN;

const PSTR_BT1: &str = "BitTorrent protocol";

const LEN_PREFIX_KEEPALIVE: u32 = 0;
const LEN_PREFIX_CHOKE: u32 = 1;
const LEN_PREFIX_UNCHOKE: u32 = 1;
const LEN_PREFIX_INTERESTED: u32 = 1;
const LEN_PREFIX_NOT_INTERESTED: u32 = 1;
const LEN_PREFIX_HAVE: u32 = 5;
const LEN_PREFIX_PIECE: u32 = 9;
const LEN_PREFIX_REQUEST: u32 = 13;

const MSGID_CHOKE: u8 = 0;
const MSGID_UNCHOKE: u8 = 1;
const MSGID_INTERESTED: u8 = 2;
const MSGID_NOT_INTERESTED: u8 = 3;
const MSGID_HAVE: u8 = 4;
const MSGID_BITFIELD: u8 = 5;
const MSGID_REQUEST: u8 = 6;
const MSGID_PIECE: u8 = 7;
const MSGID_EXTENDED: u8 = 20;

pub const MY_EXTENDED_UT_METADATA: u8 = 3;

#[derive(Serialize, Deserialize, Debug)]
pub struct Handshake<'a> {
    pub pstr: &'a str,
    pub reserved: [u8; 8],
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
}

impl<'a> Handshake<'a> {
    pub fn new(info_hash: Id, peer_id: Id) -> Handshake<'static> {
        debug_assert_eq!(PSTR_BT1.len(), 19);

        let mut reserved: u64 = 0;
        // supports extended messaging
        reserved |= 1 << 20;
        let mut reserved_arr = [0u8; 8];
        BE::write_u64(&mut reserved_arr, reserved);

        Handshake {
            pstr: PSTR_BT1,
            reserved: reserved_arr,
            info_hash: info_hash.bytes,
            peer_id: peer_id.bytes,
        }
    }

    pub fn supports_extended(&self) -> bool {
        self.reserved[5] & 0x10 > 0
    }

    fn bopts() -> impl bincode::Options {
        bincode::DefaultOptions::new()
    }

    pub fn deserialize(b: &[u8]) -> Result<(Handshake<'_>, usize), MessageDeserializeError> {
        let pstr_len = *b
            .get(0)
            .ok_or(MessageDeserializeError::NotEnoughData(1, "handshake"))?;
        let expected_len = 1usize + pstr_len as usize + 48;
        let hbuf = b
            .get(..expected_len)
            .ok_or(MessageDeserializeError::NotEnoughData(
                expected_len,
                "handshake",
            ))?;
        Ok((
            Self::bopts()
                .deserialize(hbuf)
                .map_err(|e| MessageDeserializeError::Other(e.into()))?,
            expected_len,
        ))
    }

    pub fn serialize(&self, buf: &mut Vec<u8>) {
        Self::bopts().serialize_into(buf, &self).unwrap()
    }
}
