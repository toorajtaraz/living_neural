// #[macro_use]
// extern crate glium;

// macro_rules! dopanic {
//     ($err:expr) => {{
//         eprintln!("Error: {}", $err);
//         std::process::exit(1);
//     }};
// }

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
//     use glutin::dpi::PhysicalSize;

//     let conf = cmd_handler::get_args();
//     let width: u32;
//     let height: u32;
//     let fps: f32;
//     let kernel_2d;
//     let color;
//     let skip;

//     width = conf.width;
//     height = conf.height;
//     fps = conf.fps;
//     kernel_2d = conf.kernel;
//     color = conf.color;
//     skip = conf.skip;

//     let mut single_pixel = [1.0f32 / width as f32, 1.0 / height as f32];

//     let fragment_src = conf.fragment_src;

//     let event_loop = glutin::event_loop::EventLoop::new();
//     let wb = glutin::window::WindowBuilder::new()
//         .with_always_on_top(true)
//         .with_inner_size(PhysicalSize::new(height, width))
//         .with_resizable(false);
//     let cb = glutin::ContextBuilder::new();
//     let display = glium::Display::new(wb, cb, &event_loop).unwrap_or_else(|err| dopanic!(err));

//     let points = vec![
//         Vertex::new(-1.0, -1.0),
//         Vertex::new(1.0, -1.0),
//         Vertex::new(-1.0, 1.0),
//         Vertex::new(1.0, 1.0),
//     ];
//     let vertex_buffer =
//         glium::VertexBuffer::new(&display, &points).unwrap_or_else(|err| dopanic!(err));
//     let indices = glium::IndexBuffer::new(
//         &display,
//         PrimitiveType::TrianglesList,
//         &[0, 1, 2, 3, 2, 1u16],
//     )
//     .unwrap_or_else(|err| dopanic!(err));
//     let program = glium::Program::from_source(
//         &display,
//         shaders::vertex::VERTEX_SRC,
//         fragment_src.as_str(),
//         None,
//     )
//     .unwrap_or_else(|err| dopanic!(err));

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
//                     return;
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
//         let mut target_fb = glium::framebuffer::SimpleFrameBuffer::new(&display, &dest_texture)
//             .unwrap_or_else(|err| dopanic!(err));
//         if is_first {
//             let uniforms = uniform! {
//                 u_kernel: kernel_2d,
//                 u_kernel_height: 3,
//                 u_kernel_width: 3,
//                 u_do_calc: do_calc,
//                 u_color_mask: color,
//                 u_single_pixel: single_pixel,
//                 u_plane : u_plane_base
//                             .sampled()
//                             .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat),
//             };

//             target_fb
//                 .draw(
//                     &vertex_buffer,
//                     &indices,
//                     &program,
//                     &uniforms,
//                     &Default::default(),
//                 )
//                 .unwrap_or_else(|err| dopanic!(err));
//             target_fb.fill(
//                 &dest_texture.as_surface(),
//                 glium::uniforms::MagnifySamplerFilter::Linear,
//             );

//             is_first = false;
//             do_calc = false;
//         } else {
//             let uniforms = uniform! {
//                 u_kernel: kernel_2d,
//                 u_kernel_height: 3,
//                 u_kernel_width: 3,
//                 u_do_calc: do_calc,
//                 u_color_mask: color,
//                 u_single_pixel: single_pixel,
//                 u_plane : dest_texture
//                             .sampled()
//                             .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat),
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
//                         .unwrap_or_else(|err| dopanic!(err));

//                     target_fb.fill(
//                         &dest_texture.as_surface(),
//                         glium::uniforms::MagnifySamplerFilter::Linear,
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
//                     .unwrap_or_else(|err| dopanic!(err));

//                 target.finish().unwrap_or_else(|err| dopanic!(err));
//                 do_calc = true;
//             }
//         }

//         let next_frame_time = std::time::Instant::now()
//             + std::time::Duration::from_nanos(((1.0 / fps) * 1_000_000f32) as u64);
//         *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
//     });
// }



//! A Minor on an Electric Piano

use fon::chan::Ch16;
use fon::{Audio, Frame};
use twang::noise::White;
use twang::ops::Gain;
use twang::osc::Sine;
use twang::Synth;
use rand::prelude::*;

mod wav {
    use fon::chan::Ch16;
    use fon::pos::{Left, Right};
    use fon::Audio;
    use std::convert::TryInto;
    use std::{fs, io, mem::size_of};
    
    /// Write a 16-bit PCM WAV file
    pub(super) fn write(audio: Audio<Ch16, 2>, filename: &str) -> io::Result<()> {
        let mut buf = vec![];
        write_header(&mut buf, &audio);
        write_fmt_header(&mut buf, &audio);
        write_audio_data(&mut buf, &audio);
        fs::write(filename, buf)
    }
    
    fn write_header(buf: &mut Vec<u8>, audio: &Audio<Ch16, 2>) {
        // Predict size of WAV subchunks.
        let n: u32 = audio.len().try_into().unwrap();
        // RIFF Chunk: ckID
        buf.extend(b"RIFF");
        // RIFF Chunk: cksize
        buf.extend(&(36u32 + n).to_le_bytes());
        // RIFF Chunk: WAVEID
        buf.extend(b"WAVE");
    }
    
