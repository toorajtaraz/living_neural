use crate::kernels;
use crate::shaders;
use clap::{App, Arg};
use rand::prelude::*;

#[derive(Default, Debug)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub fps: f32,
    pub kernel: [[f32; 3]; 3],
    pub color: [f32; 4],
    pub skip: u32,
    pub fragment_src: String,
    pub is_persistent: bool,
}

pub fn get_args() -> Config {
    let matches = App::new(format!("Living Neural"))                       
                          .version("\r0.1.0")
                          .author("Tooraj Taraz <tooraj.info@gmail.com>")
                          .about("Living Neural implements a simple neural automata accelerated by OPENGL.")
                          .arg(Arg::with_name("width")
                               .short('w')
                               .long("width")
                               .value_name("desired window width")
                               .help("Sets initial window width, can be resized later. Must be 32 bit unsigned int and greater than 0.")
                                .value_parser(clap::value_parser!(u32))
                               .takes_value(true))
                          .arg(Arg::with_name("height")
                               .short('h')
                               .long("height")
                               .value_name("desired window height")
                               .help("Sets initial window height, can be resized later. Must be 32 bit unsigned int and greater than 0.")
                                .value_parser(clap::value_parser!(u32))
                               .takes_value(true))
                          .arg(Arg::with_name("fps")
                               .short('f')
                               .long("fps")
                               .value_name("desired frame count per second")
                               .help("Sets the FPS. Must be 32 bit positive floating point and greater than 0.0.")
                                .value_parser(clap::value_parser!(f32))
                               .takes_value(true))
                          .arg(Arg::with_name("kernel")
                               .short('k')
                               .long("kernel")
                               .value_name("desired kernel name/mode")
                               .help("Sets the kernel.")
                               .takes_value(true))
                          .arg(Arg::with_name("color_by_name")
                               .short('c')
                               .long("color")
                               .value_name("desired color name")
                               .help("Sets the fg color.")
                               .takes_value(true))
                          .arg(Arg::with_name("color_by_code")
                               .short('C')
                               .long("COLOR")
                               .value_name("desired color code")
                               .help("Sets the fg color.")
                               .takes_value(true))
                          .arg(Arg::with_name("skip")
                               .short('s')
                               .long("skip")
                               .value_name("frames to skip")
                               .help("Sets the number of frames to skip.")
                                .value_parser(clap::value_parser!(u32))
                               .takes_value(true))
                          .arg(Arg::with_name("activation")
                               .short('a')
                               .long("activation")
                               .value_name("activation name or source")
                               .help("Sets the activation function.")
                               .takes_value(true))
                          .arg(Arg::with_name("persistent")
                               .short('p')
                               .long("persistent")
                               .help("If passed extends the fragment shader with pixel persistence."))
                          .get_matches();

    let mut conf = Config::default();
    let mut rng = rand::thread_rng();
    if let Some(_) = matches.get_one::<u8>("persistent") {
        conf.is_persistent = true;
    } else {
        conf.is_persistent = false;
    }

    if let Some(_val) = matches.get_one::<String>("color_by_name") {
        panic!("NOT IMPLEMENTED");
    } else if let Some(_val) = matches.get_one::<String>("color_by_code") {
        panic!("NOT IMPLEMENTED");
    } else {
        conf.color = [
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            1.0,
        ]
    }

    if let Some(val) = matches.get_one::<String>("kernel") {
        let val: String = val.split_whitespace().collect();
        if val.eq_ignore_ascii_case("worm") {
            conf.kernel = kernels::get_kernel(kernels::Kernel::WORM, None);
        } else if val.eq_ignore_ascii_case("fiber") {
            conf.kernel = kernels::get_kernel(kernels::Kernel::FIBER, None);
        } else if val.eq_ignore_ascii_case("rule30") {
            conf.kernel = kernels::get_kernel(kernels::Kernel::RULE30, None);
        } else if val.eq_ignore_ascii_case("waves") {
            conf.kernel = kernels::get_kernel(kernels::Kernel::WAVES, None);
        } else if val.eq_ignore_ascii_case("gameoflife") {
            conf.kernel = kernels::get_kernel(kernels::Kernel::GAMEOFLIFE, None);
        } else if val.eq_ignore_ascii_case("random") {
            conf.kernel = kernels::get_kernel(kernels::Kernel::RANDOM, None);
        } else {
            panic!("unkown kernel");
        }
    } else {
        conf.kernel = kernels::get_kernel(kernels::Kernel::RANDOM, None);
    }

    //TODO: FIX BAD INPUT
    if let Some(width) = matches.get_one::<u32>("width") {
        if *width == 0 {
            panic!("Width cannot be 0");
        }
        conf.width = *width;
    } else {
        conf.width = 500;
    }

    if let Some(height) = matches.get_one::<u32>("height") {
        if *height == 0 {
            panic!("Width cannot be 0");
        }
        conf.height = *height;
    } else {
        conf.height = 500;
    }

    if let Some(skip) = matches.get_one::<u32>("skip") {
        if *skip == 0 {
            panic!("Connot skip none");
        }
        conf.skip = *skip;
    } else {
        conf.skip = 1;
    }

    if let Some(activation_src_name) = matches.get_one::<String>("activation") {
        let val: String = activation_src_name.split_whitespace().collect();
        if val.eq_ignore_ascii_case("sin") {
            conf.fragment_src = shaders::fragment::get_fragment_shader(
                shaders::fragment::Activation::SIN,
                conf.is_persistent,
                None,
            );
        } else if val.eq_ignore_ascii_case("tanh") {
            conf.fragment_src = shaders::fragment::get_fragment_shader(
                shaders::fragment::Activation::TANH,
                conf.is_persistent,
                None,
            );
        } else if val.eq_ignore_ascii_case("power") {
            conf.fragment_src = shaders::fragment::get_fragment_shader(
                shaders::fragment::Activation::POWER,
                conf.is_persistent,
                None,
            );
        } else if val.eq_ignore_ascii_case("rule30") {
            conf.fragment_src = shaders::fragment::get_fragment_shader(
                shaders::fragment::Activation::RULE30,
                conf.is_persistent,
                None,
            );
        } else if val.eq_ignore_ascii_case("gameoflife") {
            conf.fragment_src = shaders::fragment::get_fragment_shader(
                shaders::fragment::Activation::GAMEOFLIFE,
                conf.is_persistent,
                None,
            );
        } else if val.eq_ignore_ascii_case("inversegaussian") {
            conf.fragment_src = shaders::fragment::get_fragment_shader(
                shaders::fragment::Activation::INVERSEGAUSSIAN,
                conf.is_persistent,
                None,
            );
        } else if val.eq_ignore_ascii_case("waves") {
            conf.fragment_src = shaders::fragment::get_fragment_shader(
                shaders::fragment::Activation::WAVES,
                conf.is_persistent,
                None,
            );
        } else if val.eq_ignore_ascii_case("worm") {
            conf.fragment_src = shaders::fragment::get_fragment_shader(
                shaders::fragment::Activation::WORMS,
                conf.is_persistent,
                None,
            );
        } else if val.eq_ignore_ascii_case("absolute") {
            conf.fragment_src = shaders::fragment::get_fragment_shader(
                shaders::fragment::Activation::ABSOLUTE,
                conf.is_persistent,
                None,
            );
        } else if val.eq_ignore_ascii_case("identity") {
            conf.fragment_src = shaders::fragment::get_fragment_shader(
                shaders::fragment::Activation::IDENTITY,
                conf.is_persistent,
                None,
            );
        } else {
            conf.fragment_src = shaders::fragment::get_fragment_shader(
                shaders::fragment::Activation::CUSTOM,
                conf.is_persistent,
                Some(activation_src_name.clone()),
            );
        }
    } else {
        conf.fragment_src = shaders::fragment::get_fragment_shader(
            shaders::fragment::Activation::IDENTITY,
            conf.is_persistent,
            None,
        );
    }

    if let Some(fps) = matches.get_one::<f32>("fps") {
        if *fps <= 0.0 {
            panic!("FPS cannot be 0 or negative.")
        }
        conf.fps = *fps;
    } else {
        conf.fps = 20.0;
    }
    conf
}
