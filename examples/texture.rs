// #![windows_subsystem = "windows"]

use std::{process, sync::mpsc::channel, thread};

use glfw::{fail_on_errors, Context, WindowHint};

use opengl::*;

const IMAGE: [f32; 20] = [
    0.50, 0.50, 0.0, /* Pos|Color */ 1.0, 0.0, // 0
    0.50, -0.5, 0.0, /* Pos|Color */ 1.0, 1.0, // 1
    -0.5, -0.5, 0.0, /* Pos|Color */ 0.0, 1.0, // 2
    -0.5, 0.50, 0.0, /* Pos|Color */ 0.0, 0.0, // 3
];

const INDEICE: [u32; 6] = [0, 1, 2, 0, 2, 3];

pub fn load_shader(context: &opengl::Context, v_code: &str, f_code: &str) -> Program {
    let vs = context.new_shader(ShaderType::Vertex);
    vs.source(v_code);
    vs.compile().unwrap();

    let fs = context.new_shader(ShaderType::Fragmet);
    fs.source(f_code);
    fs.compile().unwrap();

    let program = context.new_program();
    program.attach(&vs);
    program.attach(&fs);
    program.link().unwrap();
    program
}

pub fn load_buffer(context: &opengl::Context) -> Vertex {
    let mut vertex = Vertex::new(context);

    vertex.new_buffer(|vbo| {
        vbo.bind(Target::Array);
        vbo.data(Target::Array, &IMAGE, Usage::StaticDraw);
        Buffer::gen_mark(&[(GlType::f32, 3), (GlType::f32, 2)]);
    });

    vertex.new_buffer(|ebo| {
        ebo.bind(Target::ElementArray);
        ebo.data(
            Target::ElementArray,
            &INDEICE,
            Usage::StaticDraw,
        );
    });

    vertex
}

pub fn load_texture(context: &opengl::Context) -> Texture {
    let texture = context.new_texture();

    texture.bind(TexTarget::Tex2D);
    texture.set(TexParam::WrapS(Wrap::Repeat));
    texture.set(TexParam::WrapT(Wrap::Repeat));
    texture.set(TexParam::MinFilter(MinFilter::Nearest));
    texture.set(TexParam::MagFilter(MagFilter::Nearest));

    let img = image::open("examples/images/tex.png").unwrap().to_rgba8();
    let size = img.dimensions();
    let data = img.into_raw();

    Texture::load(
        ImageTarget::Texture2d,
        InternalFormat::Base(BaseFormat::RGBA),
        size,
        ImageFormat::RGBA,
        PixelDataType::u8,
        &data,
    )
    .unwrap();

    texture
}

fn main() {
    let mut glfw = match glfw::init(fail_on_errors!()) {
        Err(_) => {
            eprintln!("Failed to initialize program");
            process::exit(1);
        }
        Ok(glfw) => glfw,
    };

    glfw.window_hint(WindowHint::ContextVersion(
        OPENGL_VERSION.0,
        OPENGL_VERSION.1,
    ));
    glfw.window_hint(WindowHint::Visible(false));

    let result = glfw.create_window(640, 480, "GLE", glfw::WindowMode::Windowed);
    let (mut window, events) = if let Some(some) = result {
        some
    } else {
        eprintln!("Failed to create window");
        process::exit(1);
    };

    window.set_all_polling(true);

    let (tx, rx) = channel();
    let (tx2, rx2) = channel();
    let handle = thread::spawn(move || {
        window.make_current();

        let context = opengl::Context::new(|s| window.get_proc_address(s));
        let tex = load_texture(&context);
        let image = load_buffer(&context);
        let program = load_shader(
            &context,
            include_str!("shader/tex.vs"),
            include_str!("shader/tex.fs"),
        );
        tx.send(window).unwrap();
        let mut window: glfw::PWindow = rx2.recv().unwrap();
        while !window.should_close() {
            let (width, height) = window.get_size();
            context.view_port(0, 0, width, height);

            context.clear_color(0.3, 0.6, 0.6, 1.0);
            context.clear(Mask::COLOR_BUFFER_BIT);

            program.using();
            image.using();
            Texture::active(0);
            tex.bind(TexTarget::Tex2D);
            program.set_uniform("tex", &0).unwrap();
            context.draw_elements(Mode::Triangles, 6, GlType::u32, 0);

            window.swap_buffers();
        }
    });

    let mut window = rx.recv().unwrap();

    window.show();
    tx2.send(window).unwrap();

    let handle = thread::spawn(move || {

        while !handle.is_finished() {
            for (_, event) in glfw::flush_messages(&events) {
                let _ = event;
            }
        }
        handle.join().unwrap();
    });

    while !handle.is_finished() {
        glfw.poll_events();
    }

    handle.join().unwrap();
}
