use info::{self as scale_info};
use eosio_scale_info::TypeInfo;
use scale::Encode;

#[derive(TypeInfo, Encode)]
#[scale_info(foo)]
struct InvalidKeywordInScaleInfoAttr {
    a: u8,
    b: u16,
}

fn main() {}
