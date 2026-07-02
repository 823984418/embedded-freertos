use alloc::alloc::{alloc_zeroed, dealloc};
use core::alloc::Layout;
use core::ffi::c_void;

use crate::portBYTE_ALIGNMENT;

#[unsafe(no_mangle)]
unsafe extern "C" fn pvPortMalloc(xWantedSize: usize) -> *mut c_void {
    unsafe {
        if xWantedSize == 0 {
            return std::ptr::null_mut();
        }
        let Ok(layout) = Layout::from_size_align(xWantedSize, portBYTE_ALIGNMENT as usize) else {
            return std::ptr::null_mut();
        };
        let Ok((wrap_layout, offset)) = Layout::new::<Layout>().extend(layout) else {
            return std::ptr::null_mut();
        };
        let wrap_ptr = std::alloc::alloc(wrap_layout);
        if wrap_ptr.is_null() {
            return std::ptr::null_mut();
        }
        let ptr = wrap_ptr.add(offset);
        *(ptr.sub(size_of::<Layout>()) as *mut Layout) = layout;
        ptr as *mut c_void
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pvPortCalloc(xNum: usize, xSize: usize) -> *mut c_void {
    unsafe {
        let Some(size) = size_t::checked_mul(xNum, xSize) else {
            return std::ptr::null_mut();
        };
        if size == 0 {
            return std::ptr::null_mut();
        }
        let Ok(layout) = Layout::from_size_align(size, portBYTE_ALIGNMENT as usize) else {
            return std::ptr::null_mut();
        };
        let Ok((wrap_layout, offset)) = Layout::new::<Layout>().extend(layout) else {
            return std::ptr::null_mut();
        };
        let wrap_ptr = std::alloc::alloc_zeroed(wrap_layout);
        if wrap_ptr.is_null() {
            return std::ptr::null_mut();
        }
        let ptr = wrap_ptr.add(offset);
        *(ptr.sub(size_of::<Layout>()) as *mut Layout) = layout;
        ptr as *mut c_void
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vPortFree(x: *mut c_void) {
    unsafe {
        if x.is_null() {
            return;
        }
        let ptr = x as *mut u8;
        let layout = *(ptr.sub(size_of::<Layout>()) as *const Layout);
        let Ok((wrap_layout, offset)) = Layout::new::<Layout>().extend(layout) else {
            return;
        };
        let wrap_ptr = ptr.sub(offset);
        std::alloc::dealloc(wrap_ptr, wrap_layout)
    }
}
