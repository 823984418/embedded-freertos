use core::ffi::c_void;
use core::ptr::null_mut;

use crate::*;

pub const tmrCOMMAND_EXECUTE_CALLBACK_FROM_ISR: BaseType_t = -2;
pub const tmrCOMMAND_EXECUTE_CALLBACK: BaseType_t = -1;
pub const tmrCOMMAND_START_DONT_TRACE: BaseType_t = 0;
pub const tmrCOMMAND_START: BaseType_t = 1;
pub const tmrCOMMAND_RESET: BaseType_t = 2;
pub const tmrCOMMAND_STOP: BaseType_t = 3;
pub const tmrCOMMAND_CHANGE_PERIOD: BaseType_t = 4;
pub const tmrCOMMAND_DELETE: BaseType_t = 5;

pub const tmrFIRST_FROM_ISR_COMMAND: BaseType_t = 6;
pub const tmrCOMMAND_START_FROM_ISR: BaseType_t = 6;
pub const tmrCOMMAND_RESET_FROM_ISR: BaseType_t = 7;
pub const tmrCOMMAND_STOP_FROM_ISR: BaseType_t = 8;
pub const tmrCOMMAND_CHANGE_PERIOD_FROM_ISR: BaseType_t = 9;

pub unsafe fn xTimerStart(xTimer: TimerHandle_t, xTicksToWait: TickType_t) -> BaseType_t {
    xTimerGenericCommandFromTask(
        xTimer,
        tmrCOMMAND_START,
        xTaskGetTickCount(),
        null_mut(),
        xTicksToWait,
    )
}

pub unsafe fn xTimerStop(xTimer: TimerHandle_t, xTicksToWait: TickType_t) -> BaseType_t {
    xTimerGenericCommandFromTask(xTimer, tmrCOMMAND_STOP, 0, null_mut(), xTicksToWait)
}

pub unsafe fn xTimerChangePeriod(
    xTimer: TimerHandle_t,
    xNewPeriod: TickType_t,
    xTicksToWait: TickType_t,
) -> BaseType_t {
    xTimerGenericCommandFromTask(
        xTimer,
        tmrCOMMAND_CHANGE_PERIOD,
        xNewPeriod,
        null_mut(),
        xTicksToWait,
    )
}

pub unsafe fn xTimerDelete(xTimer: TimerHandle_t, xTicksToWait: TickType_t) -> BaseType_t {
    xTimerGenericCommandFromTask(xTimer, tmrCOMMAND_DELETE, 0, null_mut(), xTicksToWait)
}

pub unsafe fn xTimerReset(xTimer: TimerHandle_t, xTicksToWait: TickType_t) -> BaseType_t {
    xTimerGenericCommandFromTask(
        xTimer,
        tmrCOMMAND_RESET,
        xTaskGetTickCount(),
        null_mut(),
        xTicksToWait,
    )
}

pub unsafe fn xTimerStartFromISR(
    xTimer: TimerHandle_t,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xTimerGenericCommandFromISR(
        xTimer,
        tmrCOMMAND_START_FROM_ISR,
        xTaskGetTickCountFromISR(),
        pxHigherPriorityTaskWoken,
        0,
    )
}

pub unsafe fn xTimerStopFromISR(
    xTimer: TimerHandle_t,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xTimerGenericCommandFromISR(
        xTimer,
        tmrCOMMAND_STOP_FROM_ISR,
        0,
        pxHigherPriorityTaskWoken,
        0,
    )
}

pub unsafe fn xTimerChangePeriodFromISR(
    xTimer: TimerHandle_t,
    xNewPeriod: TickType_t,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xTimerGenericCommandFromISR(
        xTimer,
        tmrCOMMAND_CHANGE_PERIOD_FROM_ISR,
        xNewPeriod,
        pxHigherPriorityTaskWoken,
        0,
    )
}

pub unsafe fn xTimerResetFromISR(
    xTimer: TimerHandle_t,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xTimerGenericCommandFromISR(
        xTimer,
        tmrCOMMAND_RESET_FROM_ISR,
        xTaskGetTickCountFromISR(),
        pxHigherPriorityTaskWoken,
        0,
    )
}
