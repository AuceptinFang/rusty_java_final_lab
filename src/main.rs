use exam::server::server;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();
    info!("日志初始化成功");

    server().await.unwrap();            // p3
    //exam::fn1();                      // p1
    //score::lib::main_loop().await;    // p2
}
