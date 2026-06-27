#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(non_camel_case_types)]
#![allow(unexpected_cfgs)]
#![allow(unpredictable_function_pointer_comparisons)]

use core::ffi::c_void;
use core::ptr::null_mut;

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

pub unsafe fn listSET_LIST_ITEM_OWNER<T>(pxListItem: *mut ListItem_t, pxOwner: *mut T) {
    (*pxListItem).pvOwner = pxOwner as _;
}

pub unsafe fn listGET_LIST_ITEM_OWNER(pxListItem: *mut ListItem_t) -> *mut c_void {
    (*pxListItem).pvOwner
}

pub unsafe fn listSET_LIST_ITEM_VALUE(pxListItem: *mut ListItem_t, xValue: TickType_t) {
    (*pxListItem).xItemValue = xValue;
}

pub unsafe fn listGET_LIST_ITEM_VALUE(pxListItem: *mut ListItem_t) -> TickType_t {
    (*pxListItem).xItemValue
}

pub unsafe fn listGET_ITEM_VALUE_OF_HEAD_ENTRY(pxList: *mut List_t) -> TickType_t {
    (*(*pxList).xListEnd.pxNext).xItemValue
}

pub unsafe fn listGET_HEAD_ENTRY(pxList: *mut List_t) -> *mut ListItem_t {
    (*pxList).xListEnd.pxNext
}

pub unsafe fn listGET_NEXT(pxListItem: *mut ListItem_t) -> *mut ListItem_t {
    (*pxListItem).pxNext
}

pub unsafe fn listGET_END_MARKER(pxList: *mut List_t) -> *const ListItem_t {
    &raw const (*pxList).xListEnd as *const ListItem_t
}

pub unsafe fn listLIST_IS_EMPTY(pxList: *mut List_t) -> BaseType_t {
    if (*pxList).uxNumberOfItems == 0 {
        pdTRUE
    } else {
        pdFALSE
    }
}

pub unsafe fn listCURRENT_LIST_LENGTH(pxList: *mut List_t) -> UBaseType_t {
    (*pxList).uxNumberOfItems
}

#[cfg(configNUMBER_OF_CORES = "1")]
pub unsafe fn listGET_OWNER_OF_NEXT_ENTRY(pxList: *mut List_t) -> TaskHandle_t {
    let pxConstList = pxList;
    (*pxConstList).pxIndex = (*(*pxConstList).pxIndex).pxNext;
    if core::ptr::addr_eq((*pxConstList).pxIndex, &raw mut (*pxConstList).xListEnd) {
        (*pxConstList).pxIndex = (*pxConstList).xListEnd.pxNext;
    }
    (*(*pxConstList).pxIndex).pvOwner as TaskHandle_t
}

pub unsafe fn listREMOVE_ITEM(pxItemToRemove: *mut ListItem_t) {
    let pxList = (*pxItemToRemove).pxContainer;
    (*(*pxItemToRemove).pxNext).pxPrevious = (*pxItemToRemove).pxPrevious;
    (*(*pxItemToRemove).pxPrevious).pxNext = (*pxItemToRemove).pxNext;
    if (*pxList).pxIndex == pxItemToRemove {
        (*pxList).pxIndex = (*pxItemToRemove).pxPrevious;
    }
    (*pxItemToRemove).pxContainer = null_mut();
    (*pxList).uxNumberOfItems = (*pxList).uxNumberOfItems - 1;
}

pub unsafe fn listINSERT_END(pxList: *mut List_t, pxNewListItem: *mut ListItem_t) {
    let pxIndex = (*pxList).pxIndex;

    // FIXME: listTEST_LIST_INTEGRITY(pxList)
    // FIXME: listTEST_LIST_ITEM_INTEGRITY(pxNewListItem)

    (*pxNewListItem).pxNext = pxIndex;
    (*pxNewListItem).pxPrevious = (*pxIndex).pxPrevious;

    (*(*pxIndex).pxPrevious).pxNext = pxNewListItem;
    (*pxIndex).pxPrevious = pxNewListItem;

    (*pxNewListItem).pxContainer = pxList;
    (*pxList).uxNumberOfItems = (*pxList).uxNumberOfItems + 1;
}

pub unsafe fn listGET_OWNER_OF_HEAD_ENTRY(pxList: *mut List_t) -> *mut c_void {
    (*(*pxList).xListEnd.pxNext).pvOwner
}

pub unsafe fn listIS_CONTAINED_WITHIN(
    pxList: *mut List_t,
    pxNewListItem: *mut ListItem_t,
) -> BaseType_t {
    if (*pxNewListItem).pxContainer == pxList {
        pdTRUE
    } else {
        pdFALSE
    }
}

pub unsafe fn listLIST_ITEM_CONTAINER(pxNewListItem: *mut ListItem_t) -> *mut List_t {
    (*pxNewListItem).pxContainer
}

pub unsafe fn listLIST_IS_INITIALISED(pxList: *mut List_t) -> bool {
    (*pxList).xListEnd.xItemValue == TickType_t::MAX
}

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

// TODO: xMessageBufferCreate
// TODO: xMessageBufferCreateWithCallback
// TODO: xMessageBufferCreateStatic
// TODO: xMessageBufferCreateStaticWithCallback
// TODO: xMessageBufferGetStaticBuffers
// TODO: xMessageBufferSend
// TODO: xMessageBufferSendFromISR
// TODO: xMessageBufferReceive
// TODO: xMessageBufferReceiveFromISR
// TODO: vMessageBufferDelete
// TODO: xMessageBufferIsFull
// TODO: xMessageBufferIsEmpty
// TODO: xMessageBufferReset
// TODO: xMessageBufferResetFromISR
// TODO: xMessageBufferSpaceAvailable
// TODO: xMessageBufferSpacesAvailable
// TODO: xMessageBufferNextLengthBytes
// TODO: xMessageBufferSendCompletedFromISR
// TODO: xMessageBufferReceiveCompletedFromISR

pub const errQUEUE_EMPTY: BaseType_t = 0;
pub const errQUEUE_FULL: BaseType_t = 0;

pub const semBINARY_SEMAPHORE_QUEUE_LENGTH: UBaseType_t = 1;
pub const semSEMAPHORE_QUEUE_ITEM_LENGTH: UBaseType_t = 0;
pub const semGIVE_BLOCK_TIME: TickType_t = 0;

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

// TODO: xSemaphoreCreateCountingStatic
// TODO: vSemaphoreDelete
// TODO: xSemaphoreGetMutexHolder
// TODO: xSemaphoreGetMutexHolderFromISR
// TODO: uxSemaphoreGetCount
// TODO: uxSemaphoreGetCountFromISR
// TODO: xSemaphoreGetStaticBuffer

pub const sbTYPE_STREAM_BUFFER: BaseType_t = 0;
pub const sbTYPE_MESSAGE_BUFFER: BaseType_t = 1;
pub const sbTYPE_STREAM_BATCHING_BUFFER: BaseType_t = 2;

// TODO: xStreamBufferCreate
// TODO: xStreamBufferCreateWithCallback
// TODO: xStreamBufferCreateStatic
// TODO: xStreamBufferCreateStaticWithCallback
// TODO: xStreamBatchingBufferCreate
// TODO: xStreamBatchingBufferCreateWithCallback
// TODO: xStreamBatchingBufferCreateStatic
// TODO: xStreamBatchingBufferCreateStaticWithCallback

include!(concat!(env!("OUT_DIR"), "/bindgen.rs"));
