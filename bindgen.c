#include "FreeRTOS.h"

#include "portmacro.h"

#include "croutine.h"
#include "event_groups.h"
#include "list.h"
#include "message_buffer.h"
#include "portable.h"
#include "queue.h"
#include "semphr.h"
#include "stack_macros.h"
#include "stream_buffer.h"
#include "task.h"
#include "timers.h"

TickType_t pdMS_TO_TICKS_extern(uint64_t xTimeInMs) {
    return pdMS_TO_TICKS(xTimeInMs);
}

uint64_t pdTICKS_TO_MS_extern(TickType_t xTimeInTicks) {
    return pdTICKS_TO_MS(xTimeInTicks);
}

QueueHandle_t xSemaphoreCreateBinary_extern(void) {
    return xSemaphoreCreateBinary();
}

QueueHandle_t xSemaphoreCreateBinaryStatic_extern(StaticQueue_t *pxStaticSemaphore) {
    return xSemaphoreCreateBinaryStatic(pxStaticSemaphore);
}

BaseType_t xSemaphoreTake_extern(SemaphoreHandle_t xSemaphore, TickType_t xBlockTime) {
    return xSemaphoreTake(xSemaphore, xBlockTime);
}

BaseType_t xSemaphoreTakeRecursive_extern(QueueHandle_t xMutex, TickType_t xBlockTime) {
    return xSemaphoreTakeRecursive(xMutex, xBlockTime);
}

BaseType_t xSemaphoreGive_extern(SemaphoreHandle_t xSemaphore) {
    return xSemaphoreGive(xSemaphore);
}

BaseType_t xSemaphoreGiveRecursive_extern(QueueHandle_t xMutex) {
    return xSemaphoreGiveRecursive(xMutex);
}

BaseType_t xSemaphoreGiveFromISR_extern(SemaphoreHandle_t xSemaphore, BaseType_t *pxHigherPriorityTaskWoken) {
    return xSemaphoreGiveFromISR(xSemaphore, pxHigherPriorityTaskWoken);
}

BaseType_t xSemaphoreTakeFromISR_extern(SemaphoreHandle_t xSemaphore, BaseType_t *pxHigherPriorityTaskWoken) {
    return xSemaphoreTakeFromISR(xSemaphore, pxHigherPriorityTaskWoken);
}

void taskYIELD_extern(void) {
    taskYIELD();
}

void taskENTER_CRITICAL_extern(void) {
    taskENTER_CRITICAL();
}

void taskEXIT_CRITICAL_extern(void) {
    taskEXIT_CRITICAL();
}

UBaseType_t taskENTER_CRITICAL_FROM_ISR_extern(void) {
    return taskENTER_CRITICAL_FROM_ISR();
}

void taskEXIT_CRITICAL_FROM_ISR_extern(UBaseType_t x) {
    taskEXIT_CRITICAL_FROM_ISR(x);
}

void taskDISABLE_INTERRUPTS_extern(void) {
    taskDISABLE_INTERRUPTS();
}

void taskENABLE_INTERRUPTS_extern(void) {
    taskENABLE_INTERRUPTS();
}
