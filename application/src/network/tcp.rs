use std::sync::Arc;

use anyhow::Result;
use tokio::net::UdpSocket;

use crate::repository::types::Torrent;
use crate::tools::assert_eq;

async fn send_connect(addr: String) -> Result<i64> {
    let socket = UdpSocket::bind(&addr).await?;

    const REQ_LEN: usize = 16;
    let magic_const = (0x41727101980 as i64).to_be_bytes();
    let action = (0 as i32).to_be_bytes();
    let mut transaction_id: [u8; 4] = [0; 4];
    getrandom::getrandom(&mut transaction_id)?;

    let mut req = Vec::with_capacity(REQ_LEN);
    req.extend_from_slice(&magic_const[..]);
    req.extend_from_slice(&action[..]);
    req.extend_from_slice(&transaction_id[..]);
    let sended_bytes = socket.send(&req).await?;
    assert_eq(sended_bytes, REQ_LEN)?;

    const RECV_LEN: usize = 16;
    let mut recv: [u8; RECV_LEN] = [0; RECV_LEN];
    let (recieved_bytes, _) = socket.recv_from(&mut recv).await?;
    assert_eq(recieved_bytes, RECV_LEN)?;

    let recv_action = i32::from_be_bytes(recv[0..4].try_into()?);
    let recv_transaction_id = i32::from_be_bytes(recv[4..8].try_into()?);
    let recv_connection_id = i64::from_be_bytes(recv[8..16].try_into()?);
    assert_eq(recv_action, 0)?;
    assert_eq(recv_transaction_id.to_be_bytes(), transaction_id)?;
    Ok(recv_connection_id)
}

struct SyncState {
    torrent: Arc<Torrent>,
}

async fn send_announce(state: SyncState) {}
