#[macro_use]
extern crate gfx;
//----------------------------------------
// Cube associated data
//----------------------------------------

gfx_vertex_struct!(Vertex {
    coordinates: [i8; 2] = "coordinates",
});

impl Vertex {
    fn new(coord: [i8; 2]) -> Vertex {
        Vertex { coordinates: coord }
    }
}

gfx_pipeline!( pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    color_mask: gfx::Global<[f32; 4]> = "colorMask",
    one_pixel: gfx::Global<[f32; 2]> = "onePixel",
    do_step: gfx::Global<i32> = "doStep",
    // kernel: gfx::RawGlobal = "u_kernel[0]",
    kernel: gfx::Global<[[f32; 3]; 3]> = "kernel",
    image: gfx::TextureSampler<[f32; 4]> = "u_image",
    out_color: gfx::RenderTarget<::gfx::format::Srgba8> = "color_out",
    out_depth: gfx::DepthTarget<::gfx::format::DepthStencil> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
});

//----------------------------------------

fn main() {
    use camera_controllers::{FirstPerson, FirstPersonSettings};
    use gfx::traits::*;
    use piston_window::*;
    use shader_version::glsl::GLSL;
    use shader_version::Shaders;

    let opengl = OpenGL::V3_3;

    let mut window: PistonWindow = WindowSettings::new("piston: cube", [640, 480])
        .exit_on_esc(true)
        .samples(4)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_capture_cursor(true);

    let mut factory = window.factory.clone();
    let vertex_data = vec![
        Vertex::new([0, 0]), //0
    ];

    let index_data: &[u16] = &[
        0,
    ];

    let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, index_data);

    let texels = [
        [0xff, 0xff, 0xff, 0xff],
        [0xff, 0xff, 0xff, 0xff],
        [0xff, 0xff, 0xff, 0xff],
        [0xff, 0xff, 0xff, 0xff],
    ];
    let (_, texture_view) = factory
        .create_texture_immutable::<gfx::format::Rgba8>(
            gfx::texture::Kind::D2(2, 2, gfx::texture::AaMode::Single),
            gfx::texture::Mipmap::Provided,
            &[&texels],
        )
        .unwrap();
        
    let sinfo = gfx::texture::SamplerInfo::new(
        gfx::texture::FilterMethod::Bilinear,
        gfx::texture::WrapMode::Clamp,
    );

    let glsl = opengl.to_glsl();
    let pso = factory
        .create_pipeline_simple(
            Shaders::new()
                .set(GLSL::V1_50, include_str!("../assets/world_vertex.glsl"))
                .get(glsl)
                .unwrap()
                .as_bytes(),
            Shaders::new()
                .set(GLSL::V1_50, include_str!("../assets/world_fragment.glsl"))
                .get(glsl)
                .unwrap()
                .as_bytes(),
            pipe::new(),
        )
        .unwrap();

    let mut first_person = FirstPerson::new([0.5, 0.5, 4.0], FirstPersonSettings::keyboard_wasd());

    let mut data = pipe::Data {
        vbuf,
        color_mask: [0.0; 4],
        do_step: 1,
        one_pixel: [0.0; 2],
        kernel: [[0.0; 3]; 3],
        image: (texture_view, factory.create_sampler(sinfo)),
        out_color: window.output_color.clone(),
        out_depth: window.output_stencil.clone(),
    };
    while let Some(e) = window.next() {
        first_person.event(&e);

        window.draw_3d(&e, |window| {
            window
                .encoder
                .clear(&window.output_color, [0.3, 0.3, 0.3, 1.0]);
            window.encoder.clear_depth(&window.output_stencil, 1.0);
            window.encoder.draw(&slice, &pso, &data);
        });

        if e.resize_args().is_some() {
            data.out_color = window.output_color.clone();
            data.out_depth = window.output_stencil.clone();
        }
    }
}
