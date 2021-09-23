use {usvg, resvg, tiny_skia};
use std::fs::File;
use std::io::prelude::*;
use std::error;
use std::fmt;
use png;
type Result<T> = std::result::Result<T, RenderError>;

#[derive(Debug)]
enum RenderError{
    ConvertSVG(usvg::Error),
    SavePNG(png::EncodingError),
    EmptyMap,
    EmptyRender,
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self {
            RenderError::ConvertSVG(ref e) => e.fmt(f),
            RenderError::SavePNG(ref e) => e.fmt(f),
            RenderError::EmptyMap => write!(f, "pixmap is empty"),
            RenderError::EmptyRender => write!(f, "no rendering result"),
        }
    }
}

impl error::Error for RenderError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            RenderError::ConvertSVG(ref e) => Some(e),
            RenderError::SavePNG(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<usvg::Error> for RenderError {
    fn from(err: usvg::Error) -> RenderError {
        RenderError::ConvertSVG(err)
    }
}

impl From<png::EncodingError> for RenderError {
    fn from(err: png::EncodingError) -> RenderError {
        RenderError::SavePNG(err)
    }
}

fn to_png(element: &str) -> Result<&str> {
    let option = usvg::Options::default();
    let tree = usvg::Tree::from_str(element, &option.to_ref())?;
    let pixmap_size = tree.svg_node().size.to_screen_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
                        .ok_or(RenderError::EmptyMap)?;
    resvg::render(&tree, usvg::FitTo::Original, pixmap.as_mut())
                        .ok_or(RenderError::EmptyRender)?;
    Ok("Render success")
}

fn main() {
    let filename = "dummy.svg";
    let mut svg_string = String::new();
    let mut f = File::open(filename).expect("file not found");
    f.read_to_string(&mut svg_string).expect("something wrong with reading the file");
    let r = to_png(&svg_string);
    match r {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", e),
    }
}