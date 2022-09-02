#[macro_use]
extern crate glium;

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
    use image::{Rgba, RgbaImage};
    use rand::prelude::*;
    use std::io::Cursor;
    let mut rng = rand::thread_rng();
    let mut img = RgbaImage::new(WIDTH, HEIGHT);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let t = rng.gen::<u8>();
            img.put_pixel(x, y, Rgba([t, t, t, t]));
        }
    }

    // for x in 0..WIDTH {
    //     for y in 0..HEIGHT {
    //         img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
    //     }
    // }
    // img.put_pixel(2, HEIGHT - 1, Rgba([255, 255, 255, 255]));
    let vertex_shader_src = r#"
    #version 450
    in vec2 points;
    out vec2 v_text_points;
    void main() {
        v_text_points = (points / 2.0 + 0.5);
        // v_text_points = points;
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
        // vec2 temp = point + u_single_pixel * offset;
        // if (temp.x < -1.) {
        //     if (temp.y < -1.) {
        //         return vec2(2.0 + temp.x, 2.0 + temp.y);
        //     } else if (temp.y > 1.) {
        //         return vec2(2.0 + temp.x,  -2.0 + temp.y);
        //     } else {
        //         return vec2(2.0 + temp.x, temp.y);
        //     }
        // } else if (temp.x > 1.) {
        //     if (temp.y < -1.) {
        //         return vec2(-2.0 + temp.x, 2.0 + temp.y);
        //     } else if (temp.y > 1.) {
        //         return vec2(-2.0 + temp.x, -2.0 + temp.y);
        //     } else {
        //         return vec2(-2.0 + temp.x, temp.y);
        //     }
        // } else {
        //     if (temp.y < -1.) {
        //         return vec2(temp.x, 2.0 + temp.y);
        //     } else if (temp.y > 1.) {
        //         return vec2(temp.x, -2.0 + temp.y);
        //     } else {
        //         return point;
        //     }
        // }
        return mod(point + u_single_pixel * offset, 1.0);
        // vec2 val = point + u_single_pixel * offset;
        // return val - floor(val);
    }

    float inverse_gaussian(float x) {
      return -1./pow(2., (0.6*pow(x, 2.)))+1.;
    }
    // float activation(float x) {
    //   if (x == 1. || x == 2. || x == 3.|| x == 4.){
    //     return 1.;
    //   }
    //   return 0.;
    // }
    float tanh(float x) {
      return (exp(2.*x)-1.)/(exp(2.*x)+1.);
    }

    float activation(float x) {
      return tanh(x);
    }
    // float activation(float x) {
    //   return inverse_gaussian(x);
    // }
    // float activation(float x) {
    //     return x;
    // }

    void main() {
        if (u_do_calc) {
            // float cur = texture(u_plane, get_point(v_text_points, vec2(0.0, 0.0))).a;
            // if (cur != 0.) {
            //     color = vec4(cur, cur, cur, cur);
            //     return;
            // }
            float conv_res_a =
                      texture(u_plane, get_point(v_text_points, vec2( 1.,-1.))).a * u_kernel[0][0]
                    + texture(u_plane, get_point(v_text_points, vec2( 0.,-1.))).a * u_kernel[1][0]
                    + texture(u_plane, get_point(v_text_points, vec2(-1.,-1.))).a * u_kernel[2][0]
                    + texture(u_plane, get_point(v_text_points, vec2( 1., 0.))).a * u_kernel[0][1]
                    + texture(u_plane, get_point(v_text_points, vec2( 0., 0.))).a * u_kernel[1][1]
                    + texture(u_plane, get_point(v_text_points, vec2(-1., 0.))).a * u_kernel[2][1]
                    + texture(u_plane, get_point(v_text_points, vec2( 1., 1.))).a * u_kernel[0][2]
                    + texture(u_plane, get_point(v_text_points, vec2( 0., 1.))).a * u_kernel[1][2]
                    + texture(u_plane, get_point(v_text_points, vec2(-1., 1.))).a * u_kernel[2][2];
            float activated = activation(conv_res_a);
            // float test = texture2D(u_plane, v_text_points).a;
            // float test = texture(u_plane, v_text_points).a;
            // color = texture(u_plane, v_text_points);

            color = vec4(activated, activated, activated, activated);
            // color = texture(u_plane, v_text_points);
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
    let indices = glium::IndexBuffer::new(
        &display,
        PrimitiveType::TrianglesList,
        &[0, 1, 2, 1, 2, 3u16],
    )
    .unwrap();

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut animator = -0.5f32;
    let _image = image::load(
        Cursor::new(&include_bytes!(
            "/home/toorajtaraz/Documents/projects/rust-projects/living_neural/assets/1.png"
        )),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = img.dimensions();
    let u_plane =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(), image_dimensions);
    let mut u_plane = glium::texture::SrgbTexture2d::new(&display, u_plane).unwrap();
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
        let uniforms = uniform! {
            u_kernel: [
                [0.037, 0.43, -0.737],
                [0.406, -0.321, -0.319],
                [-0.458, 0.416, 0.478f32],
            ],
            // u_kernel: [
            //     [0.68, -0.90, 0.68],
            //     [-0.9, -0.66, -0.90],
            //     [0.68, -0.90, 0.68f32],
            // ],
            // u_kernel: [
            //     [0., 0., 0.],
            //     [0., 0., 0.],
            //     [1.0, 2.0, 4.0f32],
            // ],
            u_do_calc: true,
            u_color_mask: [1.0f32, 0.0, 0.0, 1.0],
            u_single_pixel: [1.0f32/WIDTH as f32, 1.0/HEIGHT as f32],
            u_plane : &u_plane,

        };
        let mut target = display.draw();
        // target.clear_color(1.0, 0.0, 0.0, 1.0);
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
        let image: glium::texture::RawImage2d<'_, u8> = display.read_front_buffer().unwrap();
        u_plane = glium::texture::SrgbTexture2d::new(&display, image).unwrap();
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(1116_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        animator += 0.0002;
        if animator > 0.5 {
            animator = -0.5;
        }
    });
}
