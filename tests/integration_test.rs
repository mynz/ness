extern crate rustness;

#[test]
fn test_integration_sample() {
    assert_eq!(1 + 1, 2);

    use rustness::ppu::PpuRegister;

    let r = PpuRegister::default();
    assert_eq!(r.ctrl, PpuRegister::default().ctrl);
}

#[test]
fn test_render_bg() {
    assert!(true);
}
