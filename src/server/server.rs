use crate::server::seat::Seat;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info};

pub async fn server() -> anyhow::Result<()> {
    let port = 5000;
    let listener = TcpListener::bind(("127.0.0.1", port)).await?;
    info!("服务器已启动在{}端口", port);
    let seat = Arc::new(Seat::new());

    loop {
        let (socket, _) = listener.accept().await?;
        let seat_clone = Arc::clone(&seat);
        tokio::spawn(async move {
            if let Err(e) = handle_conn(socket, seat_clone).await {
                error!("未预期的异常{}", e);
            }
        });
    }
}

async fn handle_conn(mut socket: TcpStream, seat: Arc<Seat>) -> anyhow::Result<()> {
    if let Ok(msg) = read_utf(&mut socket).await {
        match msg.find('|') {
            Some(index) => {
                let option = msg[0..index].to_string();
                let id = &msg[index + 1..];
                match option.as_str() {
                    "BOOK" => match seat.book(id).await {
                        Ok(seat_id) => {
                            write_utf(&mut socket, &format!("Ok|{}", seat_id)).await?;
                            Ok(())
                        }
                        Err(e) => {
                            write_utf(&mut socket, &format!("Err|{}", e)).await?;
                            Ok(())
                        }
                    },
                    "CANCEL" => match seat.cancel(id).await {
                        Ok(seat_id) => {
                            write_utf(&mut socket, &format!("Ok|{}", seat_id)).await?;
                            Ok(())
                        }
                        Err(e) => {
                            write_utf(&mut socket, &format!("Err|{}", e)).await?;
                            Ok(())
                        }
                    },
                    "STATUS" => match seat.status().await {
                        Ok(status) => {
                            socket.write_all(&status.to_be_bytes()).await?;
                            Ok(())
                        }
                        Err(e) => {
                            write_utf(&mut socket, &format!("Err|{}", e)).await?;
                            Ok(())
                        }
                    },
                    _ => {
                        write_utf(&mut socket, "400 BAD REQUEST|\r\n").await?;
                        Ok(())
                    }
                }
            }
            None => {
                write_utf(&mut socket, "400 BAD REQUEST|\r\n").await?;
                return Ok(());
            }
        }
    } else {
        Ok(())
    }
}

async fn read_utf(socket: &mut TcpStream) -> anyhow::Result<String> {
    let mut len_buf = [0u8; 2];
    socket.read_exact(&mut len_buf).await?;
    let msg_len = u16::from_be_bytes(len_buf) as usize;
    let mut msg_buf = vec![0u8; msg_len];
    socket.read_exact(&mut msg_buf).await?;

    let msg = String::from_utf8(msg_buf)?;
    info!("收到: {}", msg);
    Ok(msg)
}

async fn write_utf(socket: &mut TcpStream, msg: &str) -> anyhow::Result<()> {
    let len = msg.len() as u16;
    socket.write_all(&len.to_be_bytes()).await?;
    socket.write_all(msg.as_bytes()).await?;
    info!("发送: {}", msg);
    Ok(())
}
