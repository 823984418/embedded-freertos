use core::ptr::null_mut;

use crate::*;

#[cfg(configSUPPORT_DYNAMIC_ALLOCATION = "1")]
pub unsafe fn xSemaphoreCreateBinary() -> SemaphoreHandle_t {
    xQueueGenericCreate(
        1,
        semSEMAPHORE_QUEUE_ITEM_LENGTH,
        queueQUEUE_TYPE_BINARY_SEMAPHORE,
    )
}

#[cfg(configSUPPORT_STATIC_ALLOCATION = "1")]
pub unsafe fn xSemaphoreCreateBinaryStatic(
    pxStaticSemaphore: *mut StaticSemaphore_t,
) -> SemaphoreHandle_t {
    xQueueGenericCreateStatic(
        1,
        semSEMAPHORE_QUEUE_ITEM_LENGTH,
        null_mut(),
        pxStaticSemaphore,
        queueQUEUE_TYPE_BINARY_SEMAPHORE,
    )
}

pub unsafe fn xSemaphoreTake(xSemaphore: SemaphoreHandle_t, xBlockTime: TickType_t) -> BaseType_t {
    xQueueSemaphoreTake(xSemaphore, xBlockTime)
}

#[cfg(configUSE_RECURSIVE_MUTEXES = "1")]
pub unsafe fn xSemaphoreGiveRecursive(xMutex: SemaphoreHandle_t) -> BaseType_t {
    xQueueGiveMutexRecursive(xMutex)
}

pub unsafe fn xSemaphoreGiveFromISR(
    xSemaphore: SemaphoreHandle_t,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xQueueGiveFromISR(xSemaphore, pxHigherPriorityTaskWoken)
}

pub unsafe fn xSemaphoreTakeFromISR(
    xSemaphore: SemaphoreHandle_t,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xQueueReceiveFromISR(xSemaphore, null_mut(), pxHigherPriorityTaskWoken)
}

#[cfg(all(configSUPPORT_DYNAMIC_ALLOCATION = "1", configUSE_MUTEXES = "1"))]
pub unsafe fn xSemaphoreCreateMutex() -> SemaphoreHandle_t {
    xQueueCreateMutex(queueQUEUE_TYPE_MUTEX)
}

#[cfg(all(configSUPPORT_STATIC_ALLOCATION = "1", configUSE_MUTEXES = "1"))]
pub unsafe fn xSemaphoreCreateMutexStatic(
    pxMutexBuffer: *mut StaticSemaphore_t,
) -> SemaphoreHandle_t {
    xQueueCreateMutexStatic(queueQUEUE_TYPE_MUTEX, pxMutexBuffer)
}

#[cfg(all(
    configSUPPORT_DYNAMIC_ALLOCATION = "1",
    configUSE_RECURSIVE_MUTEXES = "1",
))]
pub unsafe fn xSemaphoreCreateRecursiveMutex() -> SemaphoreHandle_t {
    xQueueCreateMutex(queueQUEUE_TYPE_RECURSIVE_MUTEX)
}

#[cfg(all(
    configSUPPORT_STATIC_ALLOCATION = "1",
    configUSE_RECURSIVE_MUTEXES = "1",
))]
pub unsafe fn xSemaphoreCreateRecursiveMutexStatic(
    pxStaticSemaphore: *mut StaticSemaphore_t,
) -> SemaphoreHandle_t {
    xQueueCreateMutexStatic(queueQUEUE_TYPE_RECURSIVE_MUTEX, pxStaticSemaphore)
}

#[cfg(configSUPPORT_DYNAMIC_ALLOCATION = "1")]
pub unsafe fn xSemaphoreCreateCounting(
    uxMaxCount: UBaseType_t,
    uxInitialCount: UBaseType_t,
) -> SemaphoreHandle_t {
    xQueueCreateCountingSemaphore(uxMaxCount, uxInitialCount)
}

#[cfg(configSUPPORT_STATIC_ALLOCATION = "1")]
pub unsafe fn xSemaphoreCreateCountingStatic(
    uxMaxCount: UBaseType_t,
    uxInitialCount: UBaseType_t,
    pxSemaphoreBuffer: *mut StaticSemaphore_t,
) -> SemaphoreHandle_t {
    xQueueCreateCountingSemaphoreStatic(uxMaxCount, uxInitialCount, pxSemaphoreBuffer)
}

pub unsafe fn vSemaphoreDelete(xSemaphore: SemaphoreHandle_t) {
    vQueueDelete(xSemaphore as QueueHandle_t)
}

#[cfg(all(configUSE_MUTEXES = "1", INCLUDE_xSemaphoreGetMutexHolder = "1",))]
pub unsafe fn xSemaphoreGetMutexHolder(xMutex: SemaphoreHandle_t) -> TaskHandle_t {
    xQueueGetMutexHolder(xMutex)
}

#[cfg(all(configUSE_MUTEXES = "1", INCLUDE_xSemaphoreGetMutexHolder = "1"))]
pub unsafe fn xSemaphoreGetMutexHolderFromISR(xMutex: SemaphoreHandle_t) -> TaskHandle_t {
    xQueueGetMutexHolderFromISR(xMutex)
}

pub unsafe fn uxSemaphoreGetCount(xSemaphore: SemaphoreHandle_t) -> UBaseType_t {
    uxQueueMessagesWaiting(xSemaphore)
}

pub unsafe fn uxSemaphoreGetCountFromISR(xSemaphore: SemaphoreHandle_t) -> UBaseType_t {
    uxQueueMessagesWaitingFromISR(xSemaphore)
}

#[cfg(configSUPPORT_STATIC_ALLOCATION = "1")]
pub unsafe fn xSemaphoreGetStaticBuffer(
    xSemaphore: SemaphoreHandle_t,
    ppxSemaphoreBuffer: *mut *mut StaticSemaphore_t,
) -> BaseType_t {
    xQueueGenericGetStaticBuffers(xSemaphore, null_mut(), ppxSemaphoreBuffer)
}
