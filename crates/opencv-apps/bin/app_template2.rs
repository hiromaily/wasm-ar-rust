// workspace
use opencv_lib::cv::{grayscale, loader, template}; // Note: change name from `opencv-lib`` to `opencv_lib``

fn main() -> anyhow::Result<()> {
    // テンプレートマッチングを実行するための画像とテンプレート画像を読み込む
    println!("1. load images");
    let image_path = "./images/entireimage-nodata.png";

    let image = loader::load_image(image_path)?;
    let data = include_bytes!("../images/template.png");
    let template = loader::load_embedded_template(data)?;

    // 画像のグレースケール化を行う
    println!("2. transform images to grayscale");
    let gray_image = grayscale::convert_to_grayscale(&image)?;
    let gray_template = grayscale::convert_to_grayscale(&template)?;

    // テンプレートマッチングを実行する
    println!("3. execute template matching");
    let result = template::template_matching(&gray_image, &gray_template)?;

    // テンプレートマッチングの結果を取得する
    println!("4. get result");
    let (max_loc, max_val) = template::get_matching_result(&result)?;
    println!("Max Location: {:?}, Max Value: {:?}", max_loc, max_val);
    //Max Location: Point_ { x: 771, y: 922 }, Max Value: 0.9155991673469543

    Ok(())
}
