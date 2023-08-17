// use std::io::Cursor; // not going to use now
use image::{GenericImageView, ImageBuffer, Luma, Rgb};

fn main() {
    let path = "src/image.tiff";
    // let SavePath = "src/Generated/";

    image::open(path)
    .unwrap()
    .save( "src/base.png")
    .unwrap();


    simple_gray(path);
    pixel_b_pixel_gray(path);
    gray_max_color_channel(path, 2);
    max_color_channel(path, 4);
}

fn simple_gray(path: &str) {
    let img = image::open(path).unwrap();
 img.into_luma8().save("src/simple_gray.png").unwrap();
}
fn pixel_b_pixel_gray(path: &str) {
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
    output.save("src/pixel_b_pixel_gray.png")
        .expect("Saving failed");
}

// convert image to gray in a given a scale of colorsuse image::{GenericImageView, ImageBuffer, Luma};

fn gray_max_color_channel(input_path: &str, max_colors: u8) {
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

    output.save("src/gray_max_color_channel.png").unwrap();
}

fn max_color_channel(input_path: &str, max_colors: u8) {
    let img = image::open(input_path).unwrap();
    let mut output = ImageBuffer::new(img.width(), img.height());

    // Calculate the step size for quantization
    let step_size = (256.0 / max_colors as f32).ceil() as u8;

    for (x, y, p) in img.pixels() {
        let r = p[0] as f32;
        let g = p[1] as f32;
        let b = p[2] as f32;
       
        let quantized_r = (r / step_size as f32).floor() as u8 * step_size;
        let quantized_g = (g / step_size as f32).floor() as u8 * step_size;
        let quantized_b = (b / step_size as f32).floor() as u8 * step_size;

        output.put_pixel(x, y, Rgb([quantized_r, quantized_g, quantized_b]));
   
    }

    output.save("src/max_color_channel.png").unwrap();
}
