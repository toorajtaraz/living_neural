use glium::texture::RawImage2d;

pub struct ExtractedInformationFromBuffers {
    pub difference: Vec<Vec<i16>>,
    pub buffer_one: Vec<Vec<u8>>,
    pub buffer_two: Vec<Vec<u8>>,
}

pub fn generate_difference(
    buffer_one: RawImage2d<u8>,
    buffer_two: RawImage2d<u8>
) -> ExtractedInformationFromBuffers {
    let x = buffer_one.width;
    let y = buffer_one.height;
    let mut buff_one: Vec<Vec<u8>> = Vec::with_capacity(x as usize);
    let mut buff_two: Vec<Vec<u8>> = Vec::with_capacity(x as usize);
    let mut diff_buff: Vec<Vec<i16>> = Vec::with_capacity(x as usize);
    for i in 0..x {
        let mut row_one: Vec<u8> = Vec::with_capacity(y as usize);
        let mut row_two: Vec<u8> = Vec::with_capacity(y as usize);
        let mut diff_row: Vec<i16> = Vec::with_capacity(y as usize);
        for j in 0..y {
            let index = (i * x + j) as usize;
            row_one.push(buffer_one.data[index]);
            row_two.push(buffer_two.data[index]);
            diff_row.push((buffer_one.data[index] as i16) - (buffer_two.data[index] as i16));
        }
        buff_one.push(row_one);
        buff_two.push(row_two);
        diff_buff.push(diff_row);
    }
    ExtractedInformationFromBuffers {
        difference: diff_buff,
        buffer_one: buff_one,
        buffer_two: buff_two,
    }
}