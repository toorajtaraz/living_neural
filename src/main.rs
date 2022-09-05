use arrayfire::*;

fn main() {
    set_device(0);
    info();
    conways_game_of_life();
}

fn inverse_gaussian(a: &Array<f32>) -> Array<f32> {
    //-1./pow(2., (0.6*pow(x, 2.)))+1.0
    let c1 = constant::<f32>(1.0, Dim4::new(&[1, 1, 1, 1]));
    let cm1 = constant::<f32>(-1.0, Dim4::new(&[1, 1, 1, 1]));
    let cp6 = constant::<f32>(0.6, Dim4::new(&[1, 1, 1, 1]));
    &cm1 / &pow2(&(&cp6 * &(a * a))) + &c1
}

fn normalise(a: &Array<f32>) -> Array<f32> {
    a / (max_all(&abs(a)).0 as f32)
}

fn conways_game_of_life() {
    let h_kernel = [0.68, -0.9, 0.68, -0.9, -0.66, -0.9, 0.68, -0.9, 0.68];
    let kernel = Array::new(&h_kernel, Dim4::new(&[3, 3, 1, 1]));
    // let s = constant::<f32>(0.5, Dim4::new(&[1, 1, 1, 1]));

    let mut state = randu::<f32>(Dim4::new(&[1024, 1024, 1, 1]));
    // let mut draw_buff = randu::<f32>(Dim4::new(&[256, 256, 1, 1]));
    // let mut state = gt(&randu::<f32>(Dim4::new(&[256, 256, 3, 1])), &s, false);
    // let c0 = constant::<f32>(2.0, Dim4::new(&[1, 1, 1, 1]));
    // let c1 = constant::<f32>(3.0, Dim4::new(&[1, 1, 1, 1]));

    let win = Window::new(1024, 1024, "Game of Life".to_string());
    while !win.is_closed() {
        let mut draw = convolve2(&state, &kernel, ConvMode::DEFAULT, ConvDomain::SPATIAL);
        draw = convolve2(&draw, &kernel, ConvMode::DEFAULT, ConvDomain::SPATIAL);
        draw = convolve2(&draw, &kernel, ConvMode::DEFAULT, ConvDomain::SPATIAL);
        draw = convolve2(&draw, &kernel, ConvMode::DEFAULT, ConvDomain::SPATIAL);

        // let c0 = &eq(&n_hood, &c0, false);
        // let c1 = &eq(&n_hood, &c1, false);
        // draw_buff = state;
        state = inverse_gaussian(&draw);
        win.draw_image(&normalise(&draw), None);
    }
}
// #[macro_use]
// extern crate glium;

// mod buffer_initializer;
// mod cmd_handler;
// mod kernels;
// mod shaders;

// #[derive(Copy, Clone)]
// struct Vertex {
//     points: [f32; 2],
// }

// implement_vertex!(Vertex, points);

// impl Vertex {
//     fn new(x: f32, y: f32) -> Vertex {
//         Vertex { points: [x, y] }
//     }
// }

// fn main() {
//     use glium::glutin;
//     use glium::index::PrimitiveType;
//     use glium::Surface;
//     use glutin::dpi::LogicalSize;

//     let conf = cmd_handler::get_args();
//     let width: u32;
//     let height: u32;
//     let fps: f32;
//     let kernel;
//     let color;
//     let skip;

//     width = conf.width;
//     height = conf.height;
//     fps = conf.fps;
//     kernel = conf.kernel;
//     color = conf.color;
//     skip = conf.skip;

//     let mut single_pixel = [1.0f32 / width as f32, 1.0 / height as f32];

//     let fragment_src = conf.fragment_src;

//     let event_loop = glutin::event_loop::EventLoop::new();
//     let wb = glutin::window::WindowBuilder::new()
//         .with_always_on_top(true)
//         .with_max_inner_size(LogicalSize::new(height, width))
//         .with_resizable(false);
//     let cb = glutin::ContextBuilder::new();
//     let display = glium::Display::new(wb, cb, &event_loop).unwrap();

//     let points = vec![
//         Vertex::new(-1.0, -1.0),
//         Vertex::new(1.0, -1.0),
//         Vertex::new(-1.0, 1.0),
//         Vertex::new(1.0, 1.0),
//     ];
//     let vertex_buffer = glium::VertexBuffer::new(&display, &points).unwrap();
//     let indices = glium::IndexBuffer::new(
//         &display,
//         PrimitiveType::TrianglesList,
//         &[0, 1, 2, 1, 2, 3u16],
//     )
//     .unwrap();
//     let program = glium::Program::from_source(
//         &display,
//         shaders::vertex::VERTEX_SRC,
//         fragment_src.as_str(),
//         None,
//     )
//     .unwrap();

//     let mut u_plane_base = buffer_initializer::new_as_texture(
//         buffer_initializer::InitMode::RANDOM,
//         width,
//         height,
//         &display,
//     );
//     let mut dest_texture = buffer_initializer::new_empty_texture(width, height, &display);
//     dest_texture.as_surface().clear_color(0.0, 0.0, 0.0, 1.0);

//     let mut is_first: bool = true;
//     let mut do_calc: bool = true;

//     event_loop.run(move |ev, _, control_flow| {
//         match ev {
//             glutin::event::Event::WindowEvent { event, .. } => match event {
//                 glutin::event::WindowEvent::CloseRequested => {
//                     *control_flow = glutin::event_loop::ControlFlow::Exit;
//                     return;
//                 }
//                 glutin::event::WindowEvent::Resized(size) => {
//                     single_pixel = [1.0f32 / size.width as f32, 1.0 / size.height as f32];

//                     u_plane_base = buffer_initializer::new_as_texture(
//                         buffer_initializer::InitMode::RANDOM,
//                         size.width,
//                         size.height,
//                         &display,
//                     );
//                     dest_texture =
//                         buffer_initializer::new_empty_texture(size.width, size.height, &display);
//                     dest_texture.as_surface().clear_color(0.0, 0.0, 0.0, 1.0);

//                     is_first = true;
//                     do_calc = true;
//                 }
//                 _ => return,
//             },
//             glutin::event::Event::NewEvents(cause) => match cause {
//                 glutin::event::StartCause::ResumeTimeReached { .. } => (),
//                 glutin::event::StartCause::Init => (),
//                 _ => return,
//             },
//             _ => return,
//         }
//         let mut target_fb =
//             glium::framebuffer::SimpleFrameBuffer::new(&display, &dest_texture).unwrap();
//         if is_first {
//             let uniforms = uniform! {
//                 u_kernel: kernel,
//                 u_do_calc: do_calc,
//                 u_color_mask: color,
//                 u_single_pixel: single_pixel,
//                 u_plane : &u_plane_base,

//             };

//             for _ in 0..skip {
//                 target_fb
//                     .draw(
//                         &vertex_buffer,
//                         &indices,
//                         &program,
//                         &uniforms,
//                         &Default::default(),
//                     )
//                     .unwrap();

//                 target_fb.fill(
//                     &dest_texture.as_surface(),
//                     glium::uniforms::MagnifySamplerFilter::Nearest,
//                 );
//             }
//             is_first = false;
//             do_calc = false;
//         } else {
//             let uniforms = uniform! {
//                 u_kernel: kernel,
//                 u_do_calc: do_calc,
//                 u_color_mask: color,
//                 u_single_pixel: single_pixel,
//                 u_plane : &dest_texture,

//             };

//             if do_calc {
//                 for _ in 0..skip {
//                     target_fb
//                         .draw(
//                             &vertex_buffer,
//                             &indices,
//                             &program,
//                             &uniforms,
//                             &Default::default(),
//                         )
//                         .unwrap();

//                     target_fb.fill(
//                         &dest_texture.as_surface(),
//                         glium::uniforms::MagnifySamplerFilter::Nearest,
//                     );
//                 }
//                 do_calc = false;
//             } else {
//                 let mut target = display.draw();
//                 target
//                     .draw(
//                         &vertex_buffer,
//                         &indices,
//                         &program,
//                         &uniforms,
//                         &Default::default(),
//                     )
//                     .unwrap();

//                 target.finish().unwrap();
//                 do_calc = true;
//             }
//         }

//         let next_frame_time = std::time::Instant::now()
//             + std::time::Duration::from_nanos(((1.0 / fps) * 1_000_000f32) as u64);
//         *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
//     });
// }
