use image::{GenericImageView, ImageBuffer, Luma};

pub fn pixel_b_pixel_gray(path: &str, save_path: &str) {
    println!("pixel_b_pixel_gray, from: {} | to: {}", path, save_path);

    let img = image::open(path)
        .unwrap();
    let mut output = ImageBuffer::new(img.width(), img.height());
    for (x, y, p) in img.pixels() {
        let r = p[0] as f32;
        let g = p[1] as f32;
        let b = p[2] as f32;

        let gray = (0.29 * r + 0.59 * g + 0.11 * b) as u8;

        output.put_pixel(x, y, image::Luma([gray]));
    }
    output.save(format!("{}pixel_b_pixel_gray.png",save_path))
        .expect("Saving failed");
}

pub fn gray_max_color_channel(input_path: &str, save_path: &str, max_colors: u8) {
    println!("gray_max_color_channel, channels: {} | from: {} | to: {}", max_colors,input_path, save_path);
    
    let img = image::open(input_path).unwrap();
    let mut output = ImageBuffer::new(img.width(), img.height());

    // Calculate the step size for quantization
    let step_size = (256.0 / max_colors as f32).ceil() as u8;

    for (x, y, p) in img.pixels() {
        let r = p[0] as f32;
        let g = p[1] as f32;
        let b = p[2] as f32;

        let gray = (0.29 * r + 0.59 * g + 0.11 * b).clamp(0.0, 255.0) as u8;

        let quantized_gray = (gray as f32 / step_size as f32).floor() as u8 * step_size;

        output.put_pixel(x, y, Luma([quantized_gray]));
        // output.put_pixel(x, y, Rgb([quantized_r, quantized_g, quantized_b]));
    }

    output.save(format!("{}{}_gray_channels.png",save_path,max_colors)).unwrap();
}
