extern crate svg;

use std::ops::Deref;
use std::path::Path;

use svg::parser::{Event, Parser};
use svg::node::element::path::{Command, Data, Number, Parameters, Position};
use svg::node::element::tag::Path as PathTag;
use svg::node::Value;

use super::common::Point;
use super::path::ParametricPath;

pub struct SVGPath {
    segments: Vec<Box<dyn ParametricPath>>
}

impl SVGPath {
    pub fn new<T: AsRef<Path>>(path: T) -> Result<SVGPath, ()> {
        let svg_file = svg::open(path).unwrap();

        let data = get_first_path_data(svg_file).unwrap();
        let data = Data::parse(&data).unwrap();

        let mut segments: Vec<Box<dyn ParametricPath>> = Vec::new();

        let mut cur_pos = Point::ZERO;

        for command in data.iter() {
            match &command {
                &Command::Move(pos, params) => {
                    let numbers: &[Number] = params.deref();
                    let new_pos = Point::new(
                        *(numbers.get(0).unwrap()) as f64,
                        *(numbers.get(1).unwrap()) as f64
                    );
                    match pos {
                        Position::Absolute => cur_pos = new_pos,
                        Position::Relative => cur_pos += new_pos
                    };
                },
                &Command::Close => {},
                &Command::Line(pos, params) => segments.append(&mut Line::from_parameters(cur_pos, pos, params, &mut cur_pos)),
                &Command::HorizontalLine(pos, params) => segments.append(&mut Line::from_horiz_parameters(cur_pos, pos, params, &mut cur_pos)),
                &Command::VerticalLine(pos, params) => segments.append(&mut Line::from_vert_parameters(cur_pos, pos, params, &mut cur_pos)),
                &Command::QuadraticCurve(pos, params) => segments.append(&mut QuadraticBezierCurve::from_parameters(cur_pos, pos, params, &mut cur_pos)),
                &Command::SmoothQuadraticCurve(pos, params) => segments.append(&mut QuadraticBezierCurve::from_parameters(cur_pos, pos, params, &mut cur_pos)),
                &Command::CubicCurve(pos, params) => segments.append(&mut CubicBezierCurve::from_parameters(cur_pos, pos, params, &mut cur_pos)),
                &Command::SmoothCubicCurve(pos, params) => segments.append(&mut CubicBezierCurve::from_parameters(cur_pos, pos, params, &mut cur_pos)),
                &Command::EllipticalArc(pos, params) => segments.append(&mut EllipticalArc::from_parameters(cur_pos, pos, params, &mut cur_pos))
            }
        }

        Ok( SVGPath { segments } )
    }
}

fn get_first_path_data(mut svg_file: Parser) -> Result<String, ()> {
    let mut path_data: Option<String> = None;

    while let Some(event) = svg_file.next() {
        if let Event::Tag(PathTag, _, attributes) = event {
            if let Some(value) = attributes.get("d") {
                path_data = Some(value.to_string());
            }
        }
    }

    if let Some(data) = path_data {
        Ok(data.to_string())
    } else {
        Err(())
    }
}

impl ParametricPath for SVGPath {
    fn get_point(&self, t: f64) -> Point {
        let scaled_t = t * self.segments.len() as f64;
        let segment_index = scaled_t.floor() as usize;
        let sub_t = scaled_t.fract();

        self.segments[segment_index].get_point(sub_t)
    }
}

struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    fn from_horiz_parameters(start: Point, pos: &Position, params: &Parameters, tail: &mut Point) -> Vec<Box<dyn ParametricPath>> {
        let mut return_vec: Vec<Box<dyn ParametricPath>> = Vec::new();

        let mut numbers = params.deref().iter().peekable();

        let mut last_end = start;

        while numbers.peek().is_some() {
            let start = last_end;
            let mut end = Point::new(*(numbers.next().unwrap()) as f64, start.y);
            if pos == &Position::Relative {
                end.x += start.x
            }
            last_end = end;
            return_vec.push(Box::new(Self {start, end}));
        }

        *tail = last_end;

        return_vec
    }

    fn from_vert_parameters(start: Point, pos: &Position, params: &Parameters, tail: &mut Point) -> Vec<Box<dyn ParametricPath>> {
        let mut return_vec: Vec<Box<dyn ParametricPath>> = Vec::new();

        let mut numbers = params.deref().iter().peekable();

        let mut last_end = start;

        while numbers.peek().is_some() {
            let start = last_end;
            let mut end = Point::new(start.x, *(numbers.next().unwrap()) as f64);
            if pos == &Position::Relative {
                end.y += start.y
            }
            last_end = end;
            return_vec.push(Box::new(Self {start, end}));
        }

        *tail = last_end;

        return_vec
    }

    fn from_parameters(start: Point, pos: &Position, params: &Parameters, tail: &mut Point) -> Vec<Box<dyn ParametricPath>> {
        let mut return_vec: Vec<Box<dyn ParametricPath>> = Vec::new();

        let mut numbers = params.deref().iter().peekable();

        let mut last_end = start;

        while numbers.peek().is_some() {
            let start = last_end;
            let mut end = Point::new(
                *(numbers.next().unwrap()) as f64,
                *(numbers.next().unwrap()) as f64
            );
            if pos == &Position::Relative {
                end += start
            }
            last_end = end;
            return_vec.push(Box::new(Self {start, end}));
        }

        *tail = last_end;

        return_vec
    }
}

impl ParametricPath for Line {
    fn get_point(&self, t: f64) -> Point {
        self.start + (self.end - self.start).scale(t.fract())
    }
}

struct QuadraticBezierCurve {
    start: Point,
    control: Point,
    end: Point
}

impl QuadraticBezierCurve {
    fn new(start: Point, end: Point, control: Point) -> Self {
        Self { start, control, end }
    }

    fn from_parameters(start: Point, pos: &Position, params: &Parameters, tail: &mut Point) -> Vec<Box<dyn ParametricPath>> {
        let mut return_vec: Vec<Box<dyn ParametricPath>> = Vec::new();
        let mut numbers = params.deref().iter().peekable();
        let mut last_end = start;

        while numbers.peek().is_some() {
            let start = last_end;
            let mut control = Point::new(
                *(numbers.next().unwrap()) as f64,
                *(numbers.next().unwrap()) as f64
            );
            let mut end = Point::new(
                *(numbers.next().unwrap()) as f64,
                *(numbers.next().unwrap()) as f64
            );
            if pos == &Position::Relative {
                control += start;
                end += start;
            }
            last_end = end;
            return_vec.push(Box::new(Self::new(start, control, end)));
        }

        *tail = last_end;

        return_vec
    }
}

impl ParametricPath for QuadraticBezierCurve {
    fn get_point(&self, t: f64) -> Point {
        self.start.scale( (1.0-t).powf(2.0) ) +
        self.control.scale(2.0*t*(1.0-t)) +
        self.end.scale(t.powf(2.0))
    }
}

struct CubicBezierCurve {
    start: Point,
    control1: Point,
    control2: Point,
    end: Point
}

impl CubicBezierCurve {
    fn new(start: Point, control1: Point, control2: Point, end: Point) -> Self {
        Self { start, control1, control2, end }
    }

    fn from_parameters(start: Point, pos: &Position, params: &Parameters, tail: &mut Point) -> Vec<Box<dyn ParametricPath>> {
        let mut return_vec: Vec<Box<dyn ParametricPath>> = Vec::new();

        let mut numbers = params.deref().iter().peekable();

        let mut last_end = start;

        while numbers.peek().is_some() {
            let start = last_end;
            let mut control1 = Point::new(
                *(numbers.next().unwrap()) as f64,
                *(numbers.next().unwrap()) as f64
            );
            let mut control2 = Point::new(
                *(numbers.next().unwrap()) as f64,
                *(numbers.next().unwrap()) as f64
            );
            let mut end = Point::new(
                *(numbers.next().unwrap()) as f64,
                *(numbers.next().unwrap()) as f64
            );
            if pos == &Position::Relative {
                control1 += start;
                control2 += start;
                end += start;
            }
            last_end = end;
            return_vec.push(Box::new(Self::new(start, control1, control2, end)));
        }

        *tail = last_end;

        return_vec
    }
}

impl ParametricPath for CubicBezierCurve {
    fn get_point(&self, t: f64) -> Point {
        self.start.scale((1.0-t).powf(3.0)) +
        self.control1.scale( 3.0 * t * (1.0-t).powf(2.0) ) +
        self.control2.scale( 3.0 * t*t * (1.0 - t) ) +
        self.end.scale( t*t*t )
    }
}

struct EllipticalArc {
    start: Point,
    radii: Point,
    x_angle: f64,
    large_arc_flag: bool,
    sweep_flag: bool,
    end: Point
}

impl EllipticalArc {
    fn new(start: Point, radii: Point, x_angle: f64, large_arc_flag: bool, sweep_flag: bool, end: Point) -> Self {
        Self { start, radii, x_angle, large_arc_flag, sweep_flag, end }
    }

    fn from_parameters(start: Point, pos: &Position, params: &Parameters, tail: &mut Point) -> Vec<Box<dyn ParametricPath>> {
        let mut return_vec: Vec<Box<dyn ParametricPath>> = Vec::new();
        let mut numbers = params.deref().iter().peekable();
        let mut last_end = start;

        while numbers.peek().is_some() {
            let start = last_end;
            let radii = Point::new(
                *(numbers.next().unwrap()) as f64,
                *(numbers.next().unwrap()) as f64
            );
            let x_angle = *(numbers.next().unwrap()) as f64;
            let large_arc_flag = *(numbers.next().unwrap()) as f64 != 0.0;
            let sweep_flag = *(numbers.next().unwrap()) as f64 != 0.0;
            let mut end = Point::new(
                *(numbers.next().unwrap()) as f64,
                *(numbers.next().unwrap()) as f64
            );
            if pos == &Position::Relative {
                end += start;
            }
            last_end = end;
            return_vec.push(Box::new(Self::new(start, radii, x_angle, large_arc_flag, sweep_flag, end)));
        }

        *tail = last_end;

        return_vec
    }
}

impl ParametricPath for EllipticalArc {
    fn get_point(&self, t: f64) -> Point { // TODO: Implement Elliptical Arc
        Line::new(self.start, self.end).get_point(t)
    }
}