//! # Gymnarium: Visualisers Base
//!
//! `gymnarium_visualisers_base` is a collection of structs, traits and enums to support creating
//! visualisations for reinforcement environments within the gymnarium libraries.

#![feature(iterator_fold_self)]

#[macro_use]
extern crate serde_derive;
extern crate gymnarium_base;
extern crate serde;

use std::error::Error;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

use gymnarium_base::math::{Position2D, Size2D, Transformation2D, Transformations2D, Vector2D};
use gymnarium_base::{Agent, AgentAction, EnvironmentState, Reward, Seed};

pub mod input;

/* --- --- --- Rgb --- --- --- */

/// Used for the RgbArray.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Pixel {
    pub fn with(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn white() -> Self {
        Self {
            red: 255,
            green: 255,
            blue: 255,
        }
    }

    pub fn black() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn red() -> Self {
        Self {
            red: 255,
            green: 0,
            blue: 0,
        }
    }

    pub fn green() -> Self {
        Self {
            red: 0,
            green: 255,
            blue: 0,
        }
    }

    pub fn blue() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 255,
        }
    }

    pub fn yellow() -> Self {
        Self {
            red: 255,
            green: 255,
            blue: 0,
        }
    }

    pub fn cyan() -> Self {
        Self {
            red: 0,
            green: 255,
            blue: 255,
        }
    }

    pub fn magenta() -> Self {
        Self {
            red: 255,
            green: 0,
            blue: 255,
        }
    }
}

/* --- --- --- Color --- --- --- */

/// Classic red, green, blue and alpha for defining color.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn with(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn transparent() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        }
    }

    pub fn white() -> Self {
        Self {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 255,
        }
    }

    pub fn black() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 255,
        }
    }

    pub fn red() -> Self {
        Self {
            red: 255,
            green: 0,
            blue: 0,
            alpha: 255,
        }
    }

    pub fn green() -> Self {
        Self {
            red: 0,
            green: 255,
            blue: 0,
            alpha: 255,
        }
    }

    pub fn blue() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 255,
            alpha: 255,
        }
    }

    pub fn yellow() -> Self {
        Self {
            red: 255,
            green: 255,
            blue: 0,
            alpha: 255,
        }
    }

    pub fn cyan() -> Self {
        Self {
            red: 0,
            green: 255,
            blue: 255,
            alpha: 255,
        }
    }

    pub fn magenta() -> Self {
        Self {
            red: 255,
            green: 0,
            blue: 255,
            alpha: 255,
        }
    }
}

impl Color {
    pub fn float_array(&self) -> [f32; 4] {
        [
            self.red as f32 / 255f32,
            self.green as f32 / 255f32,
            self.blue as f32 / 255f32,
            self.alpha as f32 / 255f32,
        ]
    }
}

/* --- --- --- RgbArray --- --- --- */

/// The type returned from the RgbArrayDrawing.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct PixelArray {
    pub data: Vec<Pixel>,
    pub width: usize,
    pub height: usize,
}

impl PixelArray {
    pub fn with(width: usize, height: usize) -> Self {
        Self {
            data: Vec::new(),
            width,
            height,
        }
    }
}

impl Index<(usize, usize)> for PixelArray {
    type Output = Pixel;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.0 >= self.width || index.1 >= self.height {
            panic!();
        }
        &self.data[index.1 * self.width + index.0]
    }
}

impl IndexMut<(usize, usize)> for PixelArray {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if index.0 >= self.width || index.1 >= self.height {
            panic!();
        }
        &mut self.data[index.1 * self.width + index.0]
    }
}

/* --- --- --- LineShape --- --- --- */

/// The shape of a Line or Polyline.
#[derive(Debug, Clone, PartialEq)]
pub enum LineShape {
    /// Square edges
    Square,
    /// Round edges
    Round,
    /// Bevel edges
    Bevel,
}

/* --- --- --- CornerShape --- --- --- */

/// The shape of corners.
#[derive(Debug, Clone, PartialEq)]
pub enum CornerShape {
    /// Square corners.
    Square,
    /// Round corners, with resolution per corner.
    Round(f64, u32),
    /// Bevel corners.
    Bevel(f64),
}

/* --- --- --- TextureSource --- --- --- */

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub enum TextureSource {
    Path(String),
    /// Contains the texture as byte array.
    ///
    /// The data is usually generated by calling
    /// `let data: Vec<u8> = image::open("img.jpg").unwrap().into_rgba().into_vec();` from
    /// the `image` crate.
    ///
    /// The user of this data can reverse this by calling
    /// `let img = image::ImageBuffer::from_vec(width, height, data).unwrap();` from the `image`
    /// crate.
    ///
    /// A possibility to include this inside an environment crate is to use base64 encoded strings.
    /// First the file is going to be base64-encoded and stored inside a file within the
    /// environment crate:
    /// `base64::encode(image::open("img.jpg").unwrap().into_rgba().into_vec());`.
    /// Afterwards this file is included inside the environment crate like:
    /// `const BASE64STRING: &'static str = include_str!("img.base64");`.
    /// Then afterwards the image can be loaded by calling:
    /// `let texture_source = TextureSource::bytes(base64::decode(BASE64STRING).unwrap(), 100, 100);`.
    Bytes {
        data: Vec<u8>,
        width: u32,
        height: u32,
    },
}

impl TextureSource {
    pub fn path(path: String) -> Self {
        Self::Path(path)
    }

    pub fn bytes(data: Vec<u8>, width: u32, height: u32) -> Self {
        Self::Bytes {
            data,
            width,
            height,
        }
    }
}

/* --- --- --- Geometry2D --- --- --- */

/// All supported primitives inside the two dimensional space.
#[derive(Debug, Clone, PartialEq)]
pub enum Geometry2D {
    Point {
        position: Position2D,
        color: Color,
        transformations: Transformations2D,
    },
    Line {
        points: [Position2D; 2],
        line_color: Color,
        line_width: f64,
        line_shape: LineShape,
        transformations: Transformations2D,
    },
    Polyline {
        points: Vec<Position2D>,
        line_color: Color,
        line_width: f64,
        line_shape: LineShape,
        transformations: Transformations2D,
    },
    Triangle {
        points: [Position2D; 3],
        fill_color: Color,
        border_color: Color,
        border_width: f64,
        transformations: Transformations2D,
    },
    Square {
        center_position: Position2D,
        edge_length: f64,
        fill_color: Color,
        border_color: Color,
        border_width: f64,
        corner_shape: CornerShape,
        transformations: Transformations2D,
    },
    Rectangle {
        center_position: Position2D,
        size: Size2D,
        fill_color: Color,
        border_color: Color,
        border_width: f64,
        corner_shape: CornerShape,
        transformations: Transformations2D,
    },
    Polygon {
        points: Vec<Position2D>,
        fill_color: Color,
        border_color: Color,
        border_width: f64,
        transformations: Transformations2D,
    },
    Circle {
        center_position: Position2D,
        radius: f64,
        fill_color: Color,
        border_color: Color,
        border_width: f64,
        transformations: Transformations2D,
    },
    Ellipse {
        center_position: Position2D,
        size: Size2D,
        fill_color: Color,
        border_color: Color,
        border_width: f64,
        transformations: Transformations2D,
    },
    // TODO: Text
    Image {
        center_position: Position2D,
        size: Size2D,
        texture_source: TextureSource,
        source_rectangle: Option<(Position2D, Size2D)>,
        fill_color: Option<Color>,
        transformations: Transformations2D,
    },
    Group(Vec<Geometry2D>),
}

impl Geometry2D {
    /// Creates a new point at the given position.
    ///
    /// Defaults are `color: Color::black()`.
    pub fn point(position: Position2D) -> Self {
        Self::Point {
            position,
            color: Color::black(),
            transformations: Transformations2D::default(),
        }
    }

    /// Creates a new line between the given positions.
    ///
    /// Defaults are `color: Color::black()`, `line_width: 1f64` and
    /// `line_shape: LineShape::Square`.
    pub fn line(start: Position2D, end: Position2D) -> Self {
        Self::Line {
            points: [start, end],
            line_color: Color::black(),
            line_width: 1f64,
            line_shape: LineShape::Square,
            transformations: Transformations2D::default(),
        }
    }

    /// Creates a new polyline with the given positions.
    ///
    /// Defaults are `color: Color::black()`, `line_width: 1f64` and
    /// `line_shape: LineShape::Square`.
    pub fn polyline(points: Vec<Position2D>) -> Self {
        Self::Polyline {
            points,
            line_color: Color::black(),
            line_width: 1f64,
            line_shape: LineShape::Square,
            transformations: Transformations2D::default(),
        }
    }

    /// Creates a new triangle with the given three positions.
    ///
    /// Defaults are `fill_color: Color::black()`, `border_color: Color::transparent()` and
    /// `border_width: 0f64`.
    pub fn triangle(
        position_a: Position2D,
        position_b: Position2D,
        position_c: Position2D,
    ) -> Self {
        Self::Triangle {
            points: [position_a, position_b, position_c],
            fill_color: Color::black(),
            border_color: Color::transparent(),
            border_width: 0f64,
            transformations: Transformations2D::default(),
        }
    }

    /// Creates a new square with the given center position and edge length.
    ///
    /// Defaults are `fill_color: Color::black()`, `border_color: Color::transparent()`,
    /// `border_width: 0f64` and `corner_shape: CornerShape::Square`.
    pub fn square(center_position: Position2D, edge_length: f64) -> Self {
        Self::Square {
            center_position,
            edge_length,
            fill_color: Color::black(),
            border_color: Color::transparent(),
            border_width: 0f64,
            corner_shape: CornerShape::Square,
            transformations: Transformations2D::default(),
        }
    }

    /// Creates a new rectangle with the given center position and size.
    ///
    /// Defaults are `fill_color: Color::black()`, `border_color: Color::transparent()`,
    /// `border_width: 0f64` and `corner_shape: CornerShape::Square`.
    pub fn rectangle(center_position: Position2D, size: Size2D) -> Self {
        Self::Rectangle {
            center_position,
            size,
            fill_color: Color::black(),
            border_color: Color::transparent(),
            border_width: 0f64,
            corner_shape: CornerShape::Square,
            transformations: Transformations2D::default(),
        }
    }

    /// Creates a new polygon with the given positions.
    ///
    /// Defaults are `fill_color: Color::black()`, `border_color: Color::transparent()` and
    /// `border_width: 0f64`.
    pub fn polygon(points: Vec<Position2D>) -> Self {
        Self::Polygon {
            points,
            fill_color: Color::black(),
            border_color: Color::transparent(),
            border_width: 0f64,
            transformations: Transformations2D::default(),
        }
    }

    /// Creates a new circle with the given center position and radius.
    ///
    /// Defaults are `fill_color: Color::black()`, `border_color: Color::transparent()` and
    /// `border_width: 0f64`.
    pub fn circle(center_position: Position2D, radius: f64) -> Self {
        Self::Circle {
            center_position,
            radius,
            fill_color: Color::black(),
            border_color: Color::transparent(),
            border_width: 0f64,
            transformations: Transformations2D::default(),
        }
    }

    /// Creates a new ellipse with the given center position and size.
    ///
    /// Defaults are `fill_color: Color::black()`, `border_color: Color::transparent()` and
    /// `border_width: 0f64`.
    pub fn ellipse(center_position: Position2D, size: Size2D) -> Self {
        Self::Ellipse {
            center_position,
            size,
            fill_color: Color::black(),
            border_color: Color::transparent(),
            border_width: 0f64,
            transformations: Transformations2D::default(),
        }
    }

    /// Creates a new image with the given center position and selected image.
    ///
    /// Defaults are `source_rectangle: None` and `fill_color: None`.
    pub fn image(center_position: Position2D, size: Size2D, texture_source: TextureSource) -> Self {
        Self::Image {
            center_position,
            size,
            texture_source,
            source_rectangle: None,
            fill_color: None,
            transformations: Transformations2D::default(),
        }
    }

    /// Creates a new group with the given geometries.
    pub fn group(geometries: Vec<Geometry2D>) -> Self {
        Self::Group(geometries)
    }
}

impl Geometry2D {
    pub fn center_position_of_bounding_box(&self) -> Position2D {
        let max_bb_pos = self.maximum_position_in_transformed_bounding_box();
        let min_bb_pos = self.minimum_position_in_transformed_bounding_box();
        Position2D::with(
            (max_bb_pos.x / 2f64) + (min_bb_pos.x / 2f64),
            (max_bb_pos.y / 2f64) + (min_bb_pos.y / 2f64),
        )
    }

    pub fn size_of_bounding_box(&self) -> Size2D {
        let max_bb_pos = self.maximum_position_in_transformed_bounding_box();
        let min_bb_pos = self.minimum_position_in_transformed_bounding_box();
        Size2D::with(max_bb_pos.x - min_bb_pos.x, max_bb_pos.y - min_bb_pos.y)
    }

    pub fn minimum_position_in_transformed_bounding_box(&self) -> Position2D {
        self.conditional_position_in_transformed_bounding_box(|a, b| {
            Position2D::with(a.x.min(b.x), a.y.min(b.y))
        })
        .unwrap_or_else(|| Position2D::with(std::f64::MAX, std::f64::MAX))
    }

    pub fn maximum_position_in_transformed_bounding_box(&self) -> Position2D {
        self.conditional_position_in_transformed_bounding_box(|a, b| {
            Position2D::with(a.x.max(b.x), a.y.max(b.y))
        })
        .unwrap_or_else(|| Position2D::with(std::f64::MIN, std::f64::MIN))
    }

    fn conditional_position_in_transformed_bounding_box<
        F: Fn(Position2D, Position2D) -> Position2D + Copy,
    >(
        &self,
        merge_two_positions: F,
    ) -> Option<Position2D> {
        match self {
            Self::Point {
                position,
                transformations,
                ..
            } => Some(position.transform(transformations)),
            Self::Line {
                points,
                transformations,
                ..
            } => Some(merge_two_positions(
                points[0].transform(transformations),
                points[1].transform(transformations),
            )),
            Self::Polyline {
                points,
                transformations,
                ..
            } => points
                .iter()
                .map(|a| a.transform(transformations))
                .fold_first(merge_two_positions),
            Self::Triangle {
                points,
                transformations,
                ..
            } => Some(merge_two_positions(
                merge_two_positions(
                    points[0].transform(transformations),
                    points[1].transform(transformations),
                ),
                points[2].transform(transformations),
            )),
            Self::Square {
                center_position,
                edge_length,
                transformations,
                ..
            } => [
                *center_position - Vector2D::with(edge_length / 2f64, edge_length / 2f64),
                *center_position - Vector2D::with(-edge_length / 2f64, edge_length / 2f64),
                *center_position - Vector2D::with(-edge_length / 2f64, -edge_length / 2f64),
                *center_position - Vector2D::with(edge_length / 2f64, -edge_length / 2f64),
            ]
            .iter()
            .map(|a| a.transform(transformations))
            .fold_first(merge_two_positions),
            Self::Rectangle {
                center_position,
                size,
                transformations,
                ..
            } => [
                *center_position - Vector2D::with(size.width / 2f64, size.height / 2f64),
                *center_position - Vector2D::with(-size.width / 2f64, size.height / 2f64),
                *center_position - Vector2D::with(-size.width / 2f64, -size.height / 2f64),
                *center_position - Vector2D::with(size.width / 2f64, -size.height / 2f64),
            ]
            .iter()
            .map(|a| a.transform(transformations))
            .fold_first(merge_two_positions),
            Self::Polygon {
                points,
                transformations,
                ..
            } => points
                .iter()
                .map(|a| a.transform(transformations))
                .fold_first(merge_two_positions),
            Self::Circle {
                center_position,
                radius,
                transformations,
                ..
            } => [
                *center_position - Vector2D::with(*radius, *radius),
                *center_position - Vector2D::with(-*radius, *radius),
                *center_position - Vector2D::with(-*radius, -*radius),
                *center_position - Vector2D::with(*radius, -*radius),
            ]
            .iter()
            .map(|a| a.transform(transformations))
            .fold_first(merge_two_positions),
            Self::Ellipse {
                center_position,
                size,
                transformations,
                ..
            } => [
                *center_position - Vector2D::with(size.width / 2f64, size.height / 2f64),
                *center_position - Vector2D::with(-size.width / 2f64, size.height / 2f64),
                *center_position - Vector2D::with(-size.width / 2f64, -size.height / 2f64),
                *center_position - Vector2D::with(size.width / 2f64, -size.height / 2f64),
            ]
            .iter()
            .map(|a| a.transform(transformations))
            .fold_first(merge_two_positions),
            Self::Image {
                center_position,
                size,
                transformations,
                ..
            } => [
                *center_position - Vector2D::with(size.width / 2f64, size.height / 2f64),
                *center_position - Vector2D::with(-size.width / 2f64, size.height / 2f64),
                *center_position - Vector2D::with(-size.width / 2f64, -size.height / 2f64),
                *center_position - Vector2D::with(size.width / 2f64, -size.height / 2f64),
            ]
            .iter()
            .map(|a| a.transform(transformations))
            .fold_first(merge_two_positions),
            Self::Group(geometries) => geometries
                .iter()
                .map(|geometry| {
                    geometry.conditional_position_in_transformed_bounding_box(merge_two_positions)
                })
                .filter(|position| position.is_some())
                .map(|position| position.unwrap())
                .fold_first(merge_two_positions),
        }
    }
}

impl Geometry2D {
    pub fn line_or_border_color(self, new_line_or_border_color: Color) -> Self {
        match self {
            p @ Self::Point { .. } => p,
            Self::Line {
                points,
                line_width,
                line_shape,
                transformations,
                ..
            } => Self::Line {
                points,
                line_color: new_line_or_border_color,
                line_width,
                line_shape,
                transformations,
            },
            Self::Polyline {
                points,
                line_width,
                line_shape,
                transformations,
                ..
            } => Self::Polyline {
                points,
                line_color: new_line_or_border_color,
                line_width,
                line_shape,
                transformations,
            },
            Self::Triangle {
                points,
                fill_color,
                border_width,
                transformations,
                ..
            } => Self::Triangle {
                points,
                fill_color,
                border_color: new_line_or_border_color,
                border_width,
                transformations,
            },
            Self::Square {
                center_position,
                edge_length,
                fill_color,
                border_width,
                corner_shape,
                transformations,
                ..
            } => Self::Square {
                center_position,
                edge_length,
                fill_color,
                border_color: new_line_or_border_color,
                border_width,
                corner_shape,
                transformations,
            },
            Self::Rectangle {
                center_position,
                size,
                fill_color,
                border_width,
                corner_shape,
                transformations,
                ..
            } => Self::Rectangle {
                center_position,
                size,
                fill_color,
                border_color: new_line_or_border_color,
                border_width,
                corner_shape,
                transformations,
            },
            Self::Polygon {
                points,
                fill_color,
                border_width,
                transformations,
                ..
            } => Self::Polygon {
                points,
                border_color: new_line_or_border_color,
                fill_color,
                border_width,
                transformations,
            },
            Self::Circle {
                center_position,
                radius,
                fill_color,
                border_width,
                transformations,
                ..
            } => Self::Circle {
                center_position,
                radius,
                fill_color,
                border_color: new_line_or_border_color,
                border_width,
                transformations,
            },
            Self::Ellipse {
                center_position,
                size,
                fill_color,
                border_width,
                transformations,
                ..
            } => Self::Ellipse {
                center_position,
                size,
                border_color: new_line_or_border_color,
                fill_color,
                border_width,
                transformations,
            },
            i @ Self::Image { .. } => i,
            Self::Group(geometries) => Self::Group(
                geometries
                    .into_iter()
                    .map(|geometry| geometry.line_or_border_color(new_line_or_border_color))
                    .collect(),
            ),
        }
    }

    pub fn line_or_border_width(self, new_line_or_border_width: f64) -> Self {
        match self {
            p @ Self::Point { .. } => p,
            Self::Line {
                points,
                line_color,
                line_shape,
                transformations,
                ..
            } => Self::Line {
                points,
                line_color,
                line_width: new_line_or_border_width,
                line_shape,
                transformations,
            },
            Self::Polyline {
                points,
                line_color,
                line_shape,
                transformations,
                ..
            } => Self::Polyline {
                points,
                line_color,
                line_width: new_line_or_border_width,
                line_shape,
                transformations,
            },
            Self::Triangle {
                points,
                fill_color,
                border_color,
                transformations,
                ..
            } => Self::Triangle {
                points,
                fill_color,
                border_color,
                border_width: new_line_or_border_width,
                transformations,
            },
            Self::Square {
                center_position,
                edge_length,
                fill_color,
                border_color,
                corner_shape,
                transformations,
                ..
            } => Self::Square {
                center_position,
                edge_length,
                fill_color,
                border_color,
                border_width: new_line_or_border_width,
                corner_shape,
                transformations,
            },
            Self::Rectangle {
                center_position,
                size,
                fill_color,
                border_color,
                corner_shape,
                transformations,
                ..
            } => Self::Rectangle {
                center_position,
                size,
                fill_color,
                border_color,
                border_width: new_line_or_border_width,
                corner_shape,
                transformations,
            },
            Self::Polygon {
                points,
                fill_color,
                border_color,
                transformations,
                ..
            } => Self::Polygon {
                points,
                fill_color,
                border_color,
                border_width: new_line_or_border_width,
                transformations,
            },
            Self::Circle {
                center_position,
                radius,
                fill_color,
                border_color,
                transformations,
                ..
            } => Self::Circle {
                center_position,
                radius,
                fill_color,
                border_color,
                border_width: new_line_or_border_width,
                transformations,
            },
            Self::Ellipse {
                center_position,
                size,
                fill_color,
                border_color,
                transformations,
                ..
            } => Self::Ellipse {
                center_position,
                size,
                fill_color,
                border_color,
                border_width: new_line_or_border_width,
                transformations,
            },
            i @ Self::Image { .. } => i,
            Self::Group(geometries) => Self::Group(
                geometries
                    .into_iter()
                    .map(|geometry| geometry.line_or_border_width(new_line_or_border_width))
                    .collect(),
            ),
        }
    }

    pub fn line_shape(self, new_line_shape: LineShape) -> Self {
        match self {
            Self::Line {
                points,
                line_color,
                line_width,
                transformations,
                ..
            } => Self::Line {
                points,
                line_color,
                line_width,
                line_shape: new_line_shape,
                transformations,
            },
            Self::Polyline {
                points,
                line_color,
                line_width,
                transformations,
                ..
            } => Self::Polyline {
                points,
                line_color,
                line_width,
                line_shape: new_line_shape,
                transformations,
            },
            g => g,
        }
    }

    pub fn corner_shape(self, new_corner_shape: CornerShape) -> Self {
        match self {
            Self::Square {
                center_position,
                edge_length,
                fill_color,
                border_color,
                border_width,
                transformations,
                ..
            } => Self::Square {
                center_position,
                edge_length,
                fill_color,
                border_color,
                border_width,
                corner_shape: new_corner_shape,
                transformations,
            },
            Self::Rectangle {
                center_position,
                size,
                fill_color,
                border_color,
                border_width,
                transformations,
                ..
            } => Self::Rectangle {
                center_position,
                size,
                fill_color,
                border_color,
                border_width,
                corner_shape: new_corner_shape,
                transformations,
            },
            g => g,
        }
    }

    pub fn fill_color(self, new_fill_color: Color) -> Self {
        match self {
            Self::Point {
                position,
                transformations,
                ..
            } => Self::Point {
                position,
                color: new_fill_color,
                transformations,
            },
            l @ Self::Line { .. } => l,
            pl @ Self::Polyline { .. } => pl,
            Self::Triangle {
                points,
                border_color,
                border_width,
                transformations,
                ..
            } => Self::Triangle {
                points,
                fill_color: new_fill_color,
                border_color,
                border_width,
                transformations,
            },
            Self::Square {
                center_position,
                edge_length,
                border_color,
                border_width,
                corner_shape,
                transformations,
                ..
            } => Self::Square {
                center_position,
                edge_length,
                fill_color: new_fill_color,
                border_color,
                border_width,
                corner_shape,
                transformations,
            },
            Self::Rectangle {
                center_position,
                size,
                border_color,
                border_width,
                corner_shape,
                transformations,
                ..
            } => Self::Rectangle {
                center_position,
                size,
                fill_color: new_fill_color,
                border_color,
                border_width,
                corner_shape,
                transformations,
            },
            Self::Polygon {
                points,
                border_color,
                border_width,
                transformations,
                ..
            } => Self::Polygon {
                points,
                fill_color: new_fill_color,
                border_color,
                border_width,
                transformations,
            },
            Self::Circle {
                center_position,
                radius,
                border_color,
                border_width,
                transformations,
                ..
            } => Self::Circle {
                center_position,
                radius,
                fill_color: new_fill_color,
                border_color,
                border_width,
                transformations,
            },
            Self::Ellipse {
                center_position,
                size,
                border_color,
                border_width,
                transformations,
                ..
            } => Self::Ellipse {
                center_position,
                size,
                border_color,
                fill_color: new_fill_color,
                border_width,
                transformations,
            },
            Self::Image {
                center_position,
                size,
                texture_source,
                source_rectangle,
                transformations,
                ..
            } => Self::Image {
                center_position,
                size,
                texture_source,
                source_rectangle,
                fill_color: Some(new_fill_color),
                transformations,
            },
            Self::Group(geometries) => Self::Group(
                geometries
                    .into_iter()
                    .map(|geometry| geometry.fill_color(new_fill_color))
                    .collect(),
            ),
        }
    }

