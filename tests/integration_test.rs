// Integration test for SigmaOS-gpu-performance-monitoring
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use gpu_performance_monitoring as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
