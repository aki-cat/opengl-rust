// #![windows_subsystem = "windows"]

use std::{process, thread};

use glfw::{fail_on_errors, Context, WindowHint};
use logthis::{debug, error, Level, Log};

const TRIANGLE: [f32; 18] = [
    0.00, 0.50, 0.0, /* Pos|Color */ 1.0, 0.0, 0.0, // 0
    0.50, -0.5, 0.0, /* Pos|Color */ 0.0, 1.0, 0.0, // 1
    -0.5, -0.5, 0.0, /* Pos|Color */ 0.0, 0.0, 1.0, // 2
];

pub fn load_triangle_shader(
    context: &opengl::Context,
    v_code: &str,
    f_code: &str,
) -> opengl::Program {
    let vs = context.new_shader(opengl::ShaderType::Vertex);
    vs.source(v_code);
    vs.compile().unwrap();

    let fs = context.new_shader(opengl::ShaderType::Fragmet);
    fs.source(f_code);
    fs.compile().unwrap();

    let program = context.new_program();
    program.attach(&vs);
    program.attach(&fs);
    program.link().unwrap();
    program
}

pub fn load_triangle_buffer(context: &opengl::Context) -> opengl::Vertex {
    let mut vertex = opengl::Vertex::new(context);
    vertex.new_buffer(|vbo| {
        vbo.bind(opengl::Target::Array);
        vbo.data(opengl::Target::Array, &TRIANGLE, opengl::Usage::StaticDraw);

        opengl::Buffer::gen_mark(&[(opengl::GlType::f32, 3); 2]);
    });

    vertex
}

const SQUARE: [f32; 24] = [
    0.50, 0.50, 0.0, /* Pos|Color */ 1.0, 0.0, 0.0, // 0
    0.50, -0.5, 0.0, /* Pos|Color */ 0.0, 1.0, 0.0, // 1
    -0.5, -0.5, 0.0, /* Pos|Color */ 0.0, 0.0, 1.0, // 2
    -0.5, 0.50, 0.0, /* Pos|Color */ 1.0, 0.5, 0.0, // 3
];

const SQUARE_INDICES: [u32; 6] = [0, 1, 2, 0, 2, 3];

pub fn load_square_buffer(context: &opengl::Context) -> opengl::Vertex {
    let mut vertex = opengl::Vertex::new(context);
    vertex.new_buffer(|vbo| {
        vbo.bind(opengl::Target::Array);
        vbo.data(opengl::Target::Array, &SQUARE, opengl::Usage::StaticDraw);

        opengl::Buffer::gen_mark(&[(opengl::GlType::f32, 3); 2]);
    });

    vertex.new_buffer(|ebo| {
        ebo.bind(opengl::Target::ElementArray);
        ebo.data(
            opengl::Target::ElementArray,
            &SQUARE_INDICES,
            opengl::Usage::StaticDraw,
        );
    });

    vertex
}

fn main() {
    Log::set_level(Level::Debug);
    Log::set_current_thread_name("MainThread");
    debug!("Created Main Thread");

    let mut glfw = match glfw::init(fail_on_errors!()) {
        Err(_) => {
            error!("Failed to initialize program");
            process::exit(1);
        }
        Ok(glfw) => glfw,
    };

    glfw.window_hint(WindowHint::ContextVersion(
        opengl::OPENGL_VERSION.0,
        opengl::OPENGL_VERSION.1,
    ));

    let result = glfw.create_window(640, 480, "GLE", glfw::WindowMode::Windowed);
    let (mut window, events) = if let Some(some) = result {
        some
    } else {
        error!("Failed to create window");
        process::exit(1);
    };

    window.set_all_polling(true);

    let handle = thread::spawn(move || {
        Log::set_current_thread_name("RenderThread");
        debug!("Created Render Thread");

        window.make_current();
        let context = opengl::Context::new(|s| window.get_proc_address(s));

        let triangle = load_triangle_buffer(&context);
        let square = load_square_buffer(&context);
        let program = load_triangle_shader(
            &context,
            include_str!("shader/test.vs"),
            include_str!("shader/test.fs"),
        );
        while !window.should_close() {
            let (width, height) = window.get_size();
            context.view_port(0, 0, width, height);

            context.clear_color(0.3, 0.6, 0.6, 1.0);
            context.clear(opengl::Mask::COLOR_BUFFER_BIT);

            program.using();
            program
                .set_uniform(
                    "trans",
                    &mats::translate3(mats::Vec3::from([[0.5, 0.5, 0.0]])),
                )
                .unwrap();
            triangle.using();
            context.draw_arrays(opengl::Mode::Triangles, 0, 3);

            program
                .set_uniform(
                    "trans",
                    &mats::translate3(mats::Vec3::from([[-0.5, -0.5, 0.0]])),
                )
                .unwrap();
            context.draw_arrays(opengl::Mode::Triangles, 0, 3);

            program
                .set_uniform(
                    "trans",
                    &mats::translate3(mats::Vec3::from([[0.5, -0.5, 0.0]])),
                )
                .unwrap();
            square.using();
            context.draw_elements(opengl::Mode::Triangles, 6, opengl::GlType::u32, 0);

            program
                .set_uniform(
                    "trans",
                    &mats::translate3(mats::Vec3::from([[-0.5, 0.5, 0.0]])),
                )
                .unwrap();
            context.draw_elements(opengl::Mode::Triangles, 6, opengl::GlType::u32, 0);

            window.swap_buffers();
        }
        debug!("Render Thread Exited");
    });

    let handle = thread::spawn(move || {
        Log::set_current_thread_name("PollThread");
        debug!("Created Poll Thread");

        while !handle.is_finished() {
            for (_, event) in glfw::flush_messages(&events) {
                let _ = event;
            }
        }
        handle.join().unwrap();
        debug!("Poll Thread Exited");
    });

    while !handle.is_finished() {
        glfw.poll_events();
    }

    handle.join().unwrap();
    debug!("Main Thread Exited");
}
