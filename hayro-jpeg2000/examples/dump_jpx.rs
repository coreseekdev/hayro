fn main() {
    let data = std::fs::read("jpx-res.jp2").unwrap();
    let settings = hayro_jpeg2000::DecodeSettings::default();
    let jpx = hayro_jpeg2000::Image::new(&data, &settings).unwrap();
    println!("{}x{} comps {} alpha {}", jpx.width(), jpx.height(),
             jpx.color_space().num_channels(), jpx.has_alpha());
    let mut ctx = hayro_jpeg2000::DecoderContext::default();
    let decoded = jpx.decode(&mut ctx).unwrap();
    let samples = decoded.data_u8();
    let w = jpx.width() as usize;
    let h = jpx.height() as usize;
    // Print a few sample pixels on a 32px grid
    for y in [10, 74, 138] {
        for x in [10, 74, 138] {
            let i = (y * w + x) * 3;
            print!("({x},{y}): [{}, {}, {}]  ", samples[i], samples[i+1], samples[i+2]);
        }
        println!();
    }
    // Also save a PPM downsampled to 300x300 by box averaging 5-6 px
    let mut ppm = format!("P6\n300 300\n255\n").into_bytes();
    for y in 0..300 {
        for x in 0..300 {
            // 1600/300 ≈ 5.333: area average
            let (mut r, mut g, mut b, mut n) = (0u64, 0u64, 0u64, 0u64);
            let (x0, x1) = (x * w / 300, ((x+1) * w / 300).max(x * w / 300 + 1));
            let (y0, y1) = (y * h / 300, ((y+1) * h / 300).max(y * h / 300 + 1));
            for yy in y0..y1.min(h) {
                for xx in x0..x1.min(w) {
                    let i = (yy * w + xx) * 3;
                    r += samples[i] as u64; g += samples[i+1] as u64; b += samples[i+2] as u64;
                    n += 1;
                }
            }
            ppm.push((r/n) as u8); ppm.push((g/n) as u8); ppm.push((b/n) as u8);
        }
    }
    std::fs::write("/tmp/jpx-hayro.ppm", ppm).unwrap();
    let mut full = format!("P6
{} {}
255
", w, h).into_bytes();
    full.extend_from_slice(&samples);
    std::fs::write("/tmp/jpx-hayro-full.ppm", full).unwrap();
    println!("wrote /tmp/jpx-hayro-full.ppm (full res)");
}
