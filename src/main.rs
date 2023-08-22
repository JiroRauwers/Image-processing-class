pub mod gray_images;

use image::{Rgba, ImageBuffer};

use std::{fs, collections::HashSet, vec};

#[allow(unused_imports)]
use gray_images::gray_max_color_channel;

// use std::io::Cursor; // not going to use now

#[allow(unused)]
const RED: Rgba<u8> = Rgba([255 as u8,0 as u8,0 as u8,255 as u8]);
#[allow(unused)]
const TRANSPARENT: Rgba<u8> = Rgba([0 as u8,0 as u8,0 as u8,0 as u8]);
#[allow(unused)]
const WHITE: Rgba<u8> = Rgba([255 as u8,255 as u8,255 as u8,255 as u8]);

fn main() {
    // let base_imgae_path = "src/image.tiff";
    let save_path = "src/Generated/";

    
    fs::create_dir_all(save_path).unwrap();

    problem_lab();
}



fn problem_lab() {
    let img = image::open("src/testing.png").unwrap().to_luma8();

    let mut output = ImageBuffer::new(img.width(), img.height());

    let mut black_pixels = 0;

    let mut group_pixels:Vec<HashSet<(u32, u32)>> = Vec::new();

    for (x, y, p) in img.enumerate_pixels() {
        let scale = p[0];

        if scale > 200 {
            output.put_pixel(x, y, WHITE);
            continue;
        };
        
        black_pixels += 1;

        // get adjacent pixels
        let top = (x, (y.wrapping_sub(1)) % img.height());
        let bottom = (x, (y.wrapping_add(1)) % img.height());
        let left = ((x.wrapping_sub(1)) % img.width(), y);
        let right = ((x.wrapping_add(1)) % img.width(), y);
        let self_pixel = (x, y);

        let mut new_group: HashSet<(u32, u32)> = HashSet::new();

        for (x, y) in &vec![
            top, bottom, left, right, self_pixel
        ] {
            if img.get_pixel(*x, *y)[0] < 200 {
                new_group.insert((*x,*y));
            }
        }
        // Find the index of the first intersecting group
        let intersecting_index = group_pixels.iter().position(|existing_group| {
            existing_group.intersection(&new_group).count() > 0
        });

        if let Some(index) = intersecting_index {
            group_pixels[index].extend(new_group);
            continue;
        } 
        group_pixels.push(new_group);

    }

    output.save(format!("{}{}testing.png","src/Generated/",2)).unwrap();
    println!("Black pixels: {}", black_pixels);

    for group in &group_pixels {
        println!("Group:");
        for &(x, y) in group {
            println!("({}, {})", x, y);
        }
    }

}


// let top = get_pixel_scale(img.get_pixel(x, (y.wrapping_sub(1)) % img.height()));
// let bottom = get_pixel_scale(img.get_pixel(x, (y.wrapping_add(1)) % img.height()));
// let left = get_pixel_scale(img.get_pixel((x.wrapping_sub(1)) % img.width(), y));
// let right = get_pixel_scale(img.get_pixel((x.wrapping_add(1)) % img.width(), y));

// let adjacent_pixels = vec![top, bottom, left, right].iter()
//     .filter(|&pixel| pixel < &200 ).cloned().collect::<Vec<_>>();