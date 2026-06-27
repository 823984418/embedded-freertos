#![allow(non_snake_case)]

use bindgen::callbacks::{ItemInfo, ParseCallbacks};
use regex::Regex;
use std::ffi::OsString;

const EMBEDDED_FREERTOS_INCLUDE: &str = "EMBEDDED_FREERTOS_INCLUDE";

fn main() {
    build_rs::output::rerun_if_changed("bindgen.c");

    let target = build_rs::input::target();
    let out_dir = build_rs::input::out_dir();
    let features = build_rs::input::cargo_cfg_feature();

    let mut portable = features
        .iter()
        .filter(|i| i.starts_with("portable-"))
        .collect::<Vec<_>>();
    portable.sort();
    if portable.len() > 1 {
        panic!(
            "Enable at most one heap at the same time, enabled: {:?}",
            portable
        );
    }
    let portable = portable.first();
    let portable = match portable.map(|i| i.as_str()) {
        Some("portable-MSVC-MingW") => "FreeRTOS-Kernel/portable/MSVC-MingW",
        Some("portable-ARM_CM0") => "FreeRTOS-Kernel/portable/GCC/ARM_CM0",
        Some("portable-ARM_CM3") => "FreeRTOS-Kernel/portable/GCC/ARM_CM3",
        Some("portable-ARM_CM4F") => "FreeRTOS-Kernel/portable/GCC/ARM_CM4F",
        _ => "FreeRTOS-Kernel/portable/template",
    };
    build_rs::output::rerun_if_changed(portable);

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
    let heap = heap.map(|i| format!("FreeRTOS-Kernel/portable/MemMang/{}.c", i));
    if let Some(heap) = &heap {
        build_rs::output::rerun_if_changed(heap);
    }

    build_rs::output::rerun_if_env_changed(EMBEDDED_FREERTOS_INCLUDE);
    let include = std::env::var_os(EMBEDDED_FREERTOS_INCLUDE).unwrap_or(OsString::from("include"));
    build_rs::output::rerun_if_changed(&include);

    let mut bindgen = bindgen::builder();
    bindgen = bindgen.use_core();
    bindgen = bindgen.layout_tests(false);
    bindgen = bindgen.derive_partialeq(true);
    bindgen = bindgen.merge_extern_blocks(true);
    bindgen = bindgen.prepend_enum_name(false);
    bindgen = bindgen.allowlist_file(".*FreeRTOS.*");
    bindgen = bindgen.allowlist_file(".*bindgen.*");

    bindgen = bindgen.clang_args(["-isystem", "stdlib"]);
    bindgen = bindgen.clang_arg(format!("--target={target}"));

    #[derive(Debug, Clone)]
    struct Callbacks;

    impl ParseCallbacks for Callbacks {
        fn generated_name_override(&self, item_info: ItemInfo<'_>) -> Option<String> {
            Some(item_info.name.trim_end_matches("_extern").to_string())
        }
    }

    bindgen = bindgen.parse_callbacks(Box::new(Callbacks));

    bindgen = bindgen.clang_args(["-I", &include.to_string_lossy()]);
    bindgen = bindgen.clang_args(["-I", portable]);
    bindgen = bindgen.clang_args(["-I", "FreeRTOS-Kernel/include"]);

    bindgen = bindgen.header("bindgen.c");
    let bindgen_rs = bindgen.generate().unwrap().to_string();
    std::fs::write(out_dir.join("bindgen.rs"), &bindgen_rs).unwrap();

    let reg = Regex::new(
        r###"const[ \r\n]+(?P<NAME>config.*)[ \r\n]*:[ \r\n]*.*[ \r\n]*=[ \r\n]*(?P<VALUE>.*)[ \r\n]*;"###,
    )
        .unwrap();
    for x in reg.captures_iter(&bindgen_rs) {
        let name = &x["NAME"];
        let value = &x["VALUE"];
        build_rs::output::rustc_cfg_value(name, value);
    }

    let mut cc = cc::Build::new();
    cc.compiler("clang");
    cc.archiver("llvm-ar");
    cc.pic(false);
    cc.std("c11");

    cc.flags(["-isystem", "stdlib"]);
    cc.flag(format!("--target={target}"));

    cc.include(include);
    cc.include(portable);
    cc.include("FreeRTOS-Kernel/include");

    cc.file("FreeRTOS-Kernel/croutine.c");
    cc.file("FreeRTOS-Kernel/event_groups.c");
    cc.file("FreeRTOS-Kernel/list.c");
    cc.file("FreeRTOS-Kernel/queue.c");
    cc.file("FreeRTOS-Kernel/stream_buffer.c");
    cc.file("FreeRTOS-Kernel/tasks.c");
    cc.file("FreeRTOS-Kernel/timers.c");

    for i in std::fs::read_dir(portable).unwrap() {
        if let Ok(entry) = i {
            let path = entry.path();
            if path.is_file()
                && let Some(file_extension) = path.extension()
                && let Some(file_extension) = file_extension.to_ascii_lowercase().to_str()
                && matches!(file_extension, "c" | "s")
            {
                cc.file(entry.path());
            }
        }
    }

    if let Some(heap) = heap {
        cc.file(heap);
    }

    cc.file("bindgen.c");

    cc.compile("embedded-freertos");
}
