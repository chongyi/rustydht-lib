use super::{Id, ID_SIZE};

#[derive(Debug)]
pub enum AzureusStyleKind {
    Deluge,
    LibTorrent,
    Transmission,
    Other([char; 2]),
}

#[derive(Debug)]
pub struct AzureusStyle {
    pub kind: AzureusStyleKind,
    pub version: [char; 4],
}

impl AzureusStyleKind {
    pub const fn from_bytes(b1: u8, b2: u8) -> Self {
        match &[b1, b2] {
            b"DE" => AzureusStyleKind::Deluge,
            b"lt" | b"LT" => AzureusStyleKind::LibTorrent,
            b"TR" => AzureusStyleKind::Transmission,
            _ => AzureusStyleKind::Other([b1 as char, b2 as char]),
        }
    }
}

fn try_decode_azureus_style(p: &Id) -> Option<AzureusStyle> {
    let p = p.bytes;
    if !(p[0] == b'-' && p[7] == b'-') {
        return None;
    }
    let mut version = ['0'; 4];
    for (i, c) in (&p[3..7]).iter().copied().enumerate() {
        version[i] = c as char;
    }
    let kind = AzureusStyleKind::from_bytes(p[1], p[2]);
    Some(AzureusStyle { kind, version })
}

#[derive(Debug)]
pub enum PeerId {
    AzureusStyle(AzureusStyle),
}

pub fn try_decode_peer_id(p: Id) -> Option<PeerId> {
    Some(PeerId::AzureusStyle(try_decode_azureus_style(&p)?))
}

pub fn generate_peer_id() -> Id {
    let mut peer_id = [0u8; ID_SIZE];

    let u = uuid::Uuid::new_v4();
    (&mut peer_id[4..ID_SIZE]).copy_from_slice(&u.as_bytes()[..]);

    (&mut peer_id[..8]).copy_from_slice(b"-rQ0001-");

    Id { bytes: peer_id }
}