    pub fn append_transformation(self, transformation: Transformation2D) -> Self {
        match self {
            Self::Point {
                position,
                color,
                mut transformations,
            } => Self::Point {
                position,
                color,
                transformations: {
                    transformations.transformations.push(transformation);
                    transformations
                },
            },
            Self::Line {
                points,
                line_color,
                line_width,
                line_shape,
                mut transformations,
            } => Self::Line {
                points,
                line_color,
                line_width,
                line_shape,
                transformations: {
                    transformations.transformations.push(transformation);
                    transformations
                },
            },
            Self::Polyline {
                points,
                line_color,
                line_width,
                line_shape,
                mut transformations,
            } => Self::Polyline {
                points,
                line_color,
                line_width,
                line_shape,
                transformations: {
                    transformations.transformations.push(transformation);
                    transformations
                },
            },
            Self::Triangle {
                points,
                fill_color,
                border_color,
                border_width,
                mut transformations,
            } => Self::Triangle {
                points,
                fill_color,
                border_color,
                border_width,
                transformations: {
                    transformations.transformations.push(transformation);
                    transformations
                },
            },
            Self::Square {
                center_position,
                edge_length,
                fill_color,
                border_color,
                border_width,
                corner_shape,
                mut transformations,
            } => Self::Square {
                center_position,
                edge_length,
                fill_color,
                border_color,
                border_width,
                corner_shape,
                transformations: {
                    transformations.transformations.push(transformation);
                    transformations
                },
            },
            Self::Rectangle {
                center_position,
                size,
                fill_color,
                border_color,
                border_width,
                corner_shape,
                mut transformations,
            } => Self::Rectangle {
                center_position,
                size,
                fill_color,
                border_color,
                border_width,
                corner_shape,
                transformations: {
                    transformations.transformations.push(transformation);
                    transformations
                },
            },
            Self::Polygon {
                points,
                fill_color,
                border_color,
                border_width,
                mut transformations,
            } => Self::Polygon {
                points,
                fill_color,
                border_color,
                border_width,
                transformations: {
                    transformations.transformations.push(transformation);
                    transformations
                },
            },
            Self::Circle {
                center_position,
                radius,
                fill_color,
                border_color,
                border_width,
                mut transformations,
            } => Self::Circle {
                center_position,
                radius,
                fill_color,
                border_color,
                border_width,
                transformations: {
                    transformations.transformations.push(transformation);
                    transformations
                },
            },
            Self::Ellipse {
                center_position,
                size,
                fill_color,
                border_color,
                border_width,
                mut transformations,
            } => Self::Ellipse {
                center_position,
                size,
                fill_color,
                border_color,
                border_width,
                transformations: {
                    transformations.transformations.push(transformation);
                    transformations
                },
            },
            Self::Image {
                center_position,
                size,
                texture_source,
                source_rectangle,
                fill_color,
                mut transformations,
            } => Self::Image {
                center_position,
                size,
                texture_source,
                source_rectangle,
                fill_color,
                transformations: {
                    transformations.transformations.push(transformation);
                    transformations
                },
            },
            Self::Group(geometries) => Self::Group(
                geometries
                    .into_iter()
                    .map(|geometry| geometry.append_transformation(transformation.clone()))
                    .collect(),
            ),
        }
    }

    pub fn move_by(self, distance: Vector2D) -> Self {
        self.append_transformation(Transformation2D::translation(distance))
    }

    pub fn move_to(self, center_position: &Position2D) -> Self {
        let center_position_of_bounding_box = self.center_position_of_bounding_box();
        self.move_by(center_position_of_bounding_box.vector_to(center_position))
    }

    pub fn rotate_around_origin(self, degree: f64) -> Self {
        self.rotate_around(&Position2D::zero(), degree)
    }

    pub fn rotate_around_self(self, degree: f64) -> Self {
        let center_position_of_bounding_box = self.center_position_of_bounding_box();
        self.rotate_around(&center_position_of_bounding_box, degree)
    }

    pub fn rotate_around(self, rotate_position: &Position2D, degree: f64) -> Self {
        self.append_transformation(Transformation2D::rotation_around_position(
            rotate_position,
            degree,
        ))
    }

    pub fn scale_position(self, scale_factor: f64) -> Self {
        let center_position_of_bounding_box = self.center_position_of_bounding_box();
        self.move_to(
            &(Position2D::zero()
                + (Position2D::zero().vector_to(&center_position_of_bounding_box) * scale_factor)),
        )
    }

