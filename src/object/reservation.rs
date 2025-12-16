use crate::object::seat::Seat;

pub struct Reservation {
    seat : Box<dyn Seat>,
    hours : i32,
}

impl Reservation {
    pub(crate) fn new(seat: Box<dyn Seat>, hours: i32) -> Self {
        Self { seat, hours }
    }

    pub fn get_total_fee(&self) -> f64 {
        self.seat.calc_fee(self.hours)
    }
}