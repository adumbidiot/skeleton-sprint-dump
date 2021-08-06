fn main() {
    image::open("ultra_hd_note_texture_x61.png")
        .unwrap()
        .resize(150_000, 150_000, image::imageops::FilterType::Lanczos3)
        .save("output.png")
        .unwrap();
}
