use std::error::Error;
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
            "▧",
            "▥",
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

pub fn run(image_path: &str) -> Result<(), &'static str> {
    let image = load_image(image_path);
    match image {
        Ok(image) => {
            Ok(())
        },
        Err(_) => {
            Err("ERROR: Error in RUN method")
        },
    }
}

fn load_image(image_path: &str) -> Result<image::DynamicImage, &'static str> {
    let image = image::open(image_path);
    match image {
        Ok(image) => Ok(image),
        Err(_) => Err("ERROR: Invalid image path.")
    }
}

mod tests {
    use super::*;

    #[test]
    fn ok_load_image() {
        let image = load_image("test_images/duck.png");
        assert!(image.is_ok());
    }

    #[test]
    fn err_load_image() {
        let image = load_image("duckduckgo");
        assert!(image.is_err());
    }
}