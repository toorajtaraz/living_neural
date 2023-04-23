/*
MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 */
/*
   Credits to: https://github.com/AldaronLau/twang/blob/stable/examples/wav/mod.rs
*/
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
