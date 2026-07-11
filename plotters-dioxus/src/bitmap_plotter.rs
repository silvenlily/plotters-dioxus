use crate::{BitmapPlotterProps};
use crate::common::PlotEventWrapper;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;

use dioxus::prelude::*;

use image::codecs::png::PngEncoder;
use image::ImageEncoder;

use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::prelude::{BitMapBackend, IntoDrawingArea};

use std::io::Cursor;

pub type DioxusBitmapDrawingArea<'a> = DrawingArea<BitMapBackend<'a>, Shift>;

#[component]
pub fn DioxusBitmapPlotter<F: Fn(&mut DioxusBitmapDrawingArea) + Clone>(
    props: BitmapPlotterProps,
    init: F,
) -> Element {
    let buffer_size = ((props.size.1 * props.size.0) as usize) * 3_usize;
    let mut buffer: Vec<u8> = vec![0; buffer_size];

    {
        let backend = BitMapBackend::with_buffer(&mut buffer, props.size);
        let mut drawing_area = backend.into_drawing_area();
        init(&mut drawing_area);
        drawing_area.present().expect("Could not present image.");
    }

    let mut data = Vec::new();
    {
        let cursor = Cursor::new(&mut data);
        let encoder = PngEncoder::new(cursor);
        let color = image::ColorType::Rgb8;

        encoder
            .write_image(buffer.as_slice(), props.size.0, props.size.1, color.into())
            .expect("The Png encoder is expected to write the image");
    }

    let buffer_base64 = BASE64_STANDARD.encode(data);

    PlotEventWrapper(
        props,
        rsx! {img {src: "data:image/png;base64,{buffer_base64}"}},
    )
}
