// workspace
use opencv_lib::cv::{effect, loader, saver}; // Note: change name from `opencv-lib`` to `opencv_lib``

// save grascale image
fn main() -> anyhow::Result<()> {
    // 画像を読み込む
    println!("1. load images");
    let image_path = "./images/entireimage.png";
    let image = loader::load_image(image_path)?;

    // 画像の二値化を行う
    println!("2. transform images to binary");
    let gray_image = effect::apply_threshold(&image, 128.0, 255.0)?;

    // 保存
    let save_path = "./output/entireimage_binary.png";
    println!("3. save image: file name {}", save_path);
    saver::save_image(save_path, &gray_image)?;

    Ok(())
}