    fn write_fmt_header(buf: &mut Vec<u8>, audio: &Audio<Ch16, 2>) {
        // RIFF Subchunk: "fmt "
        buf.extend(b"fmt ");
        // Chunk size: 16, 18 or 40
        buf.extend(&(16u32).to_le_bytes());
        // 0: WAVE_FORMAT_PCM
        buf.extend(&(0x0001u16).to_le_bytes());
        // 2: Stereo
        buf.extend(&(2u16).to_le_bytes());
        // 4: Sampling Rate
        buf.extend(&u32::from(audio.sample_rate()).to_le_bytes());
        // 8: Bytes per second (i16 * 2 * sample rate)
        buf.extend(&(4 * u32::from(audio.sample_rate())).to_le_bytes());
        // 12. Data block size (bytes: i16 * 2)
        buf.extend(&(size_of::<u16>() as u16 * 2u16).to_le_bytes());
        // 14. Bits per sample
        buf.extend(&(16u16).to_le_bytes());
    }
    
    fn write_audio_data(buf: &mut Vec<u8>, audio: &Audio<Ch16, 2>) {
        // RIFF Subchunk: "data"
        buf.extend(b"data");
        // cksize (Bytes): Stereo (2) * i16 (2) * Frame Length
        buf.extend(&(4 * audio.len() as u32).to_le_bytes());
        // Sampled data
        for frame in audio.iter() {
            // Add left channel
            buf.extend(&i16::from(frame[Left]).to_le_bytes());
            // Add right channel
            buf.extend(&i16::from(frame[Right]).to_le_bytes());
        }
    }
}
// /// First ten harmonic volumes of a piano sample (sounds like electric piano).
// const HARMONICS: [f32; 10] = [
//     0.700, 0.243, 0.229, 0.095, 0.139, 0.087, 0.288, 0.199, 0.124, 0.090,
// ];
// // const HARMONICS: [f32; 20] = [
// //     1.000, 0.498, 0.330, 0.235, 0.183, 0.152, 0.130, 0.115, 0.102, 0.093, 0.084, 0.078, 0.072, 0.068, 0.064, 0.061, 0.058, 0.055, 0.053, 0.051
// // ];
// /// The three pitches in a perfectly tuned A3 minor chord
// const PITCHES: [f32; 3] = [220.0, 220.0 * 32.0 / 27.0, 220.0 * 3.0 / 2.0];
// /// Volume of the piano
// const VOLUME: f32 = 1.0 / 3.0;

// // State of the synthesizer.
// #[derive(Default)]
// struct Processors {
//     // White noise generator.
//     white: White,
//     // 10 harmonics for 3 pitches.
//     piano: [[Sine; 20]; 3],
// }

// fn main() {
//     // Initialize audio
//     let mut audio = Audio::<Ch16, 2>::with_silence(48_000, 48_000 * 5);
//     // Create audio processors
//     let mut proc = Processors::default();
//     // Adjust phases of harmonics.
//     for pitch in proc.piano.iter_mut() {
//         for harmonic in pitch.iter_mut() {
//             harmonic.shift(proc.white.step());
//         }
//     }
//     // Build synthesis algorithm
//     let mut synth = Synth::new(proc, |proc, mut frame: Frame<_, 2>| {
//         for (s, pitch) in proc.piano.iter_mut().zip(PITCHES.iter()) {
//             for ((i, o), v) in s.iter_mut().enumerate().zip(HARMONICS.iter()) {
//                 // Get next sample from oscillator.
//                 let sample = o.step(pitch * (i + 1) as f32);
//                 // Pan the generated harmonic center
//                 // let mut rng = rand::thread_rng();
//                 // let volume_rand = rng.gen::<u8>();
//                 // let volume = volume_rand as f32 / 255.0;
//                 // println!("volume: {}", volume);
//                 frame = frame.pan(Gain.step(sample, (v * VOLUME).into()), 0.0);
//             }
//         }
//         frame
//     });
//     // Synthesize 5 seconds of audio
//     synth.stream(audio.sink());
//     // Write synthesized audio to WAV file
//     wav::write(audio, "piano.wav").expect("Failed to write WAV file");
// }

use cpal::traits::{DeviceTrait, StreamTrait, HostTrait};
use rand::Rng;

const SAMPLE_RATE: u32 = 44100;
const AMPLITUDE: f32 = 0.25;
const DECAY: f32 = 0.99;
const MAX_HARMONICS: usize = 50;

fn main() {
    let device = cpal::default_host().default_output_device().expect("failed to find a default output device");
    let config = device.default_output_config().expect("failed to get default output config");
    let sample_rate = config.sample_rate().0;
    let channels = config.channels();

    let mut harmonics = [0.0; MAX_HARMONICS];
    for i in 1..=MAX_HARMONICS {
        harmonics[i - 1] = rand::thread_rng().gen_range(0.0..1.0) / i as f32;
    }

    let mut amplitude = AMPLITUDE;
    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.chunks_mut(channels as usize) {
                let mut value = 0.0;
                for (i, harmonic) in harmonics.iter().enumerate() {
                    let frequency = (i + 1) as f32;
                    value += amplitude * harmonic * (2.0 * std::f32::consts::PI * frequency * SAMPLE_RATE as f32).sin();
                }
                for channel in sample.iter_mut() {
                    *channel = value;
                }
                amplitude *= DECAY;
            }
        },
        |err| eprintln!("an error occurred on stream: {}", err),
        None
    ).expect("failed to build output stream");

    stream.play().expect("failed to play stream");

    std::thread::sleep(std::time::Duration::from_secs(10));
}