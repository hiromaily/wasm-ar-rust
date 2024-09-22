use image_hasher::HasherConfig;

// comparing entire image with target image is not good result

fn main() {
    let bg1_img = image::open("images/web1.png").unwrap();
    let bg2_img = image::open("images/web2.png").unwrap();
    let bg3_img = image::open("images/web_nodata.png").unwrap();
    let poi_img = image::open("images/poi-s.png").unwrap();

    // calculate hash by pHash (Perceptual Hash)
    let hasher = HasherConfig::new().to_hasher();

    let poi_hash = hasher.hash_image(&poi_img);
    let bg1_hash = hasher.hash_image(&bg1_img);
    let bg2_hash = hasher.hash_image(&bg2_img);
    let bg3_hash = hasher.hash_image(&bg3_img);
    // println!("bg image1 hash: {}", bg1_hash.to_base64());

    // ハッシュの類似度を比較
    // ハミング距離の値が小さいほど、画像間の類似度が高いことを意味
    let bg1_similarity = bg1_hash.dist(&poi_hash);
    let bg2_similarity = bg2_hash.dist(&poi_hash);
    let bg3_similarity = bg3_hash.dist(&poi_hash);
    println!(
        "similarity: bg1: {:?}, bg2: {:?}, bg3: {:?}",
        bg1_similarity, bg2_similarity, bg3_similarity,
    );

    // let threshold = 10;
    // if similarity < threshold {
    //     println!("image found");
    // } else {
    //     println!("not found");
    // }
}
