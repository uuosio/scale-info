use info::{self as scale_info};
use eosio_scale_info::TypeInfo;

#[derive(TypeInfo)]
struct Me {
    _me: Box<Me>,
}

fn assert_type_info<T: TypeInfo + 'static>() {}

fn main() {
    assert_type_info::<Me>();
}
