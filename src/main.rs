use exam::object::reservation::Reservation;
use exam::score;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();
    info!("日志初始化成功");
    exam::fn3();
    //exam::fn1();
    //score::lib::main_loop().await;
}