    pub fn transform(self, source_viewport: &Viewport2D, target_viewport: &Viewport2D) -> Self {
        let mut transformations = vec![Transformation2D::translation(
            source_viewport.center.vector_to(&Position2D::zero()),
        )];
        if source_viewport.flipped_y_axis != target_viewport.flipped_y_axis {
            transformations.push(Transformation2D::reflection_y());
        }
        if source_viewport.flipped_x_axis != target_viewport.flipped_x_axis {
            transformations.push(Transformation2D::reflection_x());
        }
        transformations.push(Transformation2D::scale(
            target_viewport.size.width / source_viewport.size.width,
            target_viewport.size.height / source_viewport.size.height,
        ));
        transformations.push(Transformation2D::translation(
            Position2D::zero().vector_to(&target_viewport.center),
        ));

        self.append_transformation(Transformation2D::composition(
            "ViewportTransformation".to_string(),
            transformations,
        ))
    }
}

/* --- --- --- Geometry3D --- --- --- */

/// All supported primitives inside the three dimensional space.
///
/// *To be honest, currently I have no idea which are needed, and will add them later.*
#[derive(Debug, Clone, PartialEq)]
pub enum Geometry3D {
    // TODO: Which primitives are there?
// Possibilities:
// - https://www.peachpit.com/articles/article.aspx?p=30594&seqNum=5
// - http://docs.daz3d.com/doku.php/public/software/hexagon/2/referenceguide/3d_primitives/start
}

/* --- --- --- Viewport2D --- --- --- */

/// A viewing rectangle inside the two dimensional space.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Viewport2D {
    pub center: Position2D,
    pub size: Size2D,
    pub flipped_x_axis: bool,
    pub flipped_y_axis: bool,
}

impl Viewport2D {
    pub fn with(center: Position2D, size: Size2D) -> Self {
        Self {
            center,
            size,
            flipped_x_axis: false,
            flipped_y_axis: false,
        }
    }

    pub fn flipped_x_axis(&self, flipped: bool) -> Self {
        Self {
            center: self.center,
            size: self.size,
            flipped_x_axis: flipped,
            flipped_y_axis: self.flipped_y_axis,
        }
    }

    pub fn flipped_y_axis(&self, flipped: bool) -> Self {
        Self {
            center: self.center,
            size: self.size,
            flipped_x_axis: self.flipped_x_axis,
            flipped_y_axis: flipped,
        }
    }
}

/* --- --- --- Viewport3D --- --- --- */

/// Maybe the camera view inside the three dimensional space.
///
/// *To be honest, currently I have no idea how to define something like that, and will add it
/// later.*
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Viewport3D {
    /*camera_position: Position3D,
view_direction: TODO: How to define camera view direction?,
view_depth: f64,
field_of_view: Size2D,
TODO: Is this the correct way to define a 3D viewport?
*/}

/* --- --- --- Viewport2DModification --- --- --- */

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Viewport2DModification {
    LooseAspectRatio,
    KeepAspectRatio,
    KeepAspectRatioAndScissorRemains,
}

impl Default for Viewport2DModification {
    fn default() -> Self {
        Self::LooseAspectRatio
    }
}

/* --- --- --- InputProvider --- --- --- */

/// Trait to use for the InputAgent or any other Agent.
pub trait InputProvider {
    fn clear(&mut self);
    fn peek(&self) -> Option<input::Input>;
    fn pop(&mut self) -> Option<input::Input>;
    fn pop_all(&mut self) -> Vec<input::Input>;
}

/* --- --- --- InputAgentError --- --- --- */

#[derive(Debug)]
pub enum InputAgentError<TAMError: Error> {
    ToActionMapperError(TAMError),
}

impl<TAMError: Error> Display for InputAgentError<TAMError> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl<TAMError: Error> Error for InputAgentError<TAMError> {}

impl<TAMError: Error> From<TAMError> for InputAgentError<TAMError> {
    fn from(error: TAMError) -> Self {
        Self::ToActionMapperError(error)
    }
}

/* --- --- --- InputAgent --- --- --- */

pub struct InputAgent<
    IP: InputProvider,
    TAMError: Error,
    TAM: gymnarium_base::ToActionMapper<Vec<input::Input>, TAMError>,
> {
    input_provider: IP,
    to_action_mapper: TAM,
    phantom: PhantomData<TAMError>,
}

impl<
        IP: InputProvider,
        TAMError: Error,
        TAM: gymnarium_base::ToActionMapper<Vec<input::Input>, TAMError>,
    > InputAgent<IP, TAMError, TAM>
{
    pub fn new(input_provider: IP, to_action_mapper: TAM) -> Self {
        Self {
            input_provider,
            to_action_mapper,
            phantom: PhantomData,
        }
    }
}

impl<
        IP: InputProvider,
        R: Reward,
        TAMError: Error,
        TAM: gymnarium_base::ToActionMapper<Vec<input::Input>, TAMError>,
    > Agent<InputAgentError<TAMError>, R, InputAgentStorage> for InputAgent<IP, TAMError, TAM>
{
    fn reseed(&mut self, _random_seed: Option<Seed>) -> Result<(), InputAgentError<TAMError>> {
        Ok(())
    }

    fn reset(&mut self) -> Result<(), InputAgentError<TAMError>> {
        self.input_provider.clear();
        Ok(())
    }

    fn choose_action(
        &mut self,
        _state: &EnvironmentState,
    ) -> Result<AgentAction, InputAgentError<TAMError>> {
        self.to_action_mapper
            .map(&self.input_provider.pop_all())
            .map_err(|e| e.into())
    }

    fn process_reward(
        &mut self,
        _old_state: &EnvironmentState,
        _last_action: &AgentAction,
        _new_state: &EnvironmentState,
        _reward: R,
        _is_done: bool,
    ) -> Result<(), InputAgentError<TAMError>> {
        Ok(())
    }

    fn load(&mut self, _: InputAgentStorage) -> Result<(), InputAgentError<TAMError>> {
        Ok(())
    }

    fn store(&self) -> InputAgentStorage {
        InputAgentStorage {}
    }

    fn close(&mut self) -> Result<(), InputAgentError<TAMError>> {
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct InputAgentStorage;

/* --- --- --- Visualiser --- --- --- */

/// Base trait for any structure which wants to visualise environments.
pub trait Visualiser<VisualiserError: Error> {
    fn is_open(&self) -> bool;
    fn close(&mut self) -> Result<(), VisualiserError>;
}

/* --- --- --- TwoDimensionalVisualiser --- --- --- */

/// Trait for visualisers capable of visualising two dimensional geometries.
pub trait TwoDimensionalVisualiser<
    TwoDimensionalVisualiserError: Error,
    VisualiserError: Error,
    DrawableEnvironmentError: Error,
>: Visualiser<VisualiserError>
{
    fn render_two_dimensional<
        DrawableEnvironment: TwoDimensionalDrawableEnvironment<DrawableEnvironmentError>,
    >(
        &mut self,
        drawable_environment: &DrawableEnvironment,
    ) -> Result<(), TwoDimensionalVisualiserError>;
}

/* --- --- --- ThreeDimensionalVisualiser --- --- --- */

/// Trait for visualisers capable of visualising three dimensional geometries.
pub trait ThreeDimensionalVisualiser<
    ThreeDimensionalVisualiserError: Error,
    VisualiserError: Error,
    DrawableEnvironmentError: Error,
>: Visualiser<VisualiserError>
{
    fn render_three_dimensional<
        DrawableEnvironment: ThreeDimensionalDrawableEnvironment<DrawableEnvironmentError>,
    >(
        &mut self,
        drawable_environment: &DrawableEnvironment,
    ) -> Result<(), ThreeDimensionalVisualiserError>;
}

/* --- --- --- RgbArrayVisualiser --- --- --- */

/// Trait for visualisers capable of visualising two dimensional pixel arrays.
pub trait PixelArrayVisualiser<
    PixelArrayVisualiserError: Error,
    VisualiserError: Error,
    DrawableEnvironmentError: Error,
>: Visualiser<VisualiserError>
{
    fn render_pixel_array<
        DrawableEnvironment: PixelArrayDrawableEnvironment<DrawableEnvironmentError>,
    >(
        &mut self,
        drawable_environment: &DrawableEnvironment,
    ) -> Result<(), PixelArrayVisualiserError>;
}

/* --- --- --- TextVisualiser --- --- --- */

/// Trait for visualisers capable of visualising texts.
pub trait TextVisualiser<
    TextVisualiserError: Error,
    VisualiserError: Error,
    DrawableEnvironmentError: Error,
>: Visualiser<VisualiserError>
{
    fn render_text<DrawableEnvironment: TextDrawableEnvironment<DrawableEnvironmentError>>(
        &mut self,
        drawable_environment: &DrawableEnvironment,
    ) -> Result<(), TextVisualiserError>;
}

/* --- --- --- DrawableEnvironment --- --- --- */

/// Base trait for any environment which wants to provide functions for visualisation.
pub trait DrawableEnvironment {
    fn suggested_rendered_steps_per_second() -> Option<f64>;
}

/* --- --- --- TwoDimensionalDrawableEnvironment --- --- --- */

/// Trait for drawable environments providing functions returning two dimensional geometries.
pub trait TwoDimensionalDrawableEnvironment<EnvironmentError: Error>: DrawableEnvironment {
    fn draw_two_dimensional(&self) -> Result<Vec<Geometry2D>, EnvironmentError>;
    fn preferred_view(&self) -> Option<(Viewport2D, Viewport2DModification)>;
    fn preferred_background_color(&self) -> Option<Color>;
}

/* --- --- --- ThreeDimensionalDrawableEnvironment --- --- --- */

/// Trait for drawable environments providing functions returning three dimensional geometries.
pub trait ThreeDimensionalDrawableEnvironment<EnvironmentError: Error>:
    DrawableEnvironment
{
    fn draw_three_dimensional(
        &self,
    ) -> Result<(Vec<Geometry3D>, Vec<Geometry2D>), EnvironmentError>;
}

/* --- --- --- RgbArrayDrawableEnvironment --- --- --- */

/// Trait for drawable environments providing functions returning two dimensional pixels.
pub trait PixelArrayDrawableEnvironment<EnvironmentError: Error>: DrawableEnvironment {
    fn draw_pixel_array(&self) -> Result<PixelArray, EnvironmentError>;
}

/* --- --- --- TextDrawableEnvironment --- --- --- */

/// Trait for drawable environments providing functions returning text.
pub trait TextDrawableEnvironment<EnvironmentError: Error>: DrawableEnvironment {
    fn draw_text(&self) -> Result<String, EnvironmentError>;
}
