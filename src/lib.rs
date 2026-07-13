#[cfg(feature = "svg")]
mod svg_plotter;

#[cfg(feature = "bitmap")]
mod bitmap_plotter;

#[cfg(feature = "canvas")]
mod canvas_plotter;

pub(crate) mod common;

pub mod prelude {
    #[cfg(feature = "bitmap")]
    pub use crate::bitmap_plotter::DioxusBitmapDrawingArea;
    #[cfg(feature = "bitmap")]
    pub use crate::bitmap_plotter::DioxusBitmapPlotter;
    #[cfg(feature = "svg")]
    pub use crate::svg_plotter::DioxusSvgDrawingArea;
    #[cfg(feature = "svg")]
    pub use crate::svg_plotter::DioxusSvgPlotter;
    #[cfg(feature = "canvas")]
    pub use crate::canvas_plotter::DioxusCanvasDrawingArea;
    #[cfg(feature = "canvas")]
    pub use crate::canvas_plotter::DioxusCanvasPlotter;
}

pub use prelude::*;
