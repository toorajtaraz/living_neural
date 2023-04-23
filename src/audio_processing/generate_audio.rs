use super::chords::get_piches;
use super::generate_difference::ExtractedInformationFromBuffers;
use super::wav;
use fon::chan::Ch16;
use fon::{Audio, Frame};
use twang::noise::White;
use twang::ops::Gain;
use twang::osc::Sine;
use twang::Synth;
/// First ten harmonic volumes of a piano sample (sounds like electric piano).
const HARMONICS: [f32; 10] = [
    0.700, 0.243, 0.229, 0.095, 0.139, 0.087, 0.288, 0.199, 0.124, 0.090,
];
/// The three pitches in a perfectly tuned A3 minor chord
// const PITCHES: [f32; 3] = [220.0, 220.0 * 32.0 / 27.0, 220.0 * 3.0 / 2.0];
const PITCHES: [f32; 7] = [164.81377845643485, 184.9972113558171, 207.65234878997245, 219.9999999999999, 261.6255653005985, 277.182630976872, 293.66476791740746];
/// Volume of the piano
const VOLUME: f32 = 1.0 / 3.0;

// State of the synthesizer.
#[derive(Default)]
struct Processors {
    // All the chords
    chords: Vec<Vec<f32>>,
    // White noise generator.
    white: White,
    // 10 harmonics for 3 pitches.
    piano_1: [[Sine; 10]; 1],
    piano_2: [[Sine; 10]; 2],
    piano_3: [[Sine; 10]; 3],
    piano_4: [[Sine; 10]; 4],
    piano_5: [[Sine; 10]; 5],
    piano_6: [[Sine; 10]; 6],
    piano_7: [[Sine; 10]; 7],
    counter: usize,
    frame_counter: usize,
    index: usize,
}

