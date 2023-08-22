
fn simple_gray(path: &str, save_path: &str) {
    let img = image::open(path).unwrap();
 img.into_luma8().save(format!("{}simple_gray.png",save_path)).unwrap();
}
fn pixel_b_pixel_gray(path: &str, save_path: &str) {
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
