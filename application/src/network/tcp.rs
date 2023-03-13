use anyhow::Result;
use tokio::net::UdpSocket;

use crate::client::{PEER_ID, PORT, KEY};
use crate::tools::assert_eq;

async fn send_connect(addr: String) -> Result<i64> {
    let socket = UdpSocket::bind(&addr).await?;

    const REQ_LEN: usize = 16;
    let magic_const = (0x41727101980 as i64).to_be_bytes();
    let action = (0 as i32).to_be_bytes();
    let transaction_id = generate_transaction_id()?;

    let mut req = Vec::with_capacity(REQ_LEN); // TODO: Делать на стеке
    req.extend_from_slice(&magic_const[..]);
    req.extend_from_slice(&action[..]);
    req.extend_from_slice(&transaction_id[..]);
    let sended_bytes = socket.send(&req).await?;
    assert_eq(sended_bytes, REQ_LEN)?;

    const RECV_LEN: usize = 16;
    let mut recv = [0; RECV_LEN];
    let (recieved_bytes, _) = socket.recv_from(&mut recv).await?;
    assert_eq(recieved_bytes, RECV_LEN)?;

    let recv_action = i32::from_be_bytes(recv[0..4].try_into()?);
    let recv_transaction_id = i32::from_be_bytes(recv[4..8].try_into()?);
    let recv_connection_id = i64::from_be_bytes(recv[8..16].try_into()?);
    assert_eq(recv_action, 0)?;
    assert_eq(recv_transaction_id.to_be_bytes(), transaction_id)?;
    Ok(recv_connection_id)
}

enum Event {
    None = 0,
    Completed = 1,
    Started = 2,
    Stopped = 3,
}

/// Отвечает за состояние загрузки.
/// Не менять размерности полей.
struct SyncRequest {
    connection_id: i64,
    hash: [u8; 20],
    peer_id: [u8; 20],
    downloaded: i64,
    left: i64,
    uploaded: i64,
    event: Event,
    key: i16,
    port: u64,
    addr: String,
}

async fn send_announce(request: SyncRequest) -> Result<()> {
    let socket = UdpSocket::bind(request.addr).await?;

    const REQ_LEN: usize = 98;
    let connection_id: [u8; 8] = request.connection_id.to_be_bytes();
    let action: [u8; 4] = (1 as i32).to_be_bytes();
    let peer_id: [u8; 20] = PEER_ID;
    let downloaded: [u8; 8] = request.downloaded.to_be_bytes();
    let left: [u8; 8] = request.left.to_be_bytes();
    let uploaded: [u8; 8] = request.uploaded.to_be_bytes();
    let event: [u8; 4] = (request.event as i32).to_be_bytes();
    let ip_addr: [u8; 4] = (0 as i32).to_be_bytes();
    let key: [u8; 4] = KEY.to_be_bytes();
    let num_want: [u8; 4] = (-1 as i32).to_be_bytes();
    let port: [u8; 2] = (PORT).to_be_bytes();
    let transaction_id: [u8; 4] = generate_transaction_id()?;

    let mut req = Vec::with_capacity(REQ_LEN); // TODO: Делать на стеке
    // Данные должны идти именно в такой последовательности
    req.extend_from_slice(&connection_id[..]);
    req.extend_from_slice(&action[..]);
    req.extend_from_slice(&transaction_id[..]);
    req.extend_from_slice(&request.hash[..]);
    req.extend_from_slice(&peer_id[..]);
    req.extend_from_slice(&downloaded[..]);
    req.extend_from_slice(&left[..]);
    req.extend_from_slice(&uploaded[..]);
    req.extend_from_slice(&event[..]);
    req.extend_from_slice(&ip_addr[..]);
    req.extend_from_slice(&key[..]);
    req.extend_from_slice(&num_want[..]);
    req.extend_from_slice(&port[..]);
    let sended_bytes = socket.send(&req).await?;
    assert_eq(sended_bytes, REQ_LEN)?;

    let RECV_LEN = 20 + 6 * 50; // 20 байт - фискированные данные, 50 штук пар ip/port
    let mut recv =  Vec::with_capacity(RECV_LEN);
    let (recv_bytes, _) = socket.recv_from(&mut recv).await?; 
    if recv_bytes < 20 {
        
    }


    Ok(())
}

fn generate_transaction_id() -> Result<[u8; 4]> {
    let mut transaction_id: [u8; 4] = [0; 4];
    getrandom::getrandom(&mut transaction_id)?;
    Ok(transaction_id)
}
