use glium::texture::RawImage2d;

pub struct GeneratedDifference {
    pub difference: Vec<i16>,
}

pub fn generate_difference(
    buffer_one: RawImage2d<u8>,
    buffer_two: RawImage2d<u8>
) -> GeneratedDifference {
    let x = buffer_one.width;
    let y = buffer_one.height;
    let mut difference_buffer = Vec::with_capacity((x * y) as usize);
    // Each pixel is 4 bytes, we don't need rgb channels, so we can just use the alpha channel
    for i in 0..buffer_one.data.len() {
        let difference: i16 = (buffer_one.data[i] as i16) - (buffer_two.data[i] as i16);
        difference_buffer.push(difference);

    }
    GeneratedDifference {
        difference: difference_buffer,
    }
}