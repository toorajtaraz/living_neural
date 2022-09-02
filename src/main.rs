#[macro_use]
extern crate glium;

const HEIGHT: u32 = 200;
const WIDTH: u32 = 200;

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
    use glutin::dpi::PhysicalSize;

    let vertex_shader_src = r#"
    #version 450
    in vec2 points;
    out vec2 v_text_points;
    void main() {
        v_text_points = (points / 2.0 + 0.5);
        gl_Position = vec4(points, 1.0, 1.0);
    }
"#;
    let fragment_shader_src = r#"
    #version 450
    in vec2 v_text_points;
    out vec4 color;

    uniform vec2 u_single_pixel;
    uniform vec4 u_color_mask;
    uniform mat3 u_kernel;
    uniform sampler2D u_plane;
    uniform bool u_do_calc;

    vec2 get_point(vec2 point, vec2 offset) {
        vec2 val = point + u_single_pixel * offset;
        return val - floor(val);
    }

    float inverse_gaussian(float x) {
      return -1./pow(2., (0.6*pow(x, 2.)))+1.;
    }

    float activation(float x) {
      return inverse_gaussian(x);
    }	

    void main() {
        if (u_do_calc) {
            float conv_res = 
                      texture2D(u_plane, get_point(v_text_points, vec2( 1.,-1.))).a * u_kernel[0][0] 
                    + texture2D(u_plane, get_point(v_text_points, vec2( 0.,-1.))).a * u_kernel[0][1]
                    + texture2D(u_plane, get_point(v_text_points, vec2(-1.,-1.))).a * u_kernel[0][2]
                    + texture2D(u_plane, get_point(v_text_points, vec2( 1., 0.))).a * u_kernel[1][0]
                    + texture2D(u_plane, get_point(v_text_points, vec2( 0., 0.))).a * u_kernel[1][1]
                    + texture2D(u_plane, get_point(v_text_points, vec2(-1., 0.))).a * u_kernel[1][2]
                    + texture2D(u_plane, get_point(v_text_points, vec2( 1., 1.))).a * u_kernel[2][0]
                    + texture2D(u_plane, get_point(v_text_points, vec2( 0., 1.))).a * u_kernel[2][1]
                    + texture2D(u_plane, get_point(v_text_points, vec2(-1., 1.))).a * u_kernel[2][2];
            float activated = activation(conv_res);

            color = vec4(activated, activated, activated, activated) * u_color_mask;
        }
    }
"#;

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
    let indices =
        glium::IndexBuffer::new(&display, PrimitiveType::Points, &[0u16, 1, 2, 2, 1, 3]).unwrap();

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut animator = -0.5f32;

    event_loop.run(move |ev, _, control_flow| {
        let mut uniforms = uniform! {
            u_kernel: [
                [0.68, -0.90, 0.68],
                [-0.9, -0.66, -0.90],
                [0.68, -0.90, 0.68f32],
            ],
            u_do_calc: true,
            u_color_mask: [1.0f32, 0.0, 0.0, 1.0],
            // u_single_pixel: []


        };
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
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
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(6_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        animator += 0.0002;
        if animator > 0.5 {
            animator = -0.5;
        }

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
    });
}
