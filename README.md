# cleye ![eog representation](https://travis-ci.org/evanandrewrose/cleye.svg?branch=master "Image captured via eog.")

Terminal-based image viewer.

    cleye --help         
    cleye 0.1.0
    evanandrewrose <evanandrewrose@gmail.com>
    A simple terminal-based image viewer.
    
    USAGE:
        cleye [FLAGS] [OPTIONS] <file_path>
    
    FLAGS:
        -h, --help         Prints help information
        -n, --no_render    Dry run, do not render.
        -V, --version      Prints version information
        -v, --verbose      Enable logging, use multiple `v`s to increase verbosity.
    
    OPTIONS:
        -f, --filter <filter>              Filter to apply when resizing image. (Nearest, Triangle, CatmullRom, Gaussian, Lanczos3) [default: Nearest]
        -p, --pixel_width <pixel_width>    Number of characters for a given pixel. [default: 2]

    ARGS:
        <file_path>    Input file to present.

# original
![eog representation](/img/eog_screen.png?raw=true "Image captured via eog.")

# cleye
![cleye_representation](/img/tool_screen.png?raw=true "Image captured via cleye.")

# system requirements

Acquire rust and cargo from [rustup.rs](rustup.rs), then just run:

# build

    cargo build

# run

    cargo run

# thanks

This application was extremely easy to develop thanks to:

- [termion](https://github.com/ticki/termion)
- [image](https://github.com/PistonDevelopers/image)
- [structopt](https://github.com/TeXitoi/structopt)
- [error-chain](https://github.com/rust-lang-nursery/error-chain)
- [log](https://github.com/rust-lang-nursery/log)
- [loggerv](https://github.com/clux/loggerv)
