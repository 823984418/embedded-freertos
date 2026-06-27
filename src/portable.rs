use crate::bindgen;

pub const portSTACK_GROWTH: BaseType_t = bindgen::portSTACK_GROWTH as _;
pub const portBYTE_ALIGNMENT: BaseType_t = bindgen::portBYTE_ALIGNMENT as _;

pub use bindgen::BaseType_t;
pub use bindgen::StackType_t;
pub use bindgen::TickType_t;
pub use bindgen::UBaseType_t;

pub const portTICK_TYPE_IS_ATOMIC: BaseType_t = bindgen::portTICK_TYPE_IS_ATOMIC as _;

pub const portBYTE_ALIGNMENT_MASK: BaseType_t = bindgen::portBYTE_ALIGNMENT_MASK as _;

pub use bindgen::HeapRegion_t;
pub use bindgen::HeapStats_t;

pub use bindgen::vPortDefineHeapRegions;
