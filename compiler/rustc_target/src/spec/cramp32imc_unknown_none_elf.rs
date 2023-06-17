use crate::spec::{LinkerFlavor, LldFlavor, PanicStrategy, RelocModel};
use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        data_layout: "e-m:e-p:32:32-i64:64-n32-S128".into(),
        llvm_target: "cramp32".into(),
        pointer_width: 32,
        arch: "cramp32".into(),

        options: TargetOptions {
            linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
            linker: Some("ld.lld".into()),
            cpu: "generic-cramp32".into(),
            max_atomic_width: Some(0),
            atomic_cas: false,
            //features: "+m,+c,+zbb,+experimental-zbt,+relax".into(),
            features: "+m,+c,+zbb,+relax".into(),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            ..Default::default()
        },
    }
}
