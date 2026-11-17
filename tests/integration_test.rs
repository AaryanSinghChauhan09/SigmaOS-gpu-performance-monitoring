//! Integration tests for SigmaOS-gpu-performance-monitoring.

use sigmaos_gpu_performance_monitoring_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
