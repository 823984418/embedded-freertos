use core::ffi::c_void;

use crate::*;

pub const sbTYPE_STREAM_BUFFER: BaseType_t = 0;
pub const sbTYPE_MESSAGE_BUFFER: BaseType_t = 1;
pub const sbTYPE_STREAM_BATCHING_BUFFER: BaseType_t = 2;

#[cfg(configSUPPORT_DYNAMIC_ALLOCATION = "1")]
pub unsafe fn xStreamBufferCreate(
    xBufferSizeBytes: usize,
    xTriggerLevelBytes: usize,
) -> StreamBufferHandle_t {
    xStreamBufferGenericCreate(
        xBufferSizeBytes,
        xTriggerLevelBytes,
        sbTYPE_STREAM_BUFFER,
        None,
        None,
    )
}

#[cfg(all(
    configSUPPORT_DYNAMIC_ALLOCATION = "1",
    configUSE_SB_COMPLETED_CALLBACK = "1",
))]
pub unsafe fn xStreamBufferCreateWithCallback(
    xBufferSizeBytes: usize,
    xTriggerLevelBytes: usize,
    pxSendCompletedCallback: StreamBufferCallbackFunction_t,
    pxReceiveCompletedCallback: StreamBufferCallbackFunction_t,
) -> StreamBufferHandle_t {
    xStreamBufferGenericCreate(
        xBufferSizeBytes,
        xTriggerLevelBytes,
        sbTYPE_STREAM_BUFFER,
        pxSendCompletedCallback,
        pxReceiveCompletedCallback,
    )
}

#[cfg(configSUPPORT_STATIC_ALLOCATION = "1")]
pub unsafe fn xStreamBufferCreateStatic(
    xBufferSizeBytes: usize,
    xTriggerLevelBytes: usize,
    pucStreamBufferStorageArea: *mut u8,
    pxStaticStreamBuffer: *mut StaticStreamBuffer_t,
) -> StreamBufferHandle_t {
    xStreamBufferGenericCreateStatic(
        xBufferSizeBytes,
        xTriggerLevelBytes,
        sbTYPE_STREAM_BUFFER,
        pucStreamBufferStorageArea,
        pxStaticStreamBuffer,
        None,
        None,
    )
}

#[cfg(all(
    configSUPPORT_STATIC_ALLOCATION = "1",
    configUSE_SB_COMPLETED_CALLBACK = "1"
))]
pub unsafe fn xStreamBufferCreateStaticWithCallback(
    xBufferSizeBytes: usize,
    xTriggerLevelBytes: usize,
    pucStreamBufferStorageArea: *mut u8,
    pxStaticStreamBuffer: *mut StaticStreamBuffer_t,
    pxSendCompletedCallback: StreamBufferCallbackFunction_t,
    pxReceiveCompletedCallback: StreamBufferCallbackFunction_t,
) -> StreamBufferHandle_t {
    xStreamBufferGenericCreateStatic(
        xBufferSizeBytes,
        xTriggerLevelBytes,
        sbTYPE_STREAM_BUFFER,
        pucStreamBufferStorageArea,
        pxStaticStreamBuffer,
        pxSendCompletedCallback,
        pxReceiveCompletedCallback,
    )
}

#[cfg(configSUPPORT_DYNAMIC_ALLOCATION = "1")]
pub unsafe fn xStreamBatchingBufferCreate(
    xBufferSizeBytes: usize,
    xTriggerLevelBytes: usize,
) -> StreamBufferHandle_t {
    xStreamBufferGenericCreate(
        xBufferSizeBytes,
        xTriggerLevelBytes,
        sbTYPE_STREAM_BATCHING_BUFFER,
        None,
        None,
    )
}

#[cfg(all(
    configSUPPORT_DYNAMIC_ALLOCATION = "1",
    configUSE_SB_COMPLETED_CALLBACK = "1"
))]
pub unsafe fn xStreamBatchingBufferCreateWithCallback(
    xBufferSizeBytes: usize,
    xTriggerLevelBytes: usize,
    pxSendCompletedCallback: StreamBufferCallbackFunction_t,
    pxReceiveCompletedCallback: StreamBufferCallbackFunction_t,
) -> StreamBufferHandle_t {
    xStreamBufferGenericCreate(
        xBufferSizeBytes,
        xTriggerLevelBytes,
        sbTYPE_STREAM_BATCHING_BUFFER,
        pxSendCompletedCallback,
        pxReceiveCompletedCallback,
    )
}

#[cfg(configSUPPORT_STATIC_ALLOCATION = "1")]
pub unsafe fn xStreamBatchingBufferCreateStatic(
    xBufferSizeBytes: usize,
    xTriggerLevelBytes: usize,
    pucStreamBufferStorageArea: *mut u8,
    pxStaticStreamBuffer: *mut StaticStreamBuffer_t,
) -> StreamBufferHandle_t {
    xStreamBufferGenericCreateStatic(
        xBufferSizeBytes,
        xTriggerLevelBytes,
        sbTYPE_STREAM_BATCHING_BUFFER,
        pucStreamBufferStorageArea,
        pxStaticStreamBuffer,
        None,
        None,
    )
}

#[cfg(all(
    configSUPPORT_STATIC_ALLOCATION = "1",
    configUSE_SB_COMPLETED_CALLBACK = "1",
))]
pub unsafe fn xStreamBatchingBufferCreateStaticWithCallback(
    xBufferSizeBytes: usize,
    xTriggerLevelBytes: usize,
    pucStreamBufferStorageArea: *mut u8,
    pxStaticStreamBuffer: *mut StaticStreamBuffer_t,
    pxSendCompletedCallback: StreamBufferCallbackFunction_t,
    pxReceiveCompletedCallback: StreamBufferCallbackFunction_t,
) -> StreamBufferHandle_t {
    xStreamBufferGenericCreateStatic(
        xBufferSizeBytes,
        xTriggerLevelBytes,
        sbTYPE_STREAM_BATCHING_BUFFER,
        pucStreamBufferStorageArea,
        pxStaticStreamBuffer,
        pxSendCompletedCallback,
        pxReceiveCompletedCallback,
    )
}