pub fn generate_audio(
    audio_length: usize,
    exported_file_name: String,
    info: ExtractedInformationFromBuffers,
) {
    // Initialize audio
    let mut audio = Audio::<Ch16, 2>::with_silence(48_000, 48_000 * audio_length);
    // Create audio processors
    let mut proc = Processors {
        chords: get_piches(),
        ..Default::default()
    };
    // Adjust phases of harmonics.
    for pitch in proc.piano_1.iter_mut() {
        for harmonic in pitch.iter_mut() {
            harmonic.shift(proc.white.step());
        }
    }
    for pitch in proc.piano_2.iter_mut() {
        for harmonic in pitch.iter_mut() {
            harmonic.shift(proc.white.step());
        }
    }
    for pitch in proc.piano_3.iter_mut() {
        for harmonic in pitch.iter_mut() {
            harmonic.shift(proc.white.step());
        }
    }
    for pitch in proc.piano_4.iter_mut() {
        for harmonic in pitch.iter_mut() {
            harmonic.shift(proc.white.step());
        }
    }
    for pitch in proc.piano_5.iter_mut() {
        for harmonic in pitch.iter_mut() {
            harmonic.shift(proc.white.step());
        }
    }
    for pitch in proc.piano_6.iter_mut() {
        for harmonic in pitch.iter_mut() {
            harmonic.shift(proc.white.step());
        }
    }
    for pitch in proc.piano_7.iter_mut() {
        for harmonic in pitch.iter_mut() {
            harmonic.shift(proc.white.step());
        }
    }
    // Build synthesis algorithm
    let mut synth = Synth::new(proc, move |proc, mut frame: Frame<_, 2>| {
        // First we get width and height from info
        let width = info.width;
        let height = info.height;

        // we want to get the pixel at the current frame counter,
        // the difference dictates how many frames should be the same, and the difference min is 1 and max is 8
        let fc = proc.frame_counter;
        let mut index = proc.index;
        // difference is a 2d array
        let index_x = index % width as usize;
        let index_y = index / width as usize;
        let difference = info.difference[index_x][index_y];

        // index 0 refers to the [0][0] of the first buffer, and index 1 refers to the [0][0] of the second buffer
        let buff;
        if index % 2 == 0 {
            buff = &info.buffer_one;
        } else {
            buff = &info.buffer_two;
        }

        let index = index / 2;
        let index_x = index % width as usize;
        let index_y = index / width as usize;
        let pixel = buff[index_x][index_y];
        let number_of_chords = proc.chords.len() - 1;
        // pixel is a number between 0 and 255, we want to map it to a number between 0 and number_of_chords
        let chord_index = (pixel as f32 / 255.0 * number_of_chords as f32) as usize;
        let chord = &proc.chords[chord_index];
        // println!("chord: {:?}", chord);

        match chord.len() {
            1 => {
                for (s, pitch) in proc.piano_1.iter_mut().zip(chord.iter()) {
                    for ((i, o), v) in s.iter_mut().enumerate().zip(HARMONICS.iter()) {
                        // Get next sample from oscillator.
                        let sample = o.step(pitch * (i + 1) as f32);
                        // Pan the generated harmonic center
                        frame = frame.pan(Gain.step(sample, (v * VOLUME).into()), 0.0);
                    }
                }
            }
            2 => {
                for (s, pitch) in proc.piano_2.iter_mut().zip(chord.iter()) {
                    for ((i, o), v) in s.iter_mut().enumerate().zip(HARMONICS.iter()) {
                        // Get next sample from oscillator.
                        let sample = o.step(pitch * (i + 1) as f32);
                        // Pan the generated harmonic center
                        frame = frame.pan(Gain.step(sample, (v * VOLUME).into()), 0.0);
                    }
                }
            }
            3 => {
                for (s, pitch) in proc.piano_3.iter_mut().zip(chord.iter()) {
                    for ((i, o), v) in s.iter_mut().enumerate().zip(HARMONICS.iter()) {
                        // Get next sample from oscillator.
                        let sample = o.step(pitch * (i + 1) as f32);
                        // Pan the generated harmonic center
                        frame = frame.pan(Gain.step(sample, (v * VOLUME).into()), 0.0);
                    }
                }
            }
            4 => {
                for (s, pitch) in proc.piano_4.iter_mut().zip(chord.iter()) {
                    for ((i, o), v) in s.iter_mut().enumerate().zip(HARMONICS.iter()) {
                        // Get next sample from oscillator.
                        let sample = o.step(pitch * (i + 1) as f32);
                        // Pan the generated harmonic center
                        frame = frame.pan(Gain.step(sample, (v * VOLUME).into()), 0.0);
                    }
                }
            }
            5 => {
                for (s, pitch) in proc.piano_5.iter_mut().zip(chord.iter()) {
                    for ((i, o), v) in s.iter_mut().enumerate().zip(HARMONICS.iter()) {
                        // Get next sample from oscillator.
                        let sample = o.step(pitch * (i + 1) as f32);
                        // Pan the generated harmonic center
                        frame = frame.pan(Gain.step(sample, (v * VOLUME).into()), 0.0);
                    }
                }
            }
            6 => {
                for (s, pitch) in proc.piano_6.iter_mut().zip(chord.iter()) {
                    for ((i, o), v) in s.iter_mut().enumerate().zip(HARMONICS.iter()) {
                        // Get next sample from oscillator.
                        let sample = o.step(pitch * (i + 1) as f32);
                        // Pan the generated harmonic center
                        frame = frame.pan(Gain.step(sample, (v * VOLUME).into()), 0.0);
                    }
                }
            }
            7 => {
                for (s, pitch) in proc.piano_7.iter_mut().zip(chord.iter()) {
                    for ((i, o), v) in s.iter_mut().enumerate().zip(HARMONICS.iter()) {
                        // Get next sample from oscillator.
                        let sample = o.step(pitch * (i + 1) as f32);
                        // Pan the generated harmonic center
                        frame = frame.pan(Gain.step(sample, (v * VOLUME).into()), 0.0);
                    }
                }
            }
            _ => {}
        }
        // for (s, pitch) in proc.piano_7.iter_mut().zip(PITCHES.iter()) {
        //     for ((i, o), v) in s.iter_mut().enumerate().zip(HARMONICS.iter()) {
        //         // Get next sample from oscillator.
        //         let sample = o.step(pitch * (i + 1) as f32);
        //         // Pan the generated harmonic center
        //         frame = frame.pan(Gain.step(sample, (v * VOLUME).into()), 0.0);
        //     }
        // }
        proc.frame_counter += 1;
        proc.counter += 1;
        if fc == difference as usize {
            proc.index += 1;
            proc.frame_counter = 0;
        }
        // println!("counter: {}", proc.counter);
        frame
    });
    // Synthesize audio
    synth.stream(audio.sink());
    // Write synthesized audio to WAV file
    wav::write(audio, exported_file_name.as_str()).expect("Failed to write WAV file");
}
