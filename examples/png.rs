use nanoimage::png;

fn main() {
    let image = png::decode(include_bytes!("texture.png")).unwrap();
    println!("{} {} {}", image.width, image.height, image.data.len());
}
