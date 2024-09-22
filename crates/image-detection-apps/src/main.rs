// workspace
use template_matching::{find_extremes, match_template, MatchTemplateMethod};

fn main() {
    // 1. load images
    println!("1. load image");
    //let bg_img = image::open("images/web1.png").unwrap();
    //let bg_img = image::open("images/web2.png").unwrap();
    let bg_img = image::open("images/web_nodata.png").unwrap();
    let template_img = image::open("images/poi-s.png").unwrap();

    // 2. transform to grayscale
    println!("2. transform to grayscale");
    let gray_bg_img = bg_img.to_luma32f();
    let gray_template_img = template_img.to_luma32f();

    // 3. template matching
    println!("3. template matching");
    let result = match_template(
        &gray_bg_img,
        &gray_template_img,
        MatchTemplateMethod::SumOfSquaredDifferences,
    );

    // Calculate min & max values
    println!("4. calculate min & max values");
    let extremes = find_extremes(&result);
    //result: Extremes { min_value: 8986.987, max_value: 21019.777, min_value_location: (233, 88), max_value_location: (163, 182) }
    //result: Extremes { min_value: 9166.368, max_value: 18909.066, min_value_location: (449, 80), max_value_location: (172, 164) }
    //result: Extremes { min_value: 8020.6113, max_value: 16082.4795, min_value_location: (420, 132), max_value_location: (156, 191) }
    println!("result: {:?}", extremes);
}
