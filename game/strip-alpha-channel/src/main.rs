fn main() {
    let mut img = image::open("D0.png").unwrap().into_rgba();

    for pixel in img.pixels_mut() {
        pixel.0[3] = u8::MAX;
    }

    img.save("img.png").unwrap();
}
