use macroquad::{prelude::*, miniquad::window::{screen_size, set_window_size}};

// TODO file region selection, higher dim
// TODO UI (using macroquad ui mod?)
// TODO better CLI args

#[macroquad::main("file_vis")]
async fn main() {
    set_window_size(768, 768);
    let file_content = std::fs::read(std::env::args().nth(1).unwrap()).unwrap();

    let mut patterns_count = vec![0u32; 256*256];

    for pat in file_content.windows(2) {
        let index = (pat[0] as usize) << 8 | pat[1] as usize;
        patterns_count[index] += 1;
    }

    let max = *patterns_count.iter().max().unwrap();

    let mut pixel_data = vec![0; 256*256*4];

    for (col, val) in pixel_data.chunks_exact_mut(4).zip(patterns_count) {
        let pix_val = ((val as f32 + 1.).log2() / ((max as f32 + 1.).log2() + 1.) * 255.) as u8;

        col[0] = pix_val;
        col[1] = pix_val;
        col[2] = pix_val;
        col[3] = 255;
    }

    let texture = Texture2D::from_rgba8(256, 256, &pixel_data);
    texture.set_filter(FilterMode::Nearest);

    loop {
        draw_texture_ex(&texture, 0., 0., WHITE, DrawTextureParams {
            dest_size: Some((768., 768.).into()),
            ..Default::default()
        });

        next_frame().await
    }
}