
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

#[derive(Props, Clone)]
pub struct DioxusBitmapPlotterProps<F: Fn(&mut DioxusBitmapDrawingArea) + Clone + 'static> {
    init: F,
    size: (u32, u32),
    #[props(default)]
    draggable: bool,
    on_click: Option<EventHandler<MouseEvent>>,
    on_doubleclick: Option<EventHandler<MouseEvent>>,
    on_mousemove: Option<EventHandler<MouseEvent>>,
    on_mouseout: Option<EventHandler<MouseEvent>>,
    on_mouseup: Option<EventHandler<MouseEvent>>,
    on_mousedown: Option<EventHandler<MouseEvent>>,
    on_mouseover: Option<EventHandler<MouseEvent>>,
    on_wheel: Option<EventHandler<WheelEvent>>,
    on_drag: Option<EventHandler<DragEvent>>,
    on_dragend: Option<EventHandler<DragEvent>>,
    on_dragenter: Option<EventHandler<DragEvent>>,
    on_dragleave: Option<EventHandler<DragEvent>>,
    on_dragover: Option<EventHandler<DragEvent>>,
    on_dragstart: Option<EventHandler<DragEvent>>,
    on_drop: Option<EventHandler<DragEvent>>,
    on_scroll: Option<EventHandler<ScrollEvent>>,
}

impl<F: Fn(&mut DioxusBitmapDrawingArea) + Clone + 'static> PartialEq
    for DioxusBitmapPlotterProps<F>
{
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        true
    }
}
#[allow(non_snake_case)]
pub fn DioxusBitmapPlotter<F: Fn(&mut DioxusBitmapDrawingArea) + Clone + 'static>(
    props: DioxusBitmapPlotterProps<F>,
) -> Element {
    let buffer_size = ((props.size.1 * props.size.0) as usize) * 3_usize;
    let mut buffer: Vec<u8> = vec![0; buffer_size];

    {
        let backend = BitMapBackend::with_buffer(&mut buffer, props.size);
        let mut drawing_area = backend.into_drawing_area();
        (props.init)(&mut drawing_area);
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

    rsx! {
        PlotEventWrapper {
            size: props.size,
            draggable: props.draggable,
            on_click:props.on_click,
            on_doubleclick:props.on_doubleclick,
            on_mousemove:props.on_mousemove,
            on_mouseout:props.on_mouseout,
            on_mouseup:props.on_mouseup,
            on_mousedown:props.on_mousedown,
            on_mouseover:props.on_mouseover,
            on_wheel:props.on_wheel,
            on_drag:props.on_drag,
            on_dragend:props.on_dragend,
            on_dragenter:props.on_dragenter,
            on_dragleave:props.on_dragleave,
            on_dragover:props.on_dragover,
            on_dragstart:props.on_dragstart,
            on_drop:props.on_drop,
            on_scroll:props.on_scroll,
            img {src: "data:image/png;base64,{buffer_base64}"},
        }
    }
}
