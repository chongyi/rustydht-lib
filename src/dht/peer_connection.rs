use std::{net::SocketAddr, time::Duration};

use anyhow::Context;
use serde::Serialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{
    common::{try_decode_peer_id, Id},
    peer_binary_protocol::{Handshake, PIECE_MESSAGE_DEFAULT_LEN, extended::handshake::ExtendedHandshake},
};

pub async fn read_metainfo_from_peer(
    peer_addr: SocketAddr,
    peer_id: Id,
    info_hash: Id,
) -> anyhow::Result<()> {
    let mut conn = match tokio::time::timeout(
        Duration::from_secs(10),
        tokio::net::TcpStream::connect(peer_addr),
    )
    .await
    {
        Ok(conn) => conn.context("error to connecting peer.")?,
        Err(_) => anyhow::bail!("timeout to connecting to {peer_addr}"),
    };

    // handshake first
    let mut write_buf = vec![0u8; PIECE_MESSAGE_DEFAULT_LEN];
    let handshake = Handshake::new(info_hash, peer_id);

    handshake.serialize(&mut write_buf);
    // send handshake message
    conn.write_all(&write_buf)
        .await
        .context("error writing handshake")?;
    write_buf.clear();

    // handle peer handshake response
    let mut read_buf = vec![0u8; PIECE_MESSAGE_DEFAULT_LEN * 2];
    let mut read_so_far = conn
        .read(&mut read_buf)
        .await
        .context("error reading handshake")?;
    if read_so_far == 0 {
        anyhow::bail!("bad handshake");
    }

    let (h, size) = Handshake::deserialize(&read_buf[..read_so_far])
        .map_err(|e| anyhow::anyhow!("error deserializing handshake: {:?}", e))?;

    log::debug!(
        "connected peer {}: {:?}",
        peer_addr,
        try_decode_peer_id(Id::from_bytes(&h.peer_id).context("parse peer id error")?)
    );
    if h.info_hash != info_hash.bytes {
        anyhow::bail!("info hash does not match");
    }

    let supports_extended = h.supports_extended();

    if read_so_far > size {
        // move data to head
        read_buf.copy_within(size..read_so_far, 0);
    }
    read_so_far -= size;

    // must support extended protocol
    if supports_extended {
        // extended handshake
        let extended_handshake = ExtendedHandshake::new();
        log::trace!("sending extended handshake to {peer_addr}: {extended_handshake:?}");

        
    }

    Ok(())
}
