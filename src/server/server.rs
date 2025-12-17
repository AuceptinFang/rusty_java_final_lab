use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{error, info};
use crate::server;
use crate::server::seat;

pub async fn server() -> anyhow::Result<()> {
    let PORT = 5000;
    let listener = TcpListener::bind(("127.0.0.1",PORT)).await?;
    info!("服务器已启动在{}端口",PORT);
    let mut seat = server::seat::Seat::new();
    loop{
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_conn(socket, &mut seat).await{
                error!("未预期的异常{}",e);
            }
        });
    }

    Ok(())
}

async fn handle_conn(mut socket : TcpStream, seat : &mut server::seat::Seat ) -> anyhow::Result<()> {
    if let Ok(msg) = readUTF(socket).await{
        match msg.find('|') {
            Some(index) => {
                let option = msg[0..index].to_string();
                let id = &msg[index + 1..];
                match option.as_str() {
                    "BOOK" =>{
                        match seat.book(id).await{
                            Ok(id) =>{
                                writeUTF(socket,format!("Ok|{}",id).as_str()).await?;
                                Ok(())
                            }
                            Err(e) => {
                                writeUTF(socket,format!("Err|{}",id).as_str()).await?;
                                Ok(())
                            }
                        }
                    }
                    "CANCEL" =>{
                        match seat.cancel(id).await{
                            Ok(id) =>{
                                writeUTF(socket,format!("Ok|{}",id).as_str()).await?;
                                Ok(())
                            }
                            Err(e) => {
                                writeUTF(socket,format!("Err|{}",id).as_str()).await?;
                                Ok(())
                            }
                        }
                    }
                    "STATUS" =>{
                        match seat.status().await{
                            Ok(status) =>{
                                socket.write_all(&status.to_be_bytes()).await?;
                                Ok(())
                            }
                            Err(e) =>{
                                writeUTF(socket,format!("Err|{}",e).as_str()).await?;
                                Ok(())
                            }
                        }
                    }
                    _ =>{
                        writeUTF(socket,"400 BAD REQUEST|\r\n").await?;
                        Ok(())
                    }
                }
            }
            None => {
                writeUTF(socket, "400 BAD REQUEST|\r\n").await?;
                return Ok(());
            }
        }
    }
}

async fn readUTF(mut socket : TcpStream) -> anyhow::Result<String> {
    let mut len_buf = [0u8; 2];
    socket.read_exact(&mut len_buf).await?;
    let msg_len = u16::from_be_bytes(len_buf) as usize;
    let mut msg_buf = vec![0u8; msg_len];
    socket.read_exact(&mut msg_buf).await?;

    let msg = String::from_utf8(msg_buf)?;
    info!("收到: {}", msg);
    Ok(msg)
}

async fn writeUTF(mut socket : TcpStream, msg : &str) -> anyhow::Result<()> {
    let len = msg.len() as u16;
    socket.write_all(&len.to_be_bytes()).await?;
    socket.write_all(msg.as_bytes()).await?;
    info!("发送: {}",msg);
    Ok(())
}