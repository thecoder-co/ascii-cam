# cam_ascii

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE.md)

> 📸 Real-time ASCII art rendering from your webcam, right in your terminal.

This project captures video from your webcam and renders it as live ASCII art in your terminal.

## Demo

Here's a sample of what the output looks like:

```
FPS: 29.8
........................................................................
...................,,::;;;iiii*+*OOZZ00MM$$BB88XXWW%%##@@@@@@@@............
............,::;;ii*+*OOZZ00MM$$BB88XXWW%%##@@..@@##%%WWXX88BB$$MM00........
........,;ii*+*OOZZ00MM$$BB88XXWW%%##@@..,,..@@##%%WWXX88BB$$MM00ZZOO*+......
....,;i*+*OOZZ00MM$$BB88XXWW%%##@@'    .'@@##%%WWXX88BB$$MM00ZZOO*+*i;:,....
..:;i*+*OOZZ00MM$$BB88XXWW%%##@@..      ..@@##%%WWXX88BB$$MM00ZZOO*+*i;:,...
,;i*+*OOZZ00MM$$BB88XXWW%%##@@..          ..@@##%%WWXX88BB$$MM00ZZOO*+*i;:,..
i*+*OOZZ00MM$$BB88XXWW%%##@@..              ..@@##%%WWXX88BB$$MM00ZZOO*+*i;,.
+*OOZZ00MM$$BB88XXWW%%##@@,....................,@@##%%WWXX88BB$$MM00ZZOO*+*i
OOZZ00MM$$BB88XXWW%%##@@'........................'@@##%%WWXX88BB$$MM00ZZOO*+
ZZ00MM$$BB88XXWW%%##@@'............................'@@##%%WWXX88BB$$MM00ZZOO
00MM$$BB88XXWW%%##@@'................................'@@##%%WWXX88BB$$MM00ZZ
MM$$BB88XXWW%%##@@'....................................'@@##%%WWXX88BB$$MM00
$$BB88XXWW%%##@@'........................................'@@##%%WWXX88BB$$MM
BB88XXWW%%##@@'............................................'@@##%%WWXX88BB$$
```

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

**Note for Linux users:** You may need to install development packages for `video4linux`. For Debian-based distributions (like Ubuntu), you can install it with:
```sh
sudo apt-get install libv4l-dev
```

## Getting Started

To get a local copy up and running, follow these simple steps.

1.  **Clone the repository**
    ```sh
    git clone https://github.com/your_username/cam_ascii.git
    cd cam_ascii
    ```
    *(Replace `your_username` with your GitHub username and `cam_ascii` with your repository name)*

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
*   crossterm for terminal manipulation.
