use crate::*;
use core::ffi::c_void;

pub const queueSEND_TO_BACK: BaseType_t = 0;
pub const queueSEND_TO_FRONT: BaseType_t = 1;
pub const queueOVERWRITE: BaseType_t = 2;

pub const queueQUEUE_TYPE_BASE: u8 = 0;
pub const queueQUEUE_TYPE_MUTEX: u8 = 1;
pub const queueQUEUE_TYPE_COUNTING_SEMAPHORE: u8 = 2;
pub const queueQUEUE_TYPE_BINARY_SEMAPHORE: u8 = 3;
pub const queueQUEUE_TYPE_RECURSIVE_MUTEX: u8 = 4;
pub const queueQUEUE_TYPE_SET: u8 = 5;

#[cfg(configSUPPORT_DYNAMIC_ALLOCATION = "1")]
pub unsafe fn xQueueCreate(uxQueueLength: UBaseType_t, uxItemSize: UBaseType_t) -> QueueHandle_t {
    xQueueGenericCreate(uxQueueLength, uxItemSize, queueQUEUE_TYPE_BASE)
}

#[cfg(configSUPPORT_STATIC_ALLOCATION = "1")]
pub unsafe fn xQueueCreateStatic(
    uxQueueLength: UBaseType_t,
    uxItemSize: UBaseType_t,
    pucQueueStorage: *mut u8,
    pxQueueBuffer: *mut StaticQueue_t,
) -> QueueHandle_t {
    xQueueGenericCreateStatic(
        uxQueueLength,
        uxItemSize,
        pucQueueStorage,
        pxQueueBuffer,
        queueQUEUE_TYPE_BASE,
    )
}

#[cfg(configSUPPORT_STATIC_ALLOCATION = "1")]
pub unsafe fn xQueueGetStaticBuffers(
    xQueue: QueueHandle_t,
    ppucQueueStorage: *mut *mut u8,
    ppxStaticQueue: *mut *mut StaticQueue_t,
) -> BaseType_t {
    xQueueGenericGetStaticBuffers(xQueue, ppucQueueStorage, ppxStaticQueue)
}

pub unsafe fn xQueueSendToFront(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    xTicksToWait: TickType_t,
) -> BaseType_t {
    xQueueGenericSend(xQueue, pvItemToQueue, xTicksToWait, queueSEND_TO_FRONT)
}

pub unsafe fn xQueueSendToBack(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    xTicksToWait: TickType_t,
) -> BaseType_t {
    xQueueGenericSend(xQueue, pvItemToQueue, xTicksToWait, queueSEND_TO_BACK)
}

pub unsafe fn xQueueSend(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    xTicksToWait: TickType_t,
) -> BaseType_t {
    xQueueGenericSend(xQueue, pvItemToQueue, xTicksToWait, queueSEND_TO_BACK)
}

pub unsafe fn xQueueOverwrite(xQueue: QueueHandle_t, pvItemToQueue: *const c_void) -> BaseType_t {
    xQueueGenericSend(xQueue, pvItemToQueue, 0, queueOVERWRITE)
}

pub unsafe fn xQueueSendToFrontFromISR(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xQueueGenericSendFromISR(
        xQueue,
        pvItemToQueue,
        pxHigherPriorityTaskWoken,
        queueSEND_TO_FRONT,
    )
}

pub unsafe fn xQueueSendToBackFromISR(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xQueueGenericSendFromISR(
        xQueue,
        pvItemToQueue,
        pxHigherPriorityTaskWoken,
        queueSEND_TO_BACK,
    )
}

pub unsafe fn xQueueOverwriteFromISR(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xQueueGenericSendFromISR(
        xQueue,
        pvItemToQueue,
        pxHigherPriorityTaskWoken,
        queueOVERWRITE,
    )
}

pub unsafe fn xQueueSendFromISR(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xQueueGenericSendFromISR(
        xQueue,
        pvItemToQueue,
        pxHigherPriorityTaskWoken,
        queueSEND_TO_BACK,
    )
}

// TODO: xQueueCRSendFromISR
// TODO: xQueueCRReceiveFromISR
// TODO: xQueueCRSend
// TODO: xQueueCRReceive
// TODO: xQueueCreateMutexStatic
// TODO: xQueueCreateCountingSemaphore
// TODO: xQueueCreateCountingSemaphoreStatic
// TODO: xQueueGetMutexHolder
// TODO: xQueueGetMutexHolderFromISR
// TODO: xQueueReset
