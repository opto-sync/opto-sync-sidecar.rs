#![forbid(unsafe_code)]

use opto_sync_sidecar::{config::SidecarConfig, runtime};

fn main() {
    let cfg = SidecarConfig::from_env();
    runtime::run(&cfg);
}

