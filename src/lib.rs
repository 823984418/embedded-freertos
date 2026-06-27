#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(non_camel_case_types)]
#![allow(unexpected_cfgs)]
#![allow(unpredictable_function_pointer_comparisons)]

mod bindgen;
mod list;
mod message_buffer;
mod queue;
mod semphr;
mod stream_buffer;
mod task;

pub use bindgen::*;
pub use list::*;
pub use message_buffer::*;
pub use queue::*;
pub use semphr::*;
pub use stream_buffer::*;
pub use task::*;

pub const pdFALSE: BaseType_t = 0;
pub const pdTRUE: BaseType_t = 1;
pub const pdFALSE_SIGNED: BaseType_t = 0;
pub const pdTRUE_SIGNED: BaseType_t = 1;
pub const pdFALSE_UNSIGNED: UBaseType_t = 0;
pub const pdTRUE_UNSIGNED: UBaseType_t = 1;

pub const pdPASS: BaseType_t = pdTRUE;
pub const pdFAIL: BaseType_t = pdFALSE;

pub const errQUEUE_EMPTY: BaseType_t = 0;
pub const errQUEUE_FULL: BaseType_t = 0;

pub const semBINARY_SEMAPHORE_QUEUE_LENGTH: UBaseType_t = 1;
pub const semSEMAPHORE_QUEUE_ITEM_LENGTH: UBaseType_t = 0;
pub const semGIVE_BLOCK_TIME: TickType_t = 0;

pub const fn rsMS_TO_TICKS(xTimeInMs: u64) -> TickType_t {
    (xTimeInMs * configTICK_RATE_HZ as u64 / 1000) as TickType_t
}

pub const fn rsTICKS_TO_MS(xTimeInTicks: TickType_t) -> u64 {
    xTimeInTicks as u64 * 1000 / configTICK_RATE_HZ as u64
}
