use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::time::Instant;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;
const FRACTAL_DEPTH: u32 = 32;
const GENERATION_INFINITY: f64 = 4.;

#[derive(Debug)]
struct FastTimer {
    sum: u32,
    timings: [u32; 30],
    index: usize,
    instant: Instant,
}

impl FastTimer {
    fn new() -> Self {
        FastTimer {
            sum: 0,
            timings: [0;30],
            index: 0,
            instant: Instant::now(),
        }
    }

    fn start_frame(self: &mut Self) {
        self.instant = Instant::now();
    }

    fn end_frame(self: &mut Self) {
        let elapsed = self.instant.elapsed().as_millis();
        let last_val = self.timings[self.index];
        self.timings[self.index] = elapsed as u32;

        self.sum -= last_val;
        self.sum += elapsed as u32;
        self.index = (self.index + 1) % self.timings.len();
    }

    fn average(self: &Self) -> u32 {
        self.sum / (self.timings.len() as u32)
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Fractal - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale: Scale::X2,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to Open Window");

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let range = 2.0;
    let x_min = 0. - range;
    let y_min = 0. - range;

    let x_max = 0. + range;
    let y_max = 0. + range;

    let mut angle: f64 = 0.0;

    window.set_background_color(0, 0, 20);

    let mut timer = FastTimer::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        timer.start_frame();
        for i in 0..buffer.len() {
            let mut real = map((i % WIDTH) as f64, 0., WIDTH as f64, x_min, x_max);
            let mut imag = map((i / HEIGHT) as f64, 0., HEIGHT as f64, y_min, y_max);

            let mut n = 0;

            while n < FRACTAL_DEPTH {
                let re = real.powf(2.) - imag.powf(2.);
                let im = 2. * real * imag;

                real = re + angle.cos();
                imag = im + angle.sin();

                if (real + imag).abs() > GENERATION_INFINITY {
                    break; // Leave when achieve infinity
                }
                n += 1;
            }

            buffer[i] = fill(n);
        }

        angle += 0.01;
        timer.end_frame();

        if timer.index == 0 {
            let avg = timer.average();
            println!("{} ms or {} fps", avg, 1000 / avg);
        }
        // We unwrap here as we want this code to exit if it fails
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn map(val: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
    start2 + (stop2 - start2) * ((val - start1) / (stop1 - start1))
}

fn fill(n: u32) -> u32 {
    if FRACTAL_DEPTH == n {
        return 0x00;
    } else {
        return n * 32 % 255;
    }
}
