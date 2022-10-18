use std::{
    io::Read, 
    error::Error,
    num::NonZeroU32
};
use image::DynamicImage;
use artem::{options::OptionBuilder};


fn load_image(url: &str) -> Result<DynamicImage, Box<dyn Error>> {
    let resp = ureq::get(url).call()?;

    let len: usize = resp.header("content-length")
        .unwrap()
        .parse()?;

    let mut bytes: Vec<u8> = Vec::with_capacity(len);
    resp.into_reader()
        .take(10_000_000)
        .read_to_end(&mut bytes)?;

    Ok(image::load_from_memory(&bytes)?)
}

pub fn convert(icon: &str) -> Result<String, Box<dyn Error>> {
    let mut options_builder = OptionBuilder::new();
    options_builder.characters(r#"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'. "#.to_string());
    options_builder.target_size(NonZeroU32::new(30).unwrap());

    let image = load_image(format!("https:{}", icon).as_str())?;
    Ok(artem::convert(image, options_builder.build()))
}
