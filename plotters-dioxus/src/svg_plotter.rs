#![allow(non_snake_case)]

use crate::{BitmapPlotterProps};
use crate::common::PlotEventWrapper;

use dioxus::prelude::*;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::prelude::{IntoDrawingArea, SVGBackend};

pub type DioxusSvgDrawingArea<'a> = DrawingArea<SVGBackend<'a>, Shift>;

#[component]
pub fn DioxusSvgPlotter<F: Fn(&mut DioxusSvgDrawingArea) + Clone>(
    props: BitmapPlotterProps,
    init: F,
) -> Element {
    let mut data = String::new();

    {
        let backend = SVGBackend::with_string(&mut data, props.size);
        let mut drawing_area = backend.into_drawing_area();
        init(&mut drawing_area);
        drawing_area.present().expect("Could not present image.");
    }

    PlotEventWrapper(props, rsx! {div {dangerous_inner_html: data}})
}
