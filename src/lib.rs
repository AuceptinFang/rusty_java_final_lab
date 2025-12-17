pub mod object;
pub mod score;
pub mod server;
use crate::object::reservation::Reservation;
use crate::object::seat::*;
use crate::score::*;
use rand::Rng;
use tracing::{debug, info};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::prelude::*;


pub fn calc_daily_income(res: Vec<Reservation>) -> f64 {
    let mut total = 0f64;
    for r in res {
        total += r.get_total_fee();
    }
    info!("当日总收入{}元", total);
    total
}

pub fn fn1() {
    let mut rng = rand::rng();
    let mut norm: Vec<NormalSeat> = Vec::new();
    let mut moni: Vec<MonitorSeat> = Vec::new();
    let mut room: Vec<RoomSeat> = Vec::new();
    let mut reservations: Vec<Reservation> = Vec::new();

    for i in 0..10 {
        norm.push(NormalSeat::new());
        moni.push(MonitorSeat::new());
        room.push(RoomSeat::new());
    }

    for i in 0..10 {
        let r = rng.random_range(0..3);
        let hours = rng.random_range(1..10);
        match r {
            0 => reservations.push(Reservation::new(Box::new(norm[i].clone()), hours)),
            1 => reservations.push(Reservation::new(Box::new(room[i].clone()), hours)),
            _ => reservations.push(Reservation::new(Box::new(moni[i].clone()), hours)),
        }
        debug!("生成预约{}类, {}小时", r, hours);
    }
    calc_daily_income(reservations);
}
