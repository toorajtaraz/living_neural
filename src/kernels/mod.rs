use rand::prelude::*;

#[allow(dead_code)]
pub enum Kernel {
    RANDOM,
    WORM,
    FIBER,
    WAVES,
    RULE30,
    GAMEOFLIFE,
}

pub fn get_kernel(kernel_type: Kernel, gen_range: Option<(f32, f32)>) -> [[f32; 3]; 3] {
    match kernel_type {
        Kernel::WORM => [
            [0.68, -0.90, 0.68],
            [-0.9, -0.66, -0.90],
            [0.68, -0.90, 0.68f32],
        ],
        Kernel::FIBER => [
            [0.037, 0.43, -0.737],
            [0.406, -0.321, -0.319],
            [-0.458, 0.416, 0.478f32],
        ],
        Kernel::WAVES => [
            [0.565, -0.716, 0.565],
            [-0.716, 0.627, -0.716],
            [0.565, -0.716, 0.565],
        ],
        Kernel::RANDOM => {
            let mut rng = rand::thread_rng();
            let mut temp_ker = [[0., 0., 0.], [0., 0., 0.], [0., 0., 0f32]];
            let gen_range = gen_range.unwrap_or((-1.0, 1.0));
            for i in 0..3 {
                for j in 0..3 {
                    temp_ker[i][j] = rng.gen_range(gen_range.0..=gen_range.1);
                }
            }
            temp_ker
        }
        Kernel::RULE30 => [[0., 0., 0.], [0., 0., 0.], [1.0, 2.0, 4.0f32]],
        Kernel::GAMEOFLIFE => [[1., 1., 1.], [1., 9., 1.], [1.0, 1.0, 1.0f32]],
    }
}
