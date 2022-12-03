use front_house::{
    hosting::{add_to_wait_list, seat_at_table},
    serving::{serve_order, take_order, take_payment},
};

mod front_house;

pub fn eat_at_restaurant() {
    add_to_wait_list();
    seat_at_table();
    take_order();
    serve_order();
    take_payment();
}
