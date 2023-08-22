use image::{GrayImage, Rgba, RgbaImage};
use rand::prelude::*;
use std::collections::{HashMap, VecDeque};

fn label_connected_components(image: &GrayImage) -> RgbaImage {
    let mut labels = vec![vec![0u8; image.width() as usize]; image.height() as usize];
    let mut next_label = 1u8;

    for y in 0..image.height() {
        for x in 0..image.width() {
            if image.get_pixel(x, y)[0] < 128 && labels[y as usize][x as usize] == 0 {
                label_connected_component(image, &mut labels, x as usize, y as usize, next_label);
                next_label += 1;
            }
        }
    }

    let label_colors = generate_random_colors(next_label);

    let mut labeled_image = RgbaImage::new(image.width(), image.height());
    for y in 0..image.height() {
        for x in 0..image.width() {
            let label = labels[y as usize][x as usize];
            let color = label_colors.get(&label).unwrap_or(&Rgba([0, 0, 0, 0])).clone();
            labeled_image.put_pixel(x, y, color);
        }
    }

    labeled_image
}

fn generate_random_colors(num_colors: u8) -> HashMap<u8, Rgba<u8>> {
    let mut rng = thread_rng();
    let mut colors = HashMap::new();

    for label in 1..=num_colors {
        let color = Rgba([
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            255, // Fully opaque for labeled components
        ]);
        colors.insert(label, color);
    }

    colors
}

fn label_connected_component(image: &GrayImage, labels: &mut Vec<Vec<u8>>, start_x: usize, start_y: usize, label: u8) {
    let mut queue = VecDeque::new();
    queue.push_back((start_x, start_y));

    while let Some((x, y)) = queue.pop_front() {
        if x < image.width() as usize && y < image.height() as usize && labels[y][x] == 0 && image.get_pixel(x as u32, y as u32)[0] < 128 {
            labels[y][x] = label;

            queue.push_back((x + 1, y));
            queue.push_back((x, y + 1));
            if x > 0 {
                queue.push_back((x - 1, y));
            }
            if y > 0 {
                queue.push_back((x, y - 1));
            }
        }
    }
}
