use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

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

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    mandelbrot_to_buf(&mut buffer, 1);

    let mut iter = 1;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        if window.is_key_pressed(Key::Up, minifb::KeyRepeat::No) {
            iter += 2;
            mandelbrot_to_buf(&mut buffer, iter);
        }
        if window.is_key_pressed(Key::Down, minifb::KeyRepeat::No) {
            if iter > 2 { iter -= 2 };
            mandelbrot_to_buf(&mut buffer, iter);
        }
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

fn mandelbrot_to_buf(buf: &mut [u32], max_iter: usize) {
    for y in 0-HEIGHT as isize / 2..HEIGHT as isize / 2 {
        for x in 0-WIDTH as isize / 2..WIDTH as isize / 2 {
            let val =  mandelbrot(
                x as f64 / (WIDTH as f64 / 4.0),
                y as f64 / (HEIGHT as f64 / 4.0),
                max_iter
            );
            set_pix(x, y, if val { (0, 0, 0) } else { (255,255,255) }, buf);
        }
    }
}

fn set_pix(x: isize, y: isize, color: (u8, u8, u8), buf: &mut [u32]) {
    if x.abs() >= (WIDTH / 2) as isize || y.abs() >= (HEIGHT / 2) as isize {
        return;
    }
    let color = (color.0 as u32) << 16 | (color.1 as u32) << 8 | color.2 as u32;
    buf[(
        ((HEIGHT / 2) as isize + y) * WIDTH as isize + ((WIDTH / 2) as isize + x)
    ) as usize] = color;
}

fn mandelbrot(x: f64, y: f64, max_iter: usize) -> bool {
    let mut z_real: f64 = 0.0;  // Real part of z
    let mut z_imag: f64 = 0.0;  // Imaginary part of z
    let mut z_mag_squared = 0.0;
    let c_real = x;
    let c_imag = y;

    for _ in 0..max_iter {
        z_mag_squared = z_real.powf(2.) + z_imag.powf(2.).abs();

        if z_mag_squared > 4.0 {
            return true
        }

        // Calculate the next z = z^2 + c
        let new_z_real = z_real * z_real - z_imag * z_imag + c_real ;
        let new_z_imag = 2.0 * z_real * z_imag + c_imag;

        // Update z_real and z_imag for the next iteration
        z_real = new_z_real;
        z_imag = new_z_imag;
    }

    // If we complete all iterations, we assume the point is in the Mandelbrot set
    false
}