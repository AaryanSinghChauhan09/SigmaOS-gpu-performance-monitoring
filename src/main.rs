//! SigmaOS-gpu-performance-monitoring — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_gpu_performance_monitoring_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-gpu-performance-monitoring: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-gpu-performance-monitoring: ok");
}
