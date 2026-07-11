#[cfg(feature = "svg")]
mod svg_plotter;

#[cfg(feature = "bitmap")]
mod bitmap_plotter;
pub(crate) mod common;

pub mod prelude {
    pub use crate::common::BitmapPlotterProps;

    #[cfg(feature = "bitmap")]
    pub use crate::bitmap_plotter::DioxusBitmapDrawingArea;
    #[cfg(feature = "bitmap")]
    pub use crate::bitmap_plotter::DioxusBitmapPlotter;
    #[cfg(feature = "svg")]
    pub use crate::svg_plotter::DioxusSvgDrawingArea;
    #[cfg(feature = "svg")]
    pub use crate::svg_plotter::DioxusSvgPlotter;
}

pub use prelude::*;
