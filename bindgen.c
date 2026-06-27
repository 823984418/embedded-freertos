#include "FreeRTOS.h"

#include "croutine.h"
#include "event_groups.h"
#include "list.h"
#include "message_buffer.h"
#include "queue.h"
#include "semphr.h"
#include "stack_macros.h"
#include "stream_buffer.h"
#include "task.h"
#include "timers.h"

void taskYIELD_extern() { taskYIELD(); }

void taskENTER_CRITICAL_extern() { taskENTER_CRITICAL(); }

void taskEXIT_CRITICAL_extern() { taskEXIT_CRITICAL(); }

UBaseType_t taskENTER_CRITICAL_FROM_ISR_extern() {
  return taskENTER_CRITICAL_FROM_ISR();
}

void taskEXIT_CRITICAL_FROM_ISR_extern(UBaseType_t x) {
  taskEXIT_CRITICAL_FROM_ISR(x);
}

void taskDISABLE_INTERRUPTS_extern() { taskDISABLE_INTERRUPTS(); }

void taskENABLE_INTERRUPTS_extern() { taskENABLE_INTERRUPTS(); }
