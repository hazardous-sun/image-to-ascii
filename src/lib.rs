use std::error::Error;
use image;

pub fn run(args: &[String]) -> Result<(), &'static str> {

    let config: Config;

    match Config::build(args) {
        Ok(result) => { config = result; },
        Err(e) => { return Err(e); },
    }

    let symbols: Symbols = Symbols::new();

    let image = load_image(&config.image_path);
    match image {
        Ok(image) => {
            Ok(())
        },
        Err(e) => {
            Err(e)
        },
    }
}

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

        let image_path = args[1].clone();

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

struct Symbols {
    characters: Vec<&'static str>,
}

impl Symbols {
    fn new() -> Symbols {
        let characters: Vec<&'static str> = vec![
            "□",
            "▧",
            "▥",
            "▩",
            "▦",
            "▣",
            "■",
        ];

        Symbols {
            characters
        }
    }
}

fn load_image(image_path: &String) -> Result<image::DynamicImage, &'static str> {
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
        let image = load_image(&"test_images/duck.png".to_string());
        assert!(image.is_ok());
    }

    #[test]
    fn err_load_image() {
        let image = load_image(&"duckduckgo".to_string());
        assert!(image.is_err());
    }

    #[test]
    fn ok_config_build() {
        let values = vec![".".to_string(), "path/to/image".to_string(), "0.5".to_string()];
        let config = Config::build(&values[..]);
        assert!(config.is_ok())
    }

    #[test]
    fn err_config_build() {
        let values = vec![".".to_string(), "path/to/image".to_string(), "1.5".to_string()];
        let config = Config::build(&values[..]);
        assert!(config.is_err());

        let values = vec![".".to_string(), "path/to/image".to_string()];
        let config = Config::build(&values[..]);
        assert!(config.is_err())
    }
}