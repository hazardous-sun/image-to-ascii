# Image to Text

This Rust project converts images into UTF-8 art representations.

## Features

- Converts images to grayscale before processing.
- Uses a configurable reduction factor to control the size of the output ASCII art.
- Allows for reversed intensity mapping (lighter pixels become darker characters).

## Usage

This library is used as a command-line tool.  After building the project using cargo build, you can run it with the following command:

```Bash
image-to-ascii [IMAGE_PATH] [REDUCTION_FACTOR] [REVERSE (optional)]
```

- `IMAGE_PATH`: The path to the image file you want to convert.
- `REDUCTION_FACTOR`: A floating-point number between 0.0 and 1.0 that controls the size of the output ASCII art. A smaller value results in a smaller output.
- `REVERSE` (optional): A flag that can be used to reverse the intensity mapping. With this flag, lighter pixels in the image will be represented by darker characters in the ASCII art.

### Exampĺe

```Bash
image-to-ascii path/to/image.png 0.5
```

This command will convert the image at path/to/image.png to ASCII art with a reduction factor of 0.5 (resulting in a half-sized output compared to the original image).

## Dependencies

This library depends on the following Rust crate:

- `image`: https://docs.rs/image

You can install this dependency by adding the following line to your `Cargo.toml` file:

```TOML
[dependencies]
image = "0.25.1"
```
