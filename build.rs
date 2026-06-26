use std::ffi::OsString;

const EMBEDDED_FREERTOS_INCLUDE: &str = "EMBEDDED_FREERTOS_INCLUDE";

#[allow(non_camel_case_types)]
pub enum Portable {
    ARM_CM3,
    ARM_CM4F,
    Template,
}

impl Portable {
    pub fn from_env() -> Self {
        let target = build_rs::input::target();
        match target.as_str() {
            "thumbv7m-none-eabi" => Self::ARM_CM3,
            "thumbv7em-none-eabi" => Self::ARM_CM3,
            "thumbv7em-none-eabihf" => Self::ARM_CM4F,
            _ => {
                build_rs::output::warning(&format!(
                    "Unknow target {target}, use template portable!"
                ));
                Self::Template
            }
        }
    }

    pub fn path(&self) -> &'static str {
        match self {
            Portable::ARM_CM3 => "FreeRTOS-Kernel/portable/GCC/ARM_CM3",
            Portable::ARM_CM4F => "FreeRTOS-Kernel/portable/GCC/ARM_CM4F",
            Portable::Template => "FreeRTOS-Kernel/portable/template",
        }
    }
}

fn main() {
    let target = build_rs::input::target();
    let target_feature = build_rs::input::cargo_cfg_target_feature();
    let out_dir = build_rs::input::out_dir();
    let features = build_rs::input::cargo_cfg_feature();
    let mut heap = features
        .iter()
        .filter(|i| i.starts_with("heap_"))
        .collect::<Vec<_>>();
    heap.sort();
    if heap.len() > 1 {
        panic!(
            "Enable at most one heap at the same time, enabled: {:?}",
            heap
        );
    }
    let heap = heap.first();

    let portable = Portable::from_env();

    build_rs::output::rerun_if_env_changed(EMBEDDED_FREERTOS_INCLUDE);
    let include = std::env::var_os(EMBEDDED_FREERTOS_INCLUDE).unwrap_or(OsString::from("include"));
    build_rs::output::rerun_if_changed(&include);

    let mut bindgen = bindgen::builder();
    bindgen = bindgen.use_core();
    bindgen = bindgen.layout_tests(false);
    bindgen = bindgen.derive_partialeq(true);
    bindgen = bindgen.merge_extern_blocks(true);
    bindgen = bindgen.prepend_enum_name(false);
    bindgen = bindgen.blocklist_function(".*");
    bindgen = bindgen.allowlist_file(".*FreeRTOS.*");

    bindgen = bindgen.clang_args(["-isystem", "stdlib"]);
    bindgen = bindgen.clang_arg(format!("--target={target}"));
    for i in &target_feature {
        bindgen = bindgen.clang_args(["-Xclang", "-target-feature", "-Xclang", &format!("+{i}")]);
    }

    bindgen = bindgen.clang_args(["-I", &include.to_string_lossy()]);
    bindgen = bindgen.clang_args(["-I", portable.path()]);
    bindgen = bindgen.clang_args(["-I", "FreeRTOS-Kernel/include"]);

    bindgen = bindgen.header("FreeRTOS-Kernel/include/FreeRTOS.h");
    bindgen
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("bindgen.rs"))
        .unwrap();

    let mut cc = cc::Build::new();
    cc.compiler("clang");
    cc.archiver("llvm-ar");
    cc.pic(false);
    cc.std("c11");

    cc.flags(["-isystem", "stdlib"]);
    cc.flag(format!("--target={target}"));
    for i in &target_feature {
        cc.flags(["-Xclang", "-target-feature", "-Xclang", &format!("+{i}")]);
    }

    cc.include(include);
    cc.include(portable.path());
    cc.include("FreeRTOS-Kernel/include");

    cc.file("FreeRTOS-Kernel/croutine.c");
    cc.file("FreeRTOS-Kernel/event_groups.c");
    cc.file("FreeRTOS-Kernel/list.c");
    cc.file("FreeRTOS-Kernel/queue.c");
    cc.file("FreeRTOS-Kernel/stream_buffer.c");
    cc.file("FreeRTOS-Kernel/tasks.c");
    cc.file("FreeRTOS-Kernel/timers.c");

    if let Some(heap) = heap {
        cc.file(format!("FreeRTOS-Kernel/portable/MemMang/{}.c", heap));
    }

    cc.compile("embedded-freertos");
}
