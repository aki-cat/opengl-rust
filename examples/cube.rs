mod glcontext;

use glfw::Context as _;
use mats::look_at;
use mats::*;
use opengl::*;

const VERTEX: [f32; 48] = [
    -0.5, -0.5, -0.5, /* Pos|Color */ 0.0, 0.0, 0.0, // 0
    0.50, -0.5, -0.5, /* Pos|Color */ 1.0, 0.0, 0.0, // 1
    -0.5, 0.50, -0.5, /* Pos|Color */ 0.0, 1.0, 0.0, // 2
    -0.5, -0.5, 0.50, /* Pos|Color */ 0.0, 0.0, 1.0, // 3
    -0.5, 0.50, 0.50, /* Pos|Color */ 0.0, 1.0, 1.0, // 4
    0.50, -0.5, 0.50, /* Pos|Color */ 1.0, 0.0, 1.0, // 5
    0.50, 0.50, -0.5, /* Pos|Color */ 1.0, 1.0, 0.0, // 6
    0.50, 0.50, 0.50, /* Pos|Color */ 1.0, 1.0, 1.0, // 7
];

const INDEICE: [u32; 36] = [
    0, 1, 2, // 0
    0, 2, 3, // 1
    0, 1, 3, // 2
    7, 6, 5, // 3
    7, 5, 4, // 4
    7, 6, 4, // 5
    3, 4, 5, // 6
    1, 6, 5, // 7
    4, 2, 6, // 8
    2, 4, 3, // 9
    1, 5, 3, // 10
    1, 6, 2, // 11
];

fn load_buffer(context: &opengl::Context) -> Vertex {
    let mut vertex = Vertex::new(context);
    vertex.new_buffer(&context, |vbo| {
        vbo.bind(Target::Array);
        vbo.data(&VERTEX, Usage::StaticDraw);
        Buffer::gen_mark(&[(GlType::f32, 3); 2]);
    });
    vertex.new_buffer(&context, |ebo| {
        ebo.bind(Target::ElementArray);
        ebo.data(&INDEICE, Usage::StaticDraw);
    });
    vertex
}

fn load_shader(context: &opengl::Context) -> Program {
    let vs = context.new_shader(ShaderType::Vertex);
    vs.source(include_str!("shader/test.vs"));
    vs.compile().unwrap();

    let fs = context.new_shader(ShaderType::Fragmet);
    fs.source(include_str!("shader/test.fs"));
    fs.compile().unwrap();

    let program = context.new_program();
    program.attach(&vs);
    program.attach(&fs);
    program.link().unwrap();

    program
}

fn main() {
    let (mut window, mut glfw) = glcontext::make_context((800, 600), "GLFW");
    let context = Context::new(|s| window.get_proc_address(s));
    context.enable(Cap::DepthTest);
    let vertex = load_buffer(&context);
    let program = load_shader(&context);
    let now = std::time::Instant::now();
    while !window.should_close() {
        context.clear_color(0.3, 0.3, 0.3, 1.0);
        context.clear(Mask::COLOR_BUFFER_BIT | Mask::DEPTH_BUFFER_BIT);

        let (width, height) = window.get_size();
        context.view_port(0, 0, width, height);

        let trans = perspective(45.0, width as f32 / height as f32, 0.1, 100.0)
            * look_at(
                Vec3::from([[0.0, 1.0, 2.0]]),
                Vec3::from([[0.0, 0.0, 0.0]]),
                Vec3::from([[0.0, 1.0, 0.0]]),
            )
            * rotate3(
                radian(now.elapsed().as_millis() as f32 / 10.0),
                Vec3::from([[1.0, 1.0, 0.0]]),
            );

        program.using();
        program.set_uniform("trans", &trans).unwrap();
        vertex.using();
        context.draw_elements(Mode::Triangles, 36, GlType::u32, 0);

        window.swap_buffers();
        glfw.poll_events();
    }
}
