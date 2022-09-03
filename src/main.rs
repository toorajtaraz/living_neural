#[macro_use]
extern crate glium;

mod buffer_initializer;
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
    use glutin::dpi::LogicalSize;

    let mut width: u32 = 500;
    let mut height: u32 = 500;
    let mut fps: f32 = 24.0;
    let mut kernel = kernels::get_kernel(kernels::Kernel::RANDOM, None);
    let mut single_pixel = [1.0f32 / width as f32, 1.0 / height as f32];
    let mut color = [0.4f32, 0.0, 0.6, 1.0];
    let mut skip = 4;

    let fragment_src = shaders::fragment::get_fragment_shader(
        shaders::fragment::Activation::INVERSEGAUSSIAN,
        false,
        None,
    );

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_always_on_top(true)
        .with_max_inner_size(LogicalSize::new(height, width))
        .with_resizable(false);
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let points = vec![
        Vertex::new(-1.0, -1.0),
        Vertex::new(1.0, -1.0),
        Vertex::new(-1.0, 1.0),
        Vertex::new(1.0, 1.0),
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &points).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        PrimitiveType::TrianglesList,
        &[0, 1, 2, 1, 2, 3u16],
    )
    .unwrap();
    let program = glium::Program::from_source(
        &display,
        shaders::vertex::VERTEX_SRC,
        fragment_src.as_str(),
        None,
    )
    .unwrap();

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
                    height = size.height;
                    width = size.width;
                    single_pixel = [1.0f32 / width as f32, 1.0 / height as f32];

                    u_plane_base = buffer_initializer::new_as_texture(
                        buffer_initializer::InitMode::RANDOM,
                        width,
                        height,
                        &display,
                    );
                    dest_texture = buffer_initializer::new_empty_texture(width, height, &display);
                    dest_texture.as_surface().clear_color(0.0, 0.0, 0.0, 1.0);

                    is_first = true;
                    do_calc = true;
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
        let mut target_fb =
            glium::framebuffer::SimpleFrameBuffer::new(&display, &dest_texture).unwrap();
        if is_first {
            let uniforms = uniform! {
                u_kernel: kernel,
                u_do_calc: do_calc,
                u_color_mask: color,
                u_single_pixel: single_pixel,
                u_plane : &u_plane_base,

            };

            for _ in 0..skip {
                target_fb
                    .draw(
                        &vertex_buffer,
                        &indices,
                        &program,
                        &uniforms,
                        &Default::default(),
                    )
                    .unwrap();

                target_fb.fill(
                    &dest_texture.as_surface(),
                    glium::uniforms::MagnifySamplerFilter::Nearest,
                );
            }
            is_first = false;
            do_calc = false;
        } else {
            let uniforms = uniform! {
                u_kernel: kernel,
                u_do_calc: do_calc,
                u_color_mask: color,
                u_single_pixel: single_pixel,
                u_plane : &dest_texture,

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
                        .unwrap();

                    target_fb.fill(
                        &dest_texture.as_surface(),
                        glium::uniforms::MagnifySamplerFilter::Nearest,
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
                    .unwrap();

                target.finish().unwrap();
                do_calc = true;
            }
        }

        let next_frame_time = std::time::Instant::now()
            + std::time::Duration::from_nanos(((1.0 / fps) * 1_000_000f32) as u64);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });
}
