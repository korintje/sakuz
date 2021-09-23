use {usvg, resvg, tiny_skia};
use std::rc::Rc;
use xmlwriter;

fn main() {

    // Load fonts from the system
    let mut opt = usvg::Options::default();
    opt.fontdb.load_system_fonts();

    //let svg_data = std::fs::read(&args[1]).unwrap();
    // let rtree = usvg::Tree::from_data(&svg_data, &opt.to_ref()).unwrap();
    let size = usvg::Size::new(128.0, 128.0).unwrap();
    let vbox = usvg::ViewBox{
        rect: usvg::Rect::new(96.0, 96.0, 96.0, 96.0).unwrap(),
        aspect: usvg::AspectRatio{
            defer: false,
            align: usvg::Align::None,
            slice: true,
        }
    };
    let svg = usvg::Svg{
        size: size,
        view_box: vbox,
    };
    let mut rtree = usvg::Tree::create(svg);
    let trans = usvg::Transform{
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 0.0,
        f: 0.0,
    };
    let color = usvg::Color{
        red: 24,
        green: 32,
        blue: 16,
        alpha: 10,
    };
    let paint = usvg::Paint::Color(color);
    let fill = usvg::Fill{
        paint: paint.clone(),
        opacity: usvg::Opacity::new(0.5),
        rule:  usvg::FillRule::NonZero,
    };
    let stroke = usvg::Stroke{
        paint: paint,
        dasharray: Some(vec![1.0, 1.0, 1.0, 1.0]),
        dashoffset: 0.5,
        miterlimit: usvg::StrokeMiterlimit::new(1.0),
        opacity: usvg::Opacity::new(0.5),
        width: usvg::StrokeWidth::new(1.0),
        linecap: usvg::LineCap::Square,
        linejoin: usvg::LineJoin::Round,
    };
    //let mut pathdata = usvg::PathData::new();
    let pathdata = usvg::PathData::from_rect(usvg::Rect::new(10.0, 20.0, 10.0, 20.0).unwrap());
    //pathdata.push_curve_to(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    //pathdata.push_curve_to(10.0, 2.0, 3.0, 14.0, 1.0, 1.0);
    let path = usvg::Path{
        id: "0".to_string(),
        transform: trans,
        visibility: usvg::Visibility::Visible,
        fill: Some(fill),
        stroke: Some(stroke),
        //stroke: None,
        rendering_mode: usvg::ShapeRendering::OptimizeSpeed,
        text_bbox: None,
        data: Rc::new(pathdata),
    };
    println!("{:?}", rtree.defs());
    rtree.append_to_defs(usvg::NodeKind::Path(path));
    let xmlopt = usvg::XmlOptions {id_prefix: None, writer_opts: xmlwriter::Options{
        use_single_quote: true,
        indent: xmlwriter::Indent::None,
        attributes_indent: xmlwriter::Indent::None,
    }};
    println!("{:?}", rtree.to_string(&xmlopt));

    let pixmap_size = rtree.svg_node().size.to_screen_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(&rtree, usvg::FitTo::Original, pixmap.as_mut()).unwrap();
    pixmap.save_png("test.png").unwrap();
}