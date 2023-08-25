use nanoimage::decode;

fn main() {
    let image = decode(include_bytes!("texture.png")).unwrap();
    println!("{} {} {}", image.width, image.height, image.data.len());
}
