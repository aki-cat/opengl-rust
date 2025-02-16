use glfw::*;

pub fn make_context((width, height): (u32, u32), title: &str) -> (PWindow, Glfw) {
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    let (mut window, _) = glfw
        .create_window(width, height, title, WindowMode::Windowed)
        .unwrap();
    window.make_current();
    (window, glfw)
}
