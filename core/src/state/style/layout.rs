use std::fmt::write;

use crate::Interpolator;
use crate::Entity;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaxWidth(pub f32);

impl Default for MaxWidth {
    fn default() -> Self {
        MaxWidth(std::f32::INFINITY)
    }
}

impl Interpolator for MaxWidth {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        MaxWidth(start.0 + (end.0 - start.0) * t)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaxHeight(pub f32);

impl Default for MaxHeight {
    fn default() -> Self {
        MaxHeight(std::f32::INFINITY)
    }
}

impl Interpolator for MaxHeight {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        MaxHeight(start.0 + (end.0 - start.0) * t)
    }
}

// #[derive(Copy, Clone, PartialEq, Debug)]
// pub enum Justify {
//     Start,
//     Center,
//     End,
// }

// impl Default for Justify {
//     fn default() -> Self {
//         Justify::Start
//     }
// }

// #[derive(Copy, Clone, PartialEq, Debug)]
// pub enum Align {
//     Start,
//     Center,
//     End,
// }

// impl Default for Align {
//     fn default() -> Self {
//         Align::Center
//     }
// }

// Not currently used

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Overflow {
    Visible,
    Hidden,
}

impl Default for Overflow {
    fn default() -> Self {
        Overflow::Hidden
    }
}

// #[derive(Copy, Clone, Debug, PartialEq)]
// pub struct Scroll {
//     pub x: f32,
//     pub y: f32,
//     pub w: f32,
//     pub h: f32,
// }

// impl Default for Scroll {
//     fn default() -> Self {
//         Scroll {
//             x: 0.0,
//             y: 0.0,
//             w: 1.0,
//             h: 1.0,
//         }
//     }
// }
