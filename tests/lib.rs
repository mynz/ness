extern crate rustness;

#[test]
fn test_integration_sample() {
    use rustness::ppu::PpuRegister;

    let r = PpuRegister::default();
    assert_eq!(r.ctrl, PpuRegister::default().ctrl);
}
