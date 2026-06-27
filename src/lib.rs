#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unpredictable_function_pointer_comparisons)]

pub const fn rsMS_TO_TICKS(xTimeInMs: u64) -> TickType_t {
    (xTimeInMs * configTICK_RATE_HZ as u64 / 1000) as TickType_t
}

pub const fn rsTICKS_TO_MS(xTimeInTicks: TickType_t) -> u64 {
    xTimeInTicks as u64 * 1000 / configTICK_RATE_HZ as u64
}

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

include!(concat!(env!("OUT_DIR"), "/bindgen.rs"));
