use std::collections::HashMap;
use std::f32::consts::PI;
use std::iter::Map;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
enum Shape {
    Circle {
        center: Point,
        radius: f32,
    },
    Rectange {
        top_left: Point,
        bottom_right: Point,
    },
    Triangle(Point, Point, Point),
}

impl Point {
    fn magnify(point: &Self, mag: f32) -> Point {
        Point {
            x: point.x * mag,
            y: point.y * mag,
        }
    }
}

fn get_area(shape: Shape) -> f32 {
    match shape {
        Shape::Circle { center, radius } => PI * radius * radius,
        Shape::Rectange {
            top_left,
            bottom_right,
        } => (bottom_right.x * top_left.x) * (top_left.y - bottom_right.y),
        Shape::Triangle(p1, p2, p3) => {
            let a = ((p1.x - p3.x).powi(2) + (p1.y - p3.y).powi(2)).sqrt();
            let b = ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt();
            let c = ((p2.x - p3.x).powi(2) + (p2.y - p3.y).powi(2)).sqrt();
            let p = (a + b + c) / 2.0;
            (p * (p - a) * (p - b) * (p - c)).sqrt()
        }
    }
}

fn set(){
    let point = Point { x: 1.0, y: 1.0 };

    let circle = Shape::Circle {
        center: Point { x: 2.0, y: 3.1 },
        radius: 2.0,
    };

    let point_a = Point { x: 3.5, y: 2.7 };
    let point_b = Point { x: 2.1, y: 1.3 };

    let magnify_a = Point::magnify(&point_a, 10.2);
    let rectangle = Shape::Rectange {
        top_left: magnify_a,
        bottom_right: point_b,
    };

    let triangle = Shape::Triangle(
        Point { x: 2.3, y: 3.13 },
        Point { x: 3.1, y: 3.3 },
        Point { x: 3.1, y: 3.5 },
    );

    println!("{:?}", get_area(rectangle));
}