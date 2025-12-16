use tracing::*;
use uuid::Uuid;
use std::sync::atomic::{AtomicU64, Ordering};
use rand::Rng;

static CREATION_COUNT: AtomicU64 = AtomicU64::new(0);
pub trait Seat{
    fn number(&self) -> &str;
    fn name(&self) -> &str;
    fn price(&self) -> f64;
    fn calc_fee(&self, hours: i32) -> f64;
    fn new ()-> Self where Self: Sized;
}
trait Discountable{
    fn apply_discount(&self, hours : i32) -> f64;
}
#[derive(Debug,Clone)]
pub struct NormalSeat{
    number: String,
    name: String,
    price: f64,
}
#[derive(Debug,Clone)]
pub struct MonitorSeat {
    number: String,
    name: String,
    price: f64,
}
#[derive(Debug,Clone)]
pub struct RoomSeat{
    number: String,
    name: String,
    price: f64,
}
impl Seat for NormalSeat{
    fn number(&self) -> &str{
        self.number.as_ref()
    }
    fn name(&self) -> &str{
        &self.name
    }
    fn price(&self) -> f64{
        self.price
    }
    fn calc_fee(&self, hours: i32) -> f64{
        self.price * hours as f64
    }
    fn new ()-> NormalSeat{
        let number = format!("norm-{}", &Uuid::new_v4().simple().to_string()[0..4]);
        let name = format!("普通座位A区 {} 号",CREATION_COUNT.fetch_add(1, Ordering::SeqCst));
        debug!("创建{}",name);
        NormalSeat{
            number,
            name ,
            price: 2.0,
        }
    }
}

impl Seat for MonitorSeat {
    fn number(&self) -> &str{
        self.number.as_ref()
    }
    fn name(&self) -> &str{
        &self.name
    }
    fn price(&self) -> f64{
        self.price
    }
    fn calc_fee(&self, hours: i32) -> f64{
        match hours{
            hours if hours < 3 => {
                self.price * hours as f64
            },
            hours if hours >= 3 => {
                8.5 + 2.5*(hours - 3) as f64
            }
            _ => { 0f64 }
        }
    }
    fn new ()-> MonitorSeat {
        let number = format!("monitor-{}", &Uuid::new_v4().simple().to_string()[0..4]);
        let name = format!("监控区 {} 号",CREATION_COUNT.fetch_add(1, Ordering::SeqCst));
        debug!("创建{}",name);
        MonitorSeat {
            number ,
            name ,
            price: 3.0,
        }
    }
}

impl Seat for RoomSeat{
    fn number(&self) -> &str{
        self.number.as_ref()
    }
    fn name(&self) -> &str{
        &self.name
    }
    fn price(&self) -> f64{
        self.price
    }
    fn calc_fee(&self, hours: i32) -> f64{
        if hours < 4{
            self.price * hours as f64
        }else{
            0.9 * self.price * hours as f64
        }
    }
    fn new ()-> RoomSeat{
        let number = format!("room-{}", &Uuid::new_v4().simple().to_string()[0..4]);
        let name = format!("包间区 {} 号",CREATION_COUNT.fetch_add(1, Ordering::SeqCst));
        debug!("创建{}",name);
        RoomSeat{
            number ,
            name ,
            price: 5.0,
        }
    }
}

impl Discountable for RoomSeat{
    fn apply_discount(&self, hours : i32) -> f64{
        self.calc_fee(hours) * 0.9
    }
}