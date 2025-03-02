use crate::drawbehaviour::DrawBehaviour;
use crate::render;

use chrono::{TimeZone, Utc};
use p2d::bounding_volume::AABB;
use rand::distributions::Uniform;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use super::bitmapimage::BitmapImage;
use super::brushstroke::BrushStroke;
use super::markerstroke::MarkerStroke;
use super::shapestroke::ShapeStroke;
use super::strokebehaviour::StrokeBehaviour;
use super::vectorimage::VectorImage;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "strokestyle")]
pub enum StrokeStyle {
    #[serde(rename = "markerstroke")]
    MarkerStroke(MarkerStroke),
    #[serde(rename = "brushstroke")]
    BrushStroke(BrushStroke),
    #[serde(rename = "shapestroke")]
    ShapeStroke(ShapeStroke),
    #[serde(rename = "vectorimage")]
    VectorImage(VectorImage),
    #[serde(rename = "bitmapimage")]
    BitmapImage(BitmapImage),
}

impl DrawBehaviour for StrokeStyle {
    fn bounds(&self) -> AABB {
        match self {
            Self::MarkerStroke(markerstroke) => markerstroke.bounds(),
            Self::BrushStroke(brushstroke) => brushstroke.bounds(),
            Self::ShapeStroke(shapestroke) => shapestroke.bounds(),
            Self::VectorImage(vectorimage) => vectorimage.bounds(),
            Self::BitmapImage(bitmapimage) => bitmapimage.bounds(),
        }
    }

    fn set_bounds(&mut self, bounds: AABB) {
        match self {
            Self::MarkerStroke(markerstroke) => markerstroke.set_bounds(bounds),
            Self::BrushStroke(brushstroke) => brushstroke.set_bounds(bounds),
            Self::ShapeStroke(shapestroke) => shapestroke.set_bounds(bounds),
            Self::VectorImage(vectorimage) => vectorimage.set_bounds(bounds),
            Self::BitmapImage(bitmapimage) => bitmapimage.set_bounds(bounds),
        }
    }

    fn gen_svgs(&self, offset: na::Vector2<f64>) -> Result<Vec<render::Svg>, anyhow::Error> {
        match self {
            Self::MarkerStroke(markerstroke) => markerstroke.gen_svgs(offset),
            Self::BrushStroke(brushstroke) => brushstroke.gen_svgs(offset),
            Self::ShapeStroke(shapestroke) => shapestroke.gen_svgs(offset),
            Self::VectorImage(vectorimage) => vectorimage.gen_svgs(offset),
            Self::BitmapImage(bitmapimage) => bitmapimage.gen_svgs(offset),
        }
    }
}

impl StrokeBehaviour for StrokeStyle {
    fn translate(&mut self, offset: na::Vector2<f64>) {
        match self {
            Self::MarkerStroke(markerstroke) => {
                markerstroke.translate(offset);
            }
            Self::BrushStroke(brushstroke) => {
                brushstroke.translate(offset);
            }
            Self::ShapeStroke(shapestroke) => {
                shapestroke.translate(offset);
            }
            Self::VectorImage(vectorimage) => {
                vectorimage.translate(offset);
            }
            Self::BitmapImage(bitmapimage) => {
                bitmapimage.translate(offset);
            }
        }
    }

    fn rotate(&mut self, angle: f64, center: na::Point2<f64>) {
        match self {
            Self::MarkerStroke(markerstroke) => {
                markerstroke.rotate(angle, center);
            }
            Self::BrushStroke(brushstroke) => {
                brushstroke.rotate(angle, center);
            }
            Self::ShapeStroke(shapestroke) => {
                shapestroke.rotate(angle, center);
            }
            Self::VectorImage(vectorimage) => {
                vectorimage.rotate(angle, center);
            }
            Self::BitmapImage(bitmapimage) => {
                bitmapimage.rotate(angle, center);
            }
        }
    }

    fn scale(&mut self, scale: nalgebra::Vector2<f64>) {
        match self {
            Self::MarkerStroke(markerstroke) => {
                markerstroke.scale(scale);
            }
            Self::BrushStroke(brushstroke) => {
                brushstroke.scale(scale);
            }
            Self::ShapeStroke(shapestroke) => {
                shapestroke.scale(scale);
            }
            Self::VectorImage(vectorimage) => {
                vectorimage.scale(scale);
            }
            Self::BitmapImage(bitmapimage) => {
                bitmapimage.scale(scale);
            }
        }
    }
}

impl Default for StrokeStyle {
    fn default() -> Self {
        Self::MarkerStroke(MarkerStroke::default())
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InputData {
    pos: na::Vector2<f64>,
    pressure: f64,
}

impl Default for InputData {
    fn default() -> Self {
        Self {
            pos: na::vector![0.0, 0.0],
            pressure: Self::PRESSURE_DEFAULT,
        }
    }
}

impl InputData {
    pub const PRESSURE_DEFAULT: f64 = 0.5;

    pub fn new(pos: na::Vector2<f64>, pressure: f64) -> Self {
        let mut inputdata = Self::default();
        inputdata.set_pos(pos);
        inputdata.set_pressure(pressure);

        inputdata
    }

    pub fn pos(&self) -> na::Vector2<f64> {
        self.pos
    }

    pub fn set_pos(&mut self, pos: na::Vector2<f64>) {
        self.pos = pos;
    }

    pub fn pressure(&self) -> f64 {
        self.pressure
    }

    pub fn set_pressure(&mut self, pressure: f64) {
        self.pressure = pressure.clamp(0.0, 1.0);
    }
}

// Represents a single Stroke Element
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename = "element")]
pub struct Element {
    #[serde(rename = "inputdata")]
    pub inputdata: InputData,
    #[serde(rename = "timpestamp", default = "default_datetime")]
    pub timestamp: chrono::DateTime<Utc>,
}

pub fn default_datetime() -> chrono::DateTime<Utc> {
    Utc.ymd(2000, 1, 1).and_hms(12, 0, 0)
}

impl Element {
    pub fn new(inputdata: InputData) -> Self {
        let timestamp = Utc::now();

        Self {
            inputdata,
            timestamp,
        }
    }

    pub fn validation_data(bounds: AABB) -> Vec<Self> {
        let mut rng = rand::thread_rng();
        let data_entries_uniform = Uniform::from(0..=20);
        let x_uniform = Uniform::from(bounds.mins[0]..=bounds.maxs[0]);
        let y_uniform = Uniform::from(bounds.mins[1]..=bounds.maxs[1]);
        let pressure_uniform = Uniform::from(0_f64..=1_f64);

        let mut data_entries: Vec<Self> = Vec::new();

        for _i in 0..=data_entries_uniform.sample(&mut rng) {
            data_entries.push(Self::new(InputData::new(
                na::vector![x_uniform.sample(&mut rng), y_uniform.sample(&mut rng)],
                pressure_uniform.sample(&mut rng),
            )));
        }

        data_entries
    }
}
