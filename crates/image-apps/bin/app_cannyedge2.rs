use image::{GenericImageView, GrayImage};
use imageproc::{contours, edges::canny, rect::Rect};

// detect target image

fn main() {
    // 1. load image
    println!("1. load image");
    let img = image::open("./images/web1.png").expect("Failed to open input image");

    // 2. transform to grayscale
    println!("2. transform to grayscale");
    let gray_img = img.to_luma8();

    // 3. detect Canny Edge / Cannyエッジ検出
    println!("3. detect Canny Edge");
    let edges = canny(&gray_img, 50.0, 100.0);

    // 4. find contours (輪郭)
    println!("4. find contours");
    let contours = contours::find_contours(&edges);
    println!("contours length: {}", contours.len()); // e.g. 532 too much

    // 5. detect rectangles
    println!("5. detect rectangles");
    for (i, contour) in contours.iter().enumerate() {
        //println!("contours points: {:?}", contour.points);
        if contour.points.len() < 300 {
            continue; // too small
        }

        let bounding_rect = calculate_bounding_rect(contour);
        let aspect_ratio = bounding_rect.width() as f32 / bounding_rect.height() as f32;
        //println!("aspect_ratio: {:?}", aspect_ratio);
        // 0.58 must be the best aspect_ratio
        if aspect_ratio < 0.8 && aspect_ratio > 0.4 {
            let rect = Rect::at(bounding_rect.left() as i32, bounding_rect.top() as i32)
                .of_size(bounding_rect.width(), bounding_rect.height());

            // Ensure the rectangle is within image bounds
            if rect.left() < 0
                || rect.top() < 0
                || rect.right() > gray_img.width() as i32
                || rect.bottom() > gray_img.height() as i32
            {
                continue;
            }

            // Use the sub_image method to crop the region
            let sub_img = gray_img
                .view(
                    rect.left() as u32,
                    rect.top() as u32,
                    rect.width(),
                    rect.height(),
                )
                .to_image();

            // Convert the cropped image to a GrayImage
            let sub_img_gray = GrayImage::from_raw(rect.width(), rect.height(), sub_img.into_raw())
                .expect("Failed to create GrayImage");

            // Save or process the detected rectangle region
            sub_img_gray
                .save(format!("./output2/rect_{}_{}.png", i, rect.left()))
                .expect("Failed to save image");
        }
    }
}

// calculate_bounding_rect calculates the bounding rect of a contour
fn calculate_bounding_rect(contour: &contours::Contour<i32>) -> Rect {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for point in &contour.points {
        if point.x < min_x {
            min_x = point.x;
        }
        if point.y < min_y {
            min_y = point.y;
        }
        if point.x > max_x {
            max_x = point.x;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }

    Rect::at(min_x, min_y).of_size((max_x - min_x) as u32, (max_y - min_y) as u32)
}
