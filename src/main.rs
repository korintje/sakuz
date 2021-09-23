use {usvg, resvg, tiny_skia};
//use std::rc::Rc;
//use xmlwriter;
use std::fs::File;
use std::io::prelude::*;

fn render_element(element: &str) -> Result<&str, &str> {
    let option = usvg::Options::default();
    match usvg::Tree::from_str(element, &option.to_ref()){
        Ok(usvgtree) => {
            let pixmap_size = usvgtree.svg_node().size.to_screen_size();
            match tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()){
                Some(mut pixmap) => {
                    match resvg::render(&usvgtree, usvg::FitTo::Original, pixmap.as_mut()){
                        Some(_is_rendered) => {
                            match pixmap.save_png("test.png"){
                                Ok(_is_saved) => Ok("Successfully savd"),
                                Err(_err) => Err("failed to save svg file"),
                            }
                        },
                        None => Err("could not be rendered"),
                    }
                },
                None => Err("no pixmap was obtained"),
            }
        },
        Err(_err) => Err("Failed to convert string to tree"),
    }
}

fn main() {
    let filename = "icon.svg";
    let mut svg_string = String::new();
    let mut f = File::open(filename).expect("file not found");
    f.read_to_string(&mut svg_string).expect("something wrong with reading the file");
    render_element(&svg_string).unwrap();
}