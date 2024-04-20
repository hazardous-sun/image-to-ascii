use image;

#[derive(Debug)]
pub struct Config {
    image_path: String,
    reduction_factor: f32,
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

        if reduction_factor > 1.0 {
            return Err("ERROR: Reduction factor must be equal to or smaller than 1");
        }

        Ok(Config {
            image_path,
            reduction_factor
        })
    }
}

struct AsciiSymbols {
    characters: Vec<&'static str>,
}

impl AsciiSymbols {
    fn new() -> AsciiSymbols {
        let characters: Vec<&'static str> = vec![
            "□",
            "▥",
            "▧",
            "▩",
            "▦",
            "▣",
            "■",
        ];

        AsciiSymbols {
            characters
        }
    }
}

pub fn run(image_path: &str) -> String {
    let image = load_image(image_path)?;
    match image {
        Ok(image) => {
            String::from("this should return the image as a string")
        }
    }
}

fn load_image(image_path: &str) -> Result<image::DynamicImage, image::ImageError> {
    let image = image::open(image_path)?;
    Ok(image)
}

mod tests {
    use super::*;

    #[test]
    fn load_image() {
        let image = load_image("");
    }
}