#[macro_use]
extern crate glium;

macro_rules! dopanic {
    ($err:expr) => {{
        eprintln!("Error: {}", $err);
        std::process::exit(1);
    }};
}

mod buffer_initializer;
mod cmd_handler;
mod kernels;
mod shaders;

#[derive(Copy, Clone)]
struct Vertex {
    points: [f32; 2],
}

implement_vertex!(Vertex, points);

impl Vertex {
    fn new(x: f32, y: f32) -> Vertex {
        Vertex { points: [x, y] }
    }
}

fn main() {
    use glium::glutin;
    use glium::index::PrimitiveType;
    use glium::Surface;
    use glutin::dpi::PhysicalSize;

    let conf = cmd_handler::get_args();
    let width: u32;
    let height: u32;
    let fps: f32;
    let kernel_2d;
    let color;
    let skip;

    width = conf.width;
    height = conf.height;
    fps = conf.fps;
    kernel_2d = conf.kernel;
    color = conf.color;
    skip = conf.skip;

    let mut single_pixel = [1.0f32 / width as f32, 1.0 / height as f32];

    let fragment_src = conf.fragment_src;

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_always_on_top(true)
        .with_inner_size(PhysicalSize::new(height, width))
        .with_resizable(false);
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap_or_else(|err| dopanic!(err));

    let points = vec![
        Vertex::new(-1.0, -1.0),
        Vertex::new(1.0, -1.0),
        Vertex::new(-1.0, 1.0),
        Vertex::new(1.0, 1.0),
    ];
    let vertex_buffer =
        glium::VertexBuffer::new(&display, &points).unwrap_or_else(|err| dopanic!(err));
    let indices = glium::IndexBuffer::new(
        &display,
        PrimitiveType::TrianglesList,
        &[0, 1, 2, 3, 2, 1u16],
    )
    .unwrap_or_else(|err| dopanic!(err));
    let program = glium::Program::from_source(
        &display,
        shaders::vertex::VERTEX_SRC,
        fragment_src.as_str(),
        None,
    )
    .unwrap_or_else(|err| dopanic!(err));

    let mut u_plane_base = buffer_initializer::new_as_texture(
        buffer_initializer::InitMode::RANDOM,
        width,
        height,
        &display,
    );
    let mut dest_texture = buffer_initializer::new_empty_texture(width, height, &display);
    dest_texture.as_surface().clear_color(0.0, 0.0, 0.0, 1.0);

    let mut is_first: bool = true;
    let mut do_calc: bool = true;

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::Resized(size) => {
                    single_pixel = [1.0f32 / size.width as f32, 1.0 / size.height as f32];

                    u_plane_base = buffer_initializer::new_as_texture(
                        buffer_initializer::InitMode::RANDOM,
                        size.width,
                        size.height,
                        &display,
                    );
                    dest_texture =
                        buffer_initializer::new_empty_texture(size.width, size.height, &display);
                    dest_texture.as_surface().clear_color(0.0, 0.0, 0.0, 1.0);

                    is_first = true;
                    do_calc = true;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }
        let mut target_fb = glium::framebuffer::SimpleFrameBuffer::new(&display, &dest_texture)
            .unwrap_or_else(|err| dopanic!(err));
        if is_first {
            let uniforms = uniform! {
                u_kernel: kernel_2d,
                u_kernel_height: 3,
                u_kernel_width: 3,
                u_do_calc: do_calc,
                u_color_mask: color,
                u_single_pixel: single_pixel,
                u_plane : u_plane_base
                            .sampled()
                            .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat),
            };

            target_fb
                .draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap_or_else(|err| dopanic!(err));
            // let data = kernel.read().unwrap().u_kernel[0];
            // println!("{}", data[0]);
            target_fb.fill(
                &dest_texture.as_surface(),
                glium::uniforms::MagnifySamplerFilter::Linear,
            );

            is_first = false;
            do_calc = false;
        } else {
            let uniforms = uniform! {
                u_kernel: kernel_2d,
                u_kernel_height: 3,
                u_kernel_width: 3,
                u_do_calc: do_calc,
                u_color_mask: color,
                u_single_pixel: single_pixel,
                u_plane : dest_texture
                            .sampled()
                            .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat),
            };
            if do_calc {
                for _ in 0..skip {
                    target_fb
                        .draw(
                            &vertex_buffer,
                            &indices,
                            &program,
                            &uniforms,
                            &Default::default(),
                        )
                        .unwrap_or_else(|err| dopanic!(err));

                    // let data = kernel.read().unwrap().u_kernel[0];
                    // println!("{}", data[0]);
                    target_fb.fill(
                        &dest_texture.as_surface(),
                        glium::uniforms::MagnifySamplerFilter::Linear,
                    );
                }
                do_calc = false;
            } else {
                let mut target = display.draw();
                target
                    .draw(
                        &vertex_buffer,
                        &indices,
                        &program,
                        &uniforms,
                        &Default::default(),
                    )
                    .unwrap_or_else(|err| dopanic!(err));

                target.finish().unwrap_or_else(|err| dopanic!(err));
                do_calc = true;
            }
        }

        let next_frame_time = std::time::Instant::now()
            + std::time::Duration::from_nanos(((1.0 / fps) * 1_000_000f32) as u64);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });
}
