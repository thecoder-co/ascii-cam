# cam_ascii

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE.md)

> 📸 Real-time ASCII art rendering from your webcam, right in your terminal.

This project captures video from your webcam and renders it as live ASCII art in your terminal.

## Demo

Here's a sample of what the output looks like:

<img width="1680" height="1050" alt="Screenshot 2025-08-30 at 09 18 55" src="https://github.com/user-attachments/assets/73b073ff-6d81-4709-9037-50d52fdd5c81" />

## About The Project

`cam_ascii` is a simple command-line tool written in Rust. It uses the `nokhwa` crate to access the computer's camera and the `image` crate for processing. Each frame from the camera is converted to grayscale, resized, and then its pixel brightness is mapped to a set of ASCII characters. The result is a live, animated ASCII video stream printed directly to your console.

## Features

-   **Real-time Conversion**: Captures video from your default webcam and converts it to ASCII on the fly.
-   **Terminal-Based**: Runs entirely within your terminal, no GUI needed.
-   **Performance Metrics**: Displays the current Frames Per Second (FPS).
-   **Simple & Hackable**: A straightforward implementation in a single file, easy to modify and experiment with.

## Prerequisites

Before you begin, ensure you have the following installed:
*   Rust and Cargo
*   A connected webcam

## Getting Started

To get a local copy up and running, follow these simple steps.

1.  **Clone the repository**
    ```sh
    git clone https://github.com/thecoder-co/ascii-cam.git
    cd cam_ascii
    ```

2.  **Run the application**
    For the best performance, run the application in release mode:
    ```sh
    cargo run --release
    ```

3.  **Exit the application**
    Press `Ctrl+C` in the terminal to stop the program.

## Configuration

You can easily tweak the output by modifying a few values in `src/main.rs`:

```rust
// src/main.rs

fn main() -> Result<()> {
    // ...

    // You can change the characters used for the ASCII ramp.
    // Goes from darkest to brightest.
    let ramp = "@#%WX8B&$M0QOZ*+i!;:,'. ";
    let ramp_chars: Vec<char> = ramp.chars().collect();

    // ...
    loop {
        // ...
        // You can change the output width and whether the colors are inverted.
        let ascii_frame = convert_image_to_ascii(&img, 1000, true, &ramp_chars)?;
        // ...
    }
}
```

-   `ramp`: The string of characters used for rendering, from darkest to brightest.
-   `target_width`: The width (in characters) of the ASCII output.
-   `invert`: A boolean (`true` or `false`) to invert the brightness mapping.

## License

Distributed under the MIT License. See `LICENSE.md` for more information.

Copyright (c) 2025 ABIMBOLA, Idunnuoluwa

## Acknowledgements

This project is made possible by the fantastic work of the Rust community and these key libraries:
*   nokhwa for camera controls.
*   image-rs for image processing.
