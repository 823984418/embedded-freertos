use crate::{taskENTER_CRITICAL, taskEXIT_CRITICAL};

pub struct FreeRTOSCriticalSection;

critical_section::set_impl!(FreeRTOSCriticalSection);

unsafe impl critical_section::Impl for FreeRTOSCriticalSection {
    unsafe fn acquire() -> critical_section::RawRestoreState {
        taskENTER_CRITICAL();
    }

    unsafe fn release(_restore_state: critical_section::RawRestoreState) {
        taskEXIT_CRITICAL();
    }
}
