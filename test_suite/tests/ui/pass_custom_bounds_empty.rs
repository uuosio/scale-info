use info::{self as scale_info};
use eosio_scale_info::TypeInfo;
use core::marker::PhantomData;

#[derive(TypeInfo)]
#[scale_info(bounds(), skip_type_params(T))]
struct A<T> {
    marker: PhantomData<T>,
}

fn main() {}