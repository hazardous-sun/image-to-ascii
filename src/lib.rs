use image::{DynamicImage, GrayImage};
use image::imageops::{FilterType};

pub fn run(args: &[String]) -> Result<(), &'static str> {
    let symbols: Symbols = Symbols::new();
    let config: Config;
    let mut image: DynamicImage;

    match Config::build(args) {
        Ok(result) => { config = result; }
        Err(e) => { return Err(e); }
    }

    match load_image(&config.image_path) {
        Ok(returned_image) => { image = resize_image(returned_image, &config); }
        Err(e) => { return Err(e); }
    }

    let grayscale_image = remove_colors(&image);
    let mut current_y = 0;

    for (_, y, pixel) in grayscale_image.enumerate_pixels() {
        let brightness = symbols.get_ascii_value(pixel, &config);
        if y > current_y {
            println!();
            current_y = y;
        }
        print!("{brightness}");
    }

    println!();

    return Ok(());
}

#[derive(Debug)]
struct Config {
    image_path: String,
    reduction_factor: f32,
    reversed: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let mut image_path: String;
        let mut reduction_factor: f32;
        let mut reversed= false;

        if args.len() < 3 {
            return Err("ERROR: Not enough parameters passed!\nUsage: image-to-ascii [IMAGE_PATH] [REDUCTION_FACTOR]");
        } else if args.len() < 4 {
            image_path = args[1].clone();
            reversed = true;
        } else {
            image_path = args[1].clone();
        }

        match args[2].parse() {
            Ok(number) => {
                reduction_factor = number;
            }
            Err(_) => {
                return Err("ERROR: Invalid float number passed!\n");
            }
        }

        if reduction_factor > 1.0 {
            return Err("ERROR: Reduction factor must be equal to or smaller than 1");
        }

        Ok(Config {
            image_path,
            reduction_factor,
            reversed,
        })
    }
}

struct Symbols {
    characters: Vec<&'static str>,
}

impl Symbols {
    fn new() -> Symbols {
        let characters: Vec<&'static str> = vec![
            " ",
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

    fn get_ascii_value(&self, pixel: &image::Luma<u8>, config: &Config) -> &str {
        return self.characters[self.get_intensity(pixel, config.reversed) as usize];
    }

    fn get_intensity(&self, pixel: &image::Luma<u8>, reversed: bool) -> i32 {
        let value = pixel[0] as u32;

        return if reversed {
            match value {
                0..=32 => 0,
                33..=64 => 1,
                65..=96 => 2,
                97..=128 => 3,
                129..=160 => 4,
                161..=192 => 5,
                193..=224 => 6,
                _ => 7,
            }
        } else {
            match value {
                225..=256 => 0,
                193..=224 => 1,
                161..=192 => 2,
                129..=160 => 3,
                97..=128 => 4,
                65..=96 => 5,
                33..=64 => 6,
                _ => 7
            }
        }
    }
}

fn load_image(image_path: &String) -> Result<DynamicImage, &'static str> {
    let image = image::open(image_path);
    match image {
        Ok(image) => Ok(image),
        Err(_) => Err("ERROR: Invalid image path.")
    }
}

fn resize_image(image: DynamicImage, config: &Config) -> DynamicImage {
    let (nwidth, nheight) = get_new_dimensions(&image, config);
    // return image.resize(nwidth, nheight, FilterType::Lanczos3);
    return image.resize_exact(nwidth * 2, nheight, FilterType::Lanczos3);
}

fn get_new_dimensions(image: &DynamicImage, config: &Config) -> (u32, u32) {
    (
        (image.width() as f32 * config.reduction_factor) as u32,
        (image.height() as f32 * config.reduction_factor) as u32
    )
}

fn remove_colors(image: &DynamicImage) -> GrayImage {
    image.to_luma8()
}

mod tests {
    use super::*;

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
    fn ok_resize_image() {
        let values = vec![".".to_string(), "path/to/image".to_string(), "0.5".to_string()];
        let config = Config::build(&values[..]);
        let image = load_image(&"test_images/duck.png".to_string());
        let resized_image = resize_image(image.unwrap(), &config.unwrap());

        assert_eq!(resized_image.width(), 200);
        assert_eq!(resized_image.height(), 200);
    }

    #[test]
    fn err_resize_image() {
        let values = vec![".".to_string(), "path/to/image".to_string(), "0.5".to_string()];
        let config = Config::build(&values[..]);
        let image = load_image(&"test_images/duck.png".to_string());
        let resized_image = resize_image(image.unwrap(), &config.unwrap());

        assert_ne!(resized_image.width(), 400);
        assert_ne!(resized_image.height(), 400);
    }

    #[test]
    fn ok_get_new_dimensions() {
        let values = vec![".".to_string(), "path/to/image".to_string(), "0.5".to_string()];
        let config = Config::build(&values[..]).unwrap();
        let image = load_image(&"test_images/duck.png".to_string()).unwrap();

        assert_eq!(get_new_dimensions(&image, &config), (200, 200));
    }

    #[test]
    fn err_get_new_dimensions() {
        let values = vec![".".to_string(), "path/to/image".to_string(), "0.5".to_string()];
        let config = Config::build(&values[..]).unwrap();
        let image = load_image(&"test_images/duck.png".to_string()).unwrap();

        assert_ne!(get_new_dimensions(&image, &config), (800, 800));
    }
}