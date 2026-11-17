//! Integration tests for SigmaOS-arch-shader-alliances-kernel.

use sigmaos_arch_shader_alliances_kernel_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
