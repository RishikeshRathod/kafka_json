use serde::{Deserialize, Serialize};
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

use crate::types::CharArray;

#[derive(Serialize, Deserialize, Debug, AsBytes, FromBytes, FromZeroes)]
#[repr(C, packed)]
pub struct PingPayload {
    operation_code: u32,
    name: CharArray<10>,
    email: CharArray<10>,
    order_id: u32,
    amount: u32,
}

#[derive(Serialize, Deserialize, Debug, AsBytes, FromBytes, FromZeroes)]
#[repr(C, packed)]
pub struct PongPayload {
    laptop_id: u32,
    model: CharArray<10>,
    color: CharArray<10>,
    price: u32,
}
