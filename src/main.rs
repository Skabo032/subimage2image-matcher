use image::{GenericImageView, DynamicImage, Rgba, GenericImage};
use std::error::Error;
use std::{fs};

struct SubimageWithLocation {
    location: (u32, u32),
    image: DynamicImage,
}

fn image_distance(og_img: &DynamicImage, x: u32, y: u32, target_img: &DynamicImage) -> f32 {
    //let (og_width, og_height) = og_img.dimensions();
    let (target_width, target_height) = target_img.dimensions();

    let mut sum_dist_square = 0.0;

    for ty in 0..target_height{
        for tx in 0..target_width{
            for color_channel in 0..3 {
                let target_pixel = target_img.get_pixel(tx, ty).0[color_channel] as f32;
                let og_pixel = og_img.get_pixel(x + tx, y + ty).0[color_channel] as f32;
                let dist = target_pixel - og_pixel;
                sum_dist_square += dist * dist;
            }
        }
    }
    return (sum_dist_square / (target_width * target_height) as f32).sqrt();
    // for ty in 0..target_height{
    //     for color_channel in 0..3 {
    //         let target_pixel = target_img.get_pixel(0, ty).0[color_channel] as f32;
    //         let og_pixel = og_img.get_pixel(x + 0, y + ty).0[color_channel] as f32;
    //         let dist = target_pixel - og_pixel;
    //         sum_dist_square += dist * dist;
    //         let target_pixel = target_img.get_pixel(target_width-1, ty).0[color_channel] as f32;
    //         let og_pixel = og_img.get_pixel(x + target_width-1, y + ty).0[color_channel] as f32;
    //         let dist = target_pixel - og_pixel;
    //         sum_dist_square += dist * dist;
    //         // let target_pixel = target_img.get_pixel((target_width-1)/2, ty).0[color_channel] as f32;
    //         // let og_pixel = og_img.get_pixel(x + (target_width-1)/2, y + ty).0[color_channel] as f32;
    //         // let dist = target_pixel - og_pixel;
    //         // sum_dist_square += dist * dist;
    //     }
    // }
    // for tx in 0..target_width{
    //     for color_channel in 0..3 {
    //         let target_pixel = target_img.get_pixel(tx, 0).0[color_channel] as f32;
    //         let og_pixel = og_img.get_pixel(x + tx, y + 0).0[color_channel] as f32;
    //         let dist = target_pixel - og_pixel;
    //         sum_dist_square += dist * dist;
    //         let target_pixel = target_img.get_pixel(tx, target_height-1).0[color_channel] as f32;
    //         let og_pixel = og_img.get_pixel(tx, y + target_height-1).0[color_channel] as f32;
    //         let dist = target_pixel - og_pixel;
    //         sum_dist_square += dist * dist;
    //         // let target_pixel = target_img.get_pixel(tx, (target_height-1)/2).0[color_channel] as f32;
    //         // let og_pixel = og_img.get_pixel(tx, y + (target_height-1)/2).0[color_channel] as f32;
    //         // let dist = target_pixel - og_pixel;
    //         // sum_dist_square += dist * dist;
    //     }
    // }
    // return (sum_dist_square / (2 * target_width + 2 * target_height) as f32).sqrt();
}

fn find_match(og_img: &DynamicImage, sub_img: &DynamicImage, treshold: f32) -> Option<(u32,u32)> {
    let (og_width, og_height) = og_img.dimensions();
    let (sub_width, sub_height) = sub_img.dimensions();

    for x in 0..og_width-sub_width{
        for y in 0..og_height-sub_height{
            let distance = image_distance(&og_img, x, y, &sub_img);
            println!("distance is: {}", distance);
            if distance <= treshold {
                return Some((x, y));
            }
        }
    }
    return None;
}

fn read_subimages_from_directory(directory_path: &str) -> Result<Vec<DynamicImage>, Box<dyn Error>> {
    let mut images = Vec::new();

    // Read entries in the directory
    if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                // Check if the entry is a file
                if path.is_file() {
                    // Attempt to load the image
                    if let Ok(image) = image::open(&path) {
                        images.push(image);
                    } else {
                        eprintln!("Failed to open image at path: {:?}", path);
                    }
                }
            }
        }
    } else {
        eprintln!("Error reading directory entries");
    }

    Ok(images)
}

fn main() {
    println!("=== --- subimage2image-matcher --- ===");
    let og_image = image::open("examples/picture1.jpg").expect("error opening original image");
    let subimages: Option<Vec<DynamicImage>> = match read_subimages_from_directory("examples/slika 1"){
        Ok(images) => Some(images),
        Err(e) => {
            eprintln!("Error reading subimages from directory: {}", e);
            None
        }
    };

    let mut subimages_with_location: Vec<SubimageWithLocation> = vec![];
    for image in subimages.unwrap() {
        let subimage_loc = find_match(&og_image, &image, 70.0);
        match subimage_loc {
            Some((x,y)) => subimages_with_location.push(SubimageWithLocation{
                location: (x,y),
                image: image.clone(),
            }),
            None => {
                eprintln!("Error could not find location of subimage in image");
            }
        }
    }

    let mut result_image = DynamicImage::new_rgba8(800, 800);
    // Iterate over the subimages and place them in the result image
    for subimage in &subimages_with_location {
        let (subimage_x, subimage_y) = subimage.location;

        for x in 0..subimage.image.width() {
            for y in 0..subimage.image.height() {
                let pixel = subimage.image.get_pixel(x, y).0;
                result_image.put_pixel(
                    (subimage_x + x) as u32,
                    (subimage_y + y) as u32,
                    Rgba(pixel),
                );
            }
        }
    }
    
    result_image.save("result.jpg").unwrap();
}


