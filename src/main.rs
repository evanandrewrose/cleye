extern crate image;
extern crate termion;

use std::path::Path;

use image::GenericImage;

use termion::raw::IntoRawMode;
use image::imageops;
use std::io::{stdout};

extern crate structopt;
#[macro_use] extern crate structopt_derive;

#[macro_use] extern crate error_chain;

use structopt::StructOpt;

#[macro_use] extern crate log;
extern crate loggerv;

#[derive(StructOpt, Debug)]
#[structopt(name = "cleye", about = "A simple terminal-based image viewer.")]
struct Opt {
    #[structopt(short = "f", long = "filter", help = "Filter to apply when resizing image. \
        (Nearest, Triangle, CatmullRom, Gaussian, Lanczos3)", default_value = "Nearest")]
    filter: String,

    #[structopt(help = "Input file to present.")]
    file_path: String,

    #[structopt(short = "p", long = "pixel_width", help = "Number of characters for a given pixel.", default_value="2")]
    pixel_width: usize,

    #[structopt(short = "n", long = "no_render", help = "Dry run, do not render.")]
    no_render: bool,

    #[structopt(short = "v", long = "verbose", help = "Enable logging, use multiple `v`s to increase verbosity.")]
    verbosity: u64,
}

quick_main!(|| -> Result<()> {
    let opt = Opt::from_args();

    loggerv::init_with_verbosity(opt.verbosity)?;

    let img = image::open(&Path::new(&opt.file_path))
        .chain_err(|| format!("Can't open `{}`.", opt.file_path))?;

    debug!("Opened image `{}`.", opt.file_path);

    let term_size = termion::terminal_size()
        .chain_err(|| format!("Could not parse terminal size."))?;

    // Divide term_width by pixel_width since we'll use `pixel_width` num chars per pixel.
    let term_width = term_size.0 as u32 / (opt.pixel_width as u32);
    let term_height = term_size.1 as u32;

    debug!("Terminal size: `({}, {})` (width scale factor: {}).", term_width, term_height, opt.pixel_width);

    // Select our filter.
    let filter = match opt.filter.to_lowercase().as_ref() {
        "nearest"    => Ok(imageops::FilterType::Nearest),
        "triangle"   => Ok(imageops::FilterType::Triangle),
        "catmullrom" => Ok(imageops::FilterType::CatmullRom),
        "gaussian"   => Ok(imageops::FilterType::Gaussian),
        "lanczos3"   => Ok(imageops::FilterType::Lanczos3),
        _ => Err(format!("Unknown filter type `{}`. Try --help.", opt.filter))
    }?;

    debug!("Selected filter: `{}`.", opt.filter);

    // Scale the image to the terminal size.
    let scaled_img = img.resize(term_width, term_height, filter);

    debug!("Scaled image size: `{:?}`.", scaled_img.dimensions());

    // Acquire a reference to the terminal buffer.
    let mut stdout = stdout().into_raw_mode().unwrap();

    if !opt.no_render {
        // Write our pixels as colorized spaces.
        for y in 0..(scaled_img.height()) {
            for x in 0..(scaled_img.width()) {
                let pixel = scaled_img.get_pixel(x, y);
                let (r, g, b) = (pixel[0], pixel[1], pixel[2]);

                write!(stdout,
                       "{}{}",
                       termion::color::Bg(termion::color::Rgb(r, g, b)),
                       " ".repeat(opt.pixel_width))
                    .unwrap();
            }
            write!(stdout, "\n\r").unwrap();
        }
    }

    Ok(())
});

error_chain! {
    foreign_links {
        Log(::log::SetLoggerError);
    }
}
