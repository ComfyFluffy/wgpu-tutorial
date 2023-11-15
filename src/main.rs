fn main() {
    env_logger::init();
    pollster::block_on(wgpu_tutorial::run());
}
