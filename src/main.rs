#![forbid(unsafe_code)]

#[path = "../generated/rust/env.rs"]
mod env;
#[path = "../generated/rust/runtime.rs"]
mod env_runtime;

use ores_otel_sidecar::{
    runtime, SidecarConfig, SidecarIdentity, DEFAULT_SIDECAR_CONFIG_PATH,
};

fn main() {
    let identity = SidecarIdentity::new(env::SERVICE, env::BIND);
    let values = env_runtime::load_from_os();
    let cfg = match SidecarConfig::from_bind(identity, &values.bind, false) {
        Ok(cfg) => cfg,
        Err(_) => runtime::exit_invalid_config(identity),
    };
    let cfg = match cfg.with_sidecar_file(DEFAULT_SIDECAR_CONFIG_PATH) {
        Ok(cfg) => cfg,
        Err(_) => runtime::exit_invalid_config(identity),
    };
    runtime::run(&cfg);
}
