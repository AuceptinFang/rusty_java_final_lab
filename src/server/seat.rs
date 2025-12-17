use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::Mutex;
use anyhow::Result;
use tracing::{info, warn};

pub(crate) struct Seat{
    state: Arc<Mutex<SeatState>>,
}

struct SeatState {
    seats: usize, //一个计数器
    available_seats: Vec<String>,
    taken_up_seats: Vec<String>,
    user_map: HashMap<String, String>,
}

impl Seat{
    pub fn new() -> Self{
        let mut available_seats = Vec::<String>::new();
        for i in 0..15{
            available_seats.push(format!("S{}", i));
        }

        let data = SeatState{
            seats : 15,
            available_seats,
            taken_up_seats: Vec::new(),
            user_map: HashMap::new(),
        };
        Seat{
            state : Arc::new(Mutex::new(data)),
        }
    }

    pub async fn book(&self, id : &str) -> Result<String> {
        let result = {
            let mut guard = self.state.lock();

            if let Some(seat) = guard.user_map.get(&id.to_string()){
                return anyhow::bail!(format!("User already exists: {}",seat));
            }

            if !guard.available_seats.is_empty(){
                let seat_id = guard.available_seats.remove(0);

                guard.taken_up_seats.push(seat_id.clone());
                guard.user_map.insert(id.clone().to_string(), seat_id.clone());
                guard.seats -= 1;
                Ok(seat_id)
            }else{
                anyhow::bail!("无可用座位")
            }
        };
        match result{
            Ok(seat) => {
                info!("Seat id: {} 被用户 {} 预约成功", seat, id);
                Ok(seat)
            },
            Err(e) => {
                info!("用户 {} 预约失败", id);
                Err(e)
            },
        }
    }

    pub async fn cancel(&self, id: &str) -> Result<String> {
        let result = {
            let mut guard = self.state.lock();
            if let Some(seat_id) = guard.user_map.remove(&id.to_string()) {
                // 寻找座位
                if let Some(pos) = guard.taken_up_seats.iter().position(|x| *x == seat_id) {
                    guard.taken_up_seats.remove(pos);
                }

                guard.available_seats.push(seat_id.clone());
                guard.seats += 1;

                Ok(seat_id)
            } else {
                warn!("用户 {} 未找到预订记录", id);
                anyhow::bail!("用户 {} 未找到预订记录", id)
            }
        };
        if let Ok(ref seat) = result {
            println!("用户 {} 退票成功: {}", id, seat);
        }
        result
    }

    pub async fn status(&self) -> Result<usize> {
        let count = {
            self.state.lock().seats
        };
        Ok(count)
    }
}