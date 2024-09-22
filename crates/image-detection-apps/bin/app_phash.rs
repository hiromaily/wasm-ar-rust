use image_hasher::HasherConfig;

// comparing entire image with target image is not good result

fn main() {
    let bg1_img = image::open("images/web1.png").unwrap();
    let bg2_img = image::open("images/web2.png").unwrap();
    let bg3_img = image::open("images/web_nodata.png").unwrap();

    let bg4_img = image::open("images/rect_19_252.png").unwrap();
    let bg5_img = image::open("images/rect_20_252.png").unwrap();
    let bg6_img = image::open("images/rect_21_225.png").unwrap();

    let poi_img = image::open("images/poi-s.png").unwrap();

    // calculate hash by pHash (Perceptual Hash)
    let hasher = HasherConfig::new().to_hasher();

    let poi_hash = hasher.hash_image(&poi_img);
    let bg1_hash = hasher.hash_image(&bg1_img);
    let bg2_hash = hasher.hash_image(&bg2_img);
    let bg3_hash = hasher.hash_image(&bg3_img);
    let bg4_hash = hasher.hash_image(&bg4_img);
    let bg5_hash = hasher.hash_image(&bg5_img);
    let bg6_hash = hasher.hash_image(&bg6_img);
    // println!("bg image1 hash: {}", bg1_hash.to_base64());

    // ハッシュの類似度を比較
    // ハミング距離の値が小さいほど、画像間の類似度が高いことを意味
    let bg1_similarity = bg1_hash.dist(&poi_hash);
    let bg2_similarity = bg2_hash.dist(&poi_hash);
    let bg3_similarity = bg3_hash.dist(&poi_hash);
    let bg4_similarity = bg4_hash.dist(&poi_hash);
    let bg5_similarity = bg5_hash.dist(&poi_hash);
    let bg6_similarity = bg6_hash.dist(&poi_hash);
    println!(
        "similarity: bg1: {:?}, bg2: {:?}, bg3: {:?}, bg4: {:?}, bg5: {:?}, bg6: {:?}",
        bg1_similarity,
        bg2_similarity,
        bg3_similarity,
        bg4_similarity,
        bg5_similarity,
        bg6_similarity
    );
    //similarity: bg1: 30, bg2: 31, bg3: 38, bg4: 27, bg5: 27, bg6: 35

    // let threshold = 10;
    // if similarity < threshold {
    //     println!("image found");
    // } else {
    //     println!("not found");
    // }
}
