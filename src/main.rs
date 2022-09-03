#[macro_use]
extern crate glium;

mod buffer_initializer;
mod kernels;
mod shaders;

const HEIGHT: u32 = 500;
const WIDTH: u32 = 500;

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

    let points = vec![
        Vertex::new(-1.0, -1.0),
        Vertex::new(1.0, -1.0),
        Vertex::new(-1.0, 1.0),
        Vertex::new(1.0, 1.0),
    ];

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_always_on_top(true)
        .with_max_inner_size(LogicalSize::new(HEIGHT, WIDTH))
        .with_resizable(false);
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

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
        shaders::fragment::get_fragment_shader(
            shaders::fragment::Activation::INVERSEGAUSSIAN,
            false,
            None,
        )
        .as_str(),
        None,
    )
    .unwrap();

    let u_plane_base = buffer_initializer::new_as_texture(
        buffer_initializer::InitMode::RANDOM,
        WIDTH,
        HEIGHT,
        &display,
    );
    let dest_texture = buffer_initializer::new_empty_texture(WIDTH, HEIGHT, &display);
    dest_texture.as_surface().clear_color(0.0, 0.0, 0.0, 1.0);

    let kernel = kernels::get_kernel(kernels::Kernel::RANDOM, None);

    let mut is_first: &bool = &true;
    let mut do_calc: &bool = &true;
    event_loop.run(move |ev, _, control_flow| {
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
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
        let mut target_fb =
            glium::framebuffer::SimpleFrameBuffer::new(&display, &dest_texture).unwrap();
        if *is_first {
            let uniforms = uniform! {
                u_kernel: kernel,
                u_do_calc: *do_calc,
                u_color_mask: [1.0f32, 0.0, 0.0, 1.0],
                u_single_pixel: [1.0f32/WIDTH as f32, 1.0/HEIGHT as f32],
                u_plane : &u_plane_base,

            };
            target_fb
                .draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();
            is_first = &false;
            do_calc = &false;
        } else {
            let uniforms = uniform! {
                u_kernel: kernel,
                u_do_calc: *do_calc,
                u_color_mask: [0.4f32, 0.0, 0.6, 1.0],
                u_single_pixel: [1.0f32/WIDTH as f32, 1.0/HEIGHT as f32],
                u_plane : &dest_texture,

            };

            if *do_calc {
                for _ in 0..1 {
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
                do_calc = &false;
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
                do_calc = &true;
            }
        }
        // u_plane_next = Option::Some(&dest_texture);
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(26_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });
}
