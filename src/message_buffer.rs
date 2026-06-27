use core::ffi::c_void;

use crate::*;

#[cfg(configSUPPORT_DYNAMIC_ALLOCATION = "1")]
pub unsafe fn xMessageBufferCreate(xBufferSizeBytes: usize) -> MessageBufferHandle_t {
    xStreamBufferGenericCreate(xBufferSizeBytes, 0, sbTYPE_MESSAGE_BUFFER, None, None)
}

#[cfg(all(
    configSUPPORT_DYNAMIC_ALLOCATION = "1",
    configUSE_SB_COMPLETED_CALLBACK = "1"
))]
pub unsafe fn xMessageBufferCreateWithCallback(
    xBufferSizeBytes: usize,
    pxSendCompletedCallback: StreamBufferCallbackFunction_t,
    pxReceiveCompletedCallback: StreamBufferCallbackFunction_t,
) -> MessageBufferHandle_t {
    xStreamBufferGenericCreate(
        xBufferSizeBytes,
        0,
        sbTYPE_MESSAGE_BUFFER,
        pxSendCompletedCallback,
        pxReceiveCompletedCallback,
    )
}

#[cfg(configSUPPORT_STATIC_ALLOCATION = "1")]
pub unsafe fn xMessageBufferCreateStatic(
    xBufferSizeBytes: usize,
    pucMessageBufferStorageArea: *mut u8,
    pxStaticMessageBuffer: *mut StaticMessageBuffer_t,
) -> MessageBufferHandle_t {
    xStreamBufferGenericCreateStatic(
        xBufferSizeBytes,
        0,
        sbTYPE_MESSAGE_BUFFER,
        pucMessageBufferStorageArea,
        pxStaticMessageBuffer.cast(),
        None,
        None,
    )
}

#[cfg(all(
    configSUPPORT_STATIC_ALLOCATION = "1",
    configUSE_SB_COMPLETED_CALLBACK = "1"
))]
pub unsafe fn xMessageBufferCreateStaticWithCallback(
    xBufferSizeBytes: usize,
    pucMessageBufferStorageArea: *mut u8,
    pxStaticMessageBuffer: *mut StaticMessageBuffer_t,
    pxSendCompletedCallback: StreamBufferCallbackFunction_t,
    pxReceiveCompletedCallback: StreamBufferCallbackFunction_t,
) -> MessageBufferHandle_t {
    xStreamBufferGenericCreateStatic(
        xBufferSizeBytes,
        0,
        sbTYPE_MESSAGE_BUFFER,
        pucMessageBufferStorageArea,
        pxStaticMessageBuffer.cast(),
        pxSendCompletedCallback,
        pxReceiveCompletedCallback,
    )
}

#[cfg(configSUPPORT_STATIC_ALLOCATION = "1")]
pub unsafe fn xMessageBufferGetStaticBuffers(
    xMessageBuffer: MessageBufferHandle_t,
    ppucMessageBufferStorageArea: *mut *mut u8,
    ppxStaticMessageBuffer: *mut *mut StaticMessageBuffer_t,
) -> BaseType_t {
    xStreamBufferGetStaticBuffers(
        xMessageBuffer,
        ppucMessageBufferStorageArea,
        ppxStaticMessageBuffer,
    )
}

pub unsafe fn xMessageBufferSend(
    xMessageBuffer: MessageBufferHandle_t,
    pvTxData: *const c_void,
    xDataLengthBytes: usize,
    xTicksToWait: TickType_t,
) -> usize {
    xStreamBufferSend(xMessageBuffer, pvTxData, xDataLengthBytes, xTicksToWait)
}

pub unsafe fn xMessageBufferSendFromISR(
    xMessageBuffer: MessageBufferHandle_t,
    pvTxData: *const c_void,
    xDataLengthBytes: usize,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> usize {
    xStreamBufferSendFromISR(
        xMessageBuffer,
        pvTxData,
        xDataLengthBytes,
        pxHigherPriorityTaskWoken,
    )
}

pub unsafe fn xMessageBufferReceive(
    xMessageBuffer: MessageBufferHandle_t,
    pvRxData: *mut c_void,
    xBufferLengthBytes: usize,
    xTicksToWait: TickType_t,
) -> usize {
    xStreamBufferReceive(xMessageBuffer, pvRxData, xBufferLengthBytes, xTicksToWait)
}

pub unsafe fn xMessageBufferReceiveFromISR(
    xMessageBuffer: MessageBufferHandle_t,
    pvRxData: *mut c_void,
    xBufferLengthBytes: usize,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> usize {
    xStreamBufferReceiveFromISR(
        xMessageBuffer,
        pvRxData,
        xBufferLengthBytes,
        pxHigherPriorityTaskWoken,
    )
}

pub unsafe fn vMessageBufferDelete(xMessageBuffer: MessageBufferHandle_t) {
    vStreamBufferDelete(xMessageBuffer)
}

pub unsafe fn xMessageBufferIsFull(xMessageBuffer: MessageBufferHandle_t) -> BaseType_t {
    xStreamBufferIsFull(xMessageBuffer)
}

pub unsafe fn xMessageBufferIsEmpty(xMessageBuffer: MessageBufferHandle_t) -> BaseType_t {
    xStreamBufferIsEmpty(xMessageBuffer)
}

pub unsafe fn xMessageBufferReset(xMessageBuffer: MessageBufferHandle_t) -> BaseType_t {
    xStreamBufferReset(xMessageBuffer)
}

pub unsafe fn xMessageBufferResetFromISR(xMessageBuffer: MessageBufferHandle_t) -> BaseType_t {
    xStreamBufferResetFromISR(xMessageBuffer)
}

pub unsafe fn xMessageBufferSpaceAvailable(xMessageBuffer: MessageBufferHandle_t) -> usize {
    xStreamBufferSpacesAvailable(xMessageBuffer)
}

pub unsafe fn xMessageBufferSpacesAvailable(xMessageBuffer: MessageBufferHandle_t) -> usize {
    xStreamBufferSpacesAvailable(xMessageBuffer)
}

pub unsafe fn xMessageBufferNextLengthBytes(xMessageBuffer: MessageBufferHandle_t) -> usize {
    xStreamBufferNextMessageLengthBytes(xMessageBuffer)
}

pub unsafe fn xMessageBufferSendCompletedFromISR(
    xMessageBuffer: MessageBufferHandle_t,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xStreamBufferSendCompletedFromISR(xMessageBuffer, pxHigherPriorityTaskWoken)
}

pub unsafe fn xMessageBufferReceiveCompletedFromISR(
    xMessageBuffer: MessageBufferHandle_t,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xStreamBufferReceiveCompletedFromISR(xMessageBuffer, pxHigherPriorityTaskWoken)
}
