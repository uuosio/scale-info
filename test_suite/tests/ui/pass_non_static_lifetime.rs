use info::{self as scale_info};
use eosio_scale_info::TypeInfo;

#[derive(TypeInfo)]
struct Me<'a> {
    _me: &'a Me<'a>,
}

fn assert_type_info<T: TypeInfo + 'static>() {}

fn main() {
    assert_type_info::<Me>();
}
