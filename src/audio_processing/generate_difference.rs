use glium::texture::RawImage2d;

pub struct ExtractedInformationFromBuffers {
    pub difference: Vec<Vec<usize>>,
    pub buffer_one: Vec<Vec<u8>>,
    pub buffer_two: Vec<Vec<u8>>,
    pub width: u32,
    pub height: u32,
}

const MIN_DIFF: usize = 48_000 / 8;
const MAX_DIFF: usize = 48_000;

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
    let min_diff = diff_buff.iter().flatten().min().unwrap().to_owned();
    let max_diff = diff_buff.iter().flatten().max().unwrap().to_owned();
    let mut diff_buff_reranged = Vec::new();
    for row in diff_buff {
        let mut temp = Vec::new();
        for pixel in row {
            let temp_pixel = (pixel - min_diff) as f32 / (max_diff - min_diff) as f32;
            let temp_pixel = temp_pixel * (MAX_DIFF - MIN_DIFF) as f32 + MIN_DIFF as f32;
            let ceiled_pixel = temp_pixel.ceil() as usize;
            temp.push(ceiled_pixel);
        }
        diff_buff_reranged.push(temp);
    }
    ExtractedInformationFromBuffers {
        difference: diff_buff_reranged,
        buffer_one: buff_one,
        buffer_two: buff_two,
        width: x,
        height: y,
    }
}