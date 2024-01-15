use {plotters::style::full_palette::GREY_900, proj::Transform};

use {geojson::Feature, plotters::coord::types::RangedCoordf64};

use {
    plotters::{prelude::*, style::full_palette::BLUE_600},
    proj::Coord,
};

pub fn to_drawable_point<'a, DB: DrawingBackend + 'a>(
    p: &geo_types::Point,
    size: f32,
    label: Option<String>,
) -> Box<DynElement<'a, DB, (f64, f64)>> {
    let circ = Circle::new(
        (0, 0),
        size,
        ShapeStyle::from(RGBColor(220, 20, 60)).filled(),
    );

    let empty = EmptyElement::<(f64, f64), DB>::at((p.0.x, p.0.y));

    if let Some(label) = label {
        let t = if label.starts_with("Franklin")
            || label.starts_with("10th")
            || label.starts_with("Lake")
            || label.starts_with("MSP")
            || label.contains('&')
        {
            Text::new(
                label,
                (0, 20),
                FontDesc::new(FontFamily::SansSerif, 10.0, FontStyle::Normal)
                    .color(&RGBColor(35, 31, 32))
                    .transform(FontTransform::Rotate90),
            )
        } else {
            Text::new(
                label,
                (0, -20),
                FontDesc::new(FontFamily::SansSerif, 10.0, FontStyle::Normal)
                    .color(&RGBColor(35, 31, 32))
                    .transform(FontTransform::Rotate270),
            )
        };

        Box::new((empty + circ + t).into_dyn())
    } else {
        Box::new((empty + circ).into_dyn())
    }
}

pub fn to_drawable_path_labels<'a, DB: DrawingBackend + 'a>(
    p: &geo_types::Polygon<f64>,
    _stroke_width: u32,
    _color: RGBColor,
    label: Option<String>,
) -> DynElement<'a, DB, (f64, f64)> {
    let x_iter = p.exterior().0.iter().map(|el| el.x());
    let mid_x = x_iter.sum::<f64>() / p.exterior().0.len() as f64;

    let y_iter = p.exterior().0.iter().map(|el| el.y());
    let mid_y = y_iter.sum::<f64>() / p.exterior().0.len() as f64;

    if let Some(label) = label {
        (EmptyElement::at((mid_x - 1500.0, mid_y + 500.0))
            + Text::new(
                label,
                (0, 0),
                FontDesc::new(FontFamily::SansSerif, 15.0, FontStyle::Normal),
            ))
        .into_dyn()
    } else {
        (EmptyElement::at((mid_x, mid_y))).into_dyn()
    }
}

pub fn to_drawable_path<DB: DrawingBackend>(
    p: &geo_types::Polygon<f64>,
    stroke_width: u32,
    color: RGBColor,
) -> PathElement<(f64, f64)> {
    PathElement::new(
        p.exterior()
            .points()
            .map(|el| (el.0.x(), el.0.y()))
            .collect::<Vec<_>>(),
        ShapeStyle::from(color).stroke_width(stroke_width),
    )
}

pub fn to_drawable_pathl<DB: DrawingBackend>(
    p: &geo_types::LineString,
    stroke_width: u32,
    color: RGBColor,
) -> PathElement<(f64, f64)> {
    PathElement::new(
        p.points().map(|el| (el.0.x, el.0.y)).collect::<Vec<_>>(),
        ShapeStyle::from(color).stroke_width(stroke_width),
    )
}

pub fn to_drawable_multi_pathl<DB: DrawingBackend>(
    p: &geo_types::MultiLineString,
    stroke_width: u32,
) -> Vec<PathElement<(f64, f64)>> {
    p.0.iter()
        .map(|e| to_drawable_pathl::<DB>(e, stroke_width, BLUE_600))
        .collect()
}

pub fn to_drawable_distance_label<'a, DB: DrawingBackend + 'a>(
    points: Vec<(f64, f64)>,
    _stroke_width: u32,
    _color: RGBColor,
) -> DynElement<'a, DB, (f64, f64)> {
    let values = points.iter().next().unwrap();
    (EmptyElement::at((values.0, values.1 - 200.0))
        + Text::new(
            "1 Mile",
            (0, 0),
            FontDesc::new(FontFamily::SansSerif, 10.0, FontStyle::Normal),
        ))
    .into_dyn()
}

pub fn to_drawable_distance<'a, DB: DrawingBackend + 'a>(
    points: Vec<(f64, f64)>,
    stroke_width: u32,
    color: RGBColor,
) -> DynElement<'a, DB, (f64, f64)> {
    PathElement::new(points, ShapeStyle::from(color).stroke_width(stroke_width)).into_dyn()
}

pub fn draw(
    feature: Feature,
    root: &DrawingArea<SVGBackend<'_>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    transform: &proj::Proj,
) {
    let ctu_name = feature
        .properties
        .clone()
        .map(|p| match p.get("CTU_NAME").cloned() {
            Some(v) => match v {
                serde_json::Value::Null => {
                    todo!()
                }
                serde_json::Value::Bool(_) => {
                    todo!()
                }
                serde_json::Value::Number(_) => {
                    todo!()
                }
                serde_json::Value::String(s) => Some(s.replace(" ;", "")),
                serde_json::Value::Array(_) => {
                    todo!()
                }
                serde_json::Value::Object(_) => {
                    todo!()
                }
            },
            None => None,
        })
        .flatten();

    let stop_description = feature
        .properties
        .clone()
        .map(|p| match p.get("StopDescri").cloned() {
            Some(v) => match v {
                serde_json::Value::Null => {
                    todo!()
                }
                serde_json::Value::Bool(_) => {
                    todo!()
                }
                serde_json::Value::Number(_) => {
                    todo!()
                }
                serde_json::Value::String(s) => {
                    if s.contains('-') && s.contains("Lake") {
                        None
                    } else {
                        Some(s.split(';').next().unwrap().replace(" ;", ""))
                    }
                }
                serde_json::Value::Array(_) => {
                    todo!()
                }
                serde_json::Value::Object(_) => {
                    todo!()
                }
            },
            None => None,
        })
        .flatten();

    if let Some(geo) = feature.geometry {
        let mut g: geo_types::Geometry = geo.try_into().unwrap();

        g.transform(&transform).unwrap();

        match g {
            geo_types::Geometry::Point(p) => root
                .draw(&*to_drawable_point::<SVGBackend>(&p, 5.0, stop_description))
                .unwrap(),
            geo_types::Geometry::Line(_) => {}
            geo_types::Geometry::LineString(p) => {
                root.draw(&to_drawable_pathl::<SVGBackend>(&p, 3, RGBColor(0, 0, 130)))
                    .unwrap();
            }
            geo_types::Geometry::Polygon(p) => {
                root.draw(&to_drawable_path_labels::<SVGBackend>(
                    &p, 1, GREY_900, ctu_name,
                ))
                .unwrap();
                root.draw(&to_drawable_path::<SVGBackend>(&p, 1, GREY_900))
                    .unwrap();
            }
            geo_types::Geometry::MultiPoint(_) => {}
            geo_types::Geometry::MultiLineString(m) => {
                for l in to_drawable_multi_pathl::<SVGBackend>(&m, 3) {
                    root.draw(&l).unwrap();
                }
            }
            geo_types::Geometry::MultiPolygon(_poly) => {}
            geo_types::Geometry::GeometryCollection(_) => {}
            geo_types::Geometry::Rect(_) => {}
            geo_types::Geometry::Triangle(_) => {}
        }
    }
}

pub fn to_drawable_train<'a, DB: DrawingBackend + 'a>(
    p: &geo_types::Point,
    size: f32,
    stroke_width: u32,
    color: RGBColor,
) -> Box<DynElement<'a, DB, (f64, f64)>> {
    let circ = Circle::new(
        (0, 0),
        size,
        ShapeStyle::from(color).stroke_width(stroke_width),
    );

    let empty = EmptyElement::<(f64, f64), DB>::at((p.0.x, p.0.y));

    Box::new((empty + circ).into_dyn())
}

pub fn draw_train(
    feature: Feature,
    root: &DrawingArea<SVGBackend<'_>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    transform: &proj::Proj,
) {
    if let Some(geo) = feature.geometry {
        let mut g: geo_types::Geometry = geo.try_into().unwrap();

        g.transform(&transform).unwrap();

        match g {
            geo_types::Geometry::Point(p) => root
                .draw(&*to_drawable_train::<SVGBackend>(
                    &p,
                    7.0,
                    2,
                    RGBColor(45, 32, 31),
                ))
                .unwrap(),
            geo_types::Geometry::Line(_) => {}
            geo_types::Geometry::LineString(p) => {}
            geo_types::Geometry::Polygon(p) => {}
            geo_types::Geometry::MultiPoint(_) => {}
            geo_types::Geometry::MultiLineString(m) => {}
            geo_types::Geometry::MultiPolygon(_poly) => {}
            geo_types::Geometry::GeometryCollection(_) => {}
            geo_types::Geometry::Rect(_) => {}
            geo_types::Geometry::Triangle(_) => {}
        }
    }
}
