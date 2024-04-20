use image;

#[derive(Debug)]
pub struct Config {
    image_path: String,
    reduction_factor: f32
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("ERROR: Not enough parameters passed!\nUsage: image-to-ascii [IMAGE_PATH] [REDUCTION_FACTOR]");
        }

        let image_path = args[1].to_string();

        let reduction_factor: f32;

        match args[2].parse() {
            Ok(number) => {
                reduction_factor = number;
            },
            Err(_) => {
                return Err("ERROR: Invalid float number passed!\n");
            }
        }

        if reduction_factor > 1.0 {}

        Ok(Config {
            image_path,
            reduction_factor
        })
    }
}

pub fn run(image_path: &str) -> String {
    let image = load_image(image_path)?;
    match image {
        Ok(image) => {
            image_to_ascii(image)
        }
    }
}

fn load_image(image_path: &str) -> Result<image::DynamicImage, image::ImageError> {
    let image = image::open(image_path)?;
    Ok(image)
}

fn image_to_ascii(image: image::DynamicImage) -> String {
    let bw_image = make_image_bw(image);
}

fn make_image_bw(image: image::DynamicImage) -> image::Image {
    image::
}