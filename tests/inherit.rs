#![forbid(unsafe_code)]

use ores_otel_sidecar::{health, SidecarIdentity};

#[test]
fn inherits_shared_health() {
    let identity = SidecarIdentity::new("opto-sync-sidecar", "OPTO_SYNC_SIDECAR_BIND");
    let payload = health::current(identity, None);
    assert!(payload.ok);
    assert_eq!(payload.service, "opto-sync-sidecar");
}
