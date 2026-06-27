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
