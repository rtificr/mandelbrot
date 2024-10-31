use minifb::{Key, Window, WindowOptions};
use rayon::prelude::*;
use std::time::Instant;

const WIDTH: usize = 2000 / 3;
const HEIGHT: usize = 2000 / 3;
const SUPERSAMPLE: usize = 2;
const SAMPLES: usize = 2;

fn main() {
    let mut buffer = [0u32; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.set_target_fps(60);

    let mut iter = 10;
    mandelbrot_to_buf(&mut buffer, iter);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Up) {
            let now = Instant::now();
            iter += 1;
            mandelbrot_to_buf(&mut buffer, iter);
            window.set_title(&format!(
                "Mandelbrot - Iter: {} - Update time: {:.8}s",
                iter,
                now.elapsed().as_secs_f64()
            ));
        }
        if window.is_key_down(Key::Down) {
            let now = Instant::now();
            if iter > 1 {
                iter -= 1
            };
            mandelbrot_to_buf(&mut buffer, iter);
            window.set_title(&format!(
                "Mandelbrot - Iter: {} - Update time: {:.8}s",
                iter,
                now.elapsed().as_secs_f64()
            ));
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn mandelbrot_to_buf(buf: &mut [u32], max_iter: usize) {
    buf.par_chunks_mut(WIDTH).enumerate().for_each(|(y, row)| {
        row.iter_mut().enumerate().for_each(|(x, pixel)| {
            let mut total_val = 0.0;
            let x_base = (x as isize - (WIDTH / 2) as isize) as f32 / (WIDTH as f32 / 4.0);
            let y_base = (y as isize - (HEIGHT / 2) as isize) as f32 / (HEIGHT as f32 / 4.0);
            for i in 0..SAMPLES {
                for j in 0..SAMPLES {
                    let x_offset = (i as f32 / SAMPLES as f32 - 0.5) / (WIDTH as f32 / 4.0);
                    let y_offset = (j as f32 / SAMPLES as f32 - 0.5) / (HEIGHT as f32 / 4.0);
                    total_val += mandelbrot(x_base + x_offset, y_base + y_offset, max_iter);
                }
            }
            let avg_val = total_val / (SAMPLES * SAMPLES) as f32;
            *pixel = ((avg_val * 255.0) as u8 as u32) << 16
                | ((avg_val * 255.0) as u8 as u32) << 8
                | (avg_val * 255.0) as u8 as u32;
        });
    });
}
fn set_pix(x: isize, y: isize, color: (u8, u8, u8), buf: &mut [u32]) {
    if x.abs() >= (WIDTH / 2) as isize || y.abs() >= (HEIGHT / 2) as isize {
        return;
    }
    let color = (color.0 as u32) << 16 | (color.1 as u32) << 8 | color.2 as u32;
    buf[(((HEIGHT / 2) as isize + y) * WIDTH as isize + ((WIDTH / 2) as isize + x)) as usize] =
        color;
}

fn mandelbrot(x: f32, y: f32, max_iter: usize) -> f32 {
    let mut z_real: f32 = 0.0;
    let mut z_imag: f32 = 0.0;
    let mut z_mag_squared = 0.0;
    let c_real = x;
    let c_imag = y;

    for i in 0..max_iter {
        z_mag_squared = z_real.powf(2.) + z_imag.powf(2.).abs();

        if z_mag_squared > 4.0 {
            return i as f32 / max_iter as f32;
        }

        let new_z_real = z_real * z_real - z_imag * z_imag + c_real;
        let new_z_imag = 2.0 * z_real * z_imag + c_imag;
        z_real = new_z_real;
        z_imag = new_z_imag;
    }

    0.0
}
