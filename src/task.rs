use core::ptr::null_mut;

use crate::*;

pub const tskIDLE_PRIORITY: UBaseType_t = 0;
pub const tskNO_AFFINITY: UBaseType_t = UBaseType_t::MAX;

pub const taskSCHEDULER_SUSPENDED: BaseType_t = 0;
pub const taskSCHEDULER_NOT_STARTED: BaseType_t = 1;
pub const taskSCHEDULER_RUNNING: BaseType_t = 2;

pub fn taskVALID_CORE_ID(xCoreID: BaseType_t) -> BaseType_t {
    if 0 <= xCoreID && xCoreID < configNUMBER_OF_CORES as BaseType_t {
        pdTRUE
    } else {
        pdFALSE
    }
}

pub unsafe fn vTaskDelayUntil(pxPreviousWakeTime: *mut TickType_t, xTimeIncrement: TickType_t) {
    xTaskDelayUntil(pxPreviousWakeTime, xTimeIncrement);
}

#[cfg(all(
    configUSE_TRACE_FACILITY = "1",
    configUSE_STATS_FORMATTING_FUNCTIONS = "1",
))]
pub unsafe fn vTaskList(pcWriteBuffer: *mut c_char) {
    vTaskListTasks(pcWriteBuffer, configSTATS_BUFFER_MAX_LENGTH);
}

#[cfg(all(
    configGENERATE_RUN_TIME_STATS = "1",
    configUSE_STATS_FORMATTING_FUNCTIONS = "1",
    configUSE_TRACE_FACILITY = "1",
))]
pub unsafe fn vTaskGetRunTimeStats(pcWriteBuffer: *mut c_char) {
    vTaskGetRunTimeStatistics(pcWriteBuffer, configSTATS_BUFFER_MAX_LENGTH);
}

pub unsafe fn xTaskNotify(
    xTaskToNotify: TaskHandle_t,
    ulValue: u32,
    eAction: eNotifyAction,
) -> BaseType_t {
    xTaskGenericNotify(
        xTaskToNotify,
        tskDEFAULT_INDEX_TO_NOTIFY as UBaseType_t,
        ulValue,
        eAction,
        null_mut(),
    )
}

pub unsafe fn xTaskNotifyIndexed(
    xTaskToNotify: TaskHandle_t,
    uxIndexToNotify: UBaseType_t,
    ulValue: u32,
    eAction: eNotifyAction,
) -> BaseType_t {
    xTaskGenericNotify(xTaskToNotify, uxIndexToNotify, ulValue, eAction, null_mut())
}

pub unsafe fn xTaskNotifyAndQuery(
    xTaskToNotify: TaskHandle_t,
    ulValue: u32,
    eAction: eNotifyAction,
    pulPreviousNotifyValue: *mut u32,
) -> BaseType_t {
    xTaskGenericNotify(
        xTaskToNotify,
        tskDEFAULT_INDEX_TO_NOTIFY as UBaseType_t,
        ulValue,
        eAction,
        pulPreviousNotifyValue,
    )
}

pub unsafe fn xTaskNotifyAndQueryIndexed(
    xTaskToNotify: TaskHandle_t,
    uxIndexToNotify: UBaseType_t,
    ulValue: u32,
    eAction: eNotifyAction,
    pulPreviousNotifyValue: *mut u32,
) -> BaseType_t {
    xTaskGenericNotify(
        xTaskToNotify,
        uxIndexToNotify,
        ulValue,
        eAction,
        pulPreviousNotifyValue,
    )
}

pub unsafe fn xTaskNotifyFromISR(
    xTaskToNotify: TaskHandle_t,
    ulValue: u32,
    eAction: eNotifyAction,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xTaskGenericNotifyFromISR(
        xTaskToNotify,
        tskDEFAULT_INDEX_TO_NOTIFY as UBaseType_t,
        ulValue,
        eAction,
        null_mut(),
        pxHigherPriorityTaskWoken,
    )
}

pub unsafe fn xTaskNotifyIndexedFromISR(
    xTaskToNotify: TaskHandle_t,
    uxIndexToNotify: UBaseType_t,
    ulValue: u32,
    eAction: eNotifyAction,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xTaskGenericNotifyFromISR(
        xTaskToNotify,
        uxIndexToNotify,
        ulValue,
        eAction,
        null_mut(),
        pxHigherPriorityTaskWoken,
    )
}

pub unsafe fn xTaskNotifyAndQueryFromISR(
    xTaskToNotify: TaskHandle_t,
    ulValue: u32,
    eAction: eNotifyAction,
    pulPreviousNotificationValue: *mut u32,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xTaskGenericNotifyFromISR(
        xTaskToNotify,
        tskDEFAULT_INDEX_TO_NOTIFY as UBaseType_t,
        ulValue,
        eAction,
        pulPreviousNotificationValue,
        pxHigherPriorityTaskWoken,
    )
}

pub unsafe fn xTaskNotifyAndQueryIndexedFromISR(
    xTaskToNotify: TaskHandle_t,
    uxIndexToNotify: UBaseType_t,
    ulValue: u32,
    eAction: eNotifyAction,
    pulPreviousNotificationValue: *mut u32,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xTaskGenericNotifyFromISR(
        xTaskToNotify,
        uxIndexToNotify,
        ulValue,
        eAction,
        pulPreviousNotificationValue,
        pxHigherPriorityTaskWoken,
    )
}

pub unsafe fn xTaskNotifyWait(
    ulBitsToClearOnEntry: u32,
    ulBitsToClearOnExit: u32,
    pulNotificationValue: *mut u32,
    xTicksToWait: TickType_t,
) -> BaseType_t {
    xTaskGenericNotifyWait(
        tskDEFAULT_INDEX_TO_NOTIFY as UBaseType_t,
        ulBitsToClearOnEntry,
        ulBitsToClearOnExit,
        pulNotificationValue,
        xTicksToWait,
    )
}

pub unsafe fn xTaskNotifyWaitIndexed(
    uxIndexToWaitOn: UBaseType_t,
    ulBitsToClearOnEntry: u32,
    ulBitsToClearOnExit: u32,
    pulNotificationValue: *mut u32,
    xTicksToWait: TickType_t,
) -> BaseType_t {
    xTaskGenericNotifyWait(
        uxIndexToWaitOn,
        ulBitsToClearOnEntry,
        ulBitsToClearOnExit,
        pulNotificationValue,
        xTicksToWait,
    )
}

pub unsafe fn xTaskNotifyGive(xTaskToNotify: TaskHandle_t) -> BaseType_t {
    xTaskGenericNotify(
        xTaskToNotify,
        tskDEFAULT_INDEX_TO_NOTIFY as UBaseType_t,
        0,
        eIncrement,
        null_mut(),
    )
}

pub unsafe fn xTaskNotifyGiveIndexed(
    xTaskToNotify: TaskHandle_t,
    uxIndexToNotify: UBaseType_t,
) -> BaseType_t {
    xTaskGenericNotify(xTaskToNotify, uxIndexToNotify, 0, eIncrement, null_mut())
}

pub unsafe fn vTaskNotifyGiveFromISR(
    xTaskToNotify: TaskHandle_t,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) {
    vTaskGenericNotifyGiveFromISR(
        xTaskToNotify,
        tskDEFAULT_INDEX_TO_NOTIFY as UBaseType_t,
        pxHigherPriorityTaskWoken,
    )
}

pub unsafe fn vTaskNotifyGiveIndexedFromISR(
    xTaskToNotify: TaskHandle_t,
    uxIndexToNotify: UBaseType_t,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) {
    vTaskGenericNotifyGiveFromISR(xTaskToNotify, uxIndexToNotify, pxHigherPriorityTaskWoken)
}

pub unsafe fn ulTaskNotifyTake(xClearCountOnExit: BaseType_t, xTicksToWait: TickType_t) -> u32 {
    ulTaskGenericNotifyTake(
        tskDEFAULT_INDEX_TO_NOTIFY as UBaseType_t,
        xClearCountOnExit,
        xTicksToWait,
    )
}

pub unsafe fn ulTaskNotifyTakeIndexed(
    uxIndexToWaitOn: UBaseType_t,
    xClearCountOnExit: BaseType_t,
    xTicksToWait: TickType_t,
) -> u32 {
    ulTaskGenericNotifyTake(uxIndexToWaitOn, xClearCountOnExit, xTicksToWait)
}

pub unsafe fn xTaskNotifyStateClear(xTask: TaskHandle_t) -> BaseType_t {
    xTaskGenericNotifyStateClear(xTask, tskDEFAULT_INDEX_TO_NOTIFY as UBaseType_t)
}

pub unsafe fn xTaskNotifyStateClearIndexed(
    xTask: TaskHandle_t,
    uxIndexToClear: UBaseType_t,
) -> BaseType_t {
    xTaskGenericNotifyStateClear(xTask, uxIndexToClear)
}

pub unsafe fn ulTaskNotifyValueClear(xTask: TaskHandle_t, ulBitsToClear: u32) -> u32 {
    ulTaskGenericNotifyValueClear(
        xTask,
        tskDEFAULT_INDEX_TO_NOTIFY as UBaseType_t,
        ulBitsToClear,
    )
}

pub unsafe fn ulTaskNotifyValueClearIndexed(
    xTask: TaskHandle_t,
    uxIndexToClear: UBaseType_t,
    ulBitsToClear: u32,
) -> u32 {
    ulTaskGenericNotifyValueClear(xTask, uxIndexToClear, ulBitsToClear)
}
