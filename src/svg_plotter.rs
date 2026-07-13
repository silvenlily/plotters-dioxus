use crate::common::PlotEventWrapper;
use dioxus::prelude::*;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::prelude::{IntoDrawingArea, SVGBackend};

pub type DioxusSvgDrawingArea<'a> = DrawingArea<SVGBackend<'a>, Shift>;

#[derive(Props, Clone)]
pub struct DioxusSvgPlotterProps<F: Fn(&mut DioxusSvgDrawingArea) + Clone + 'static> {
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

impl<F: Fn(&mut DioxusSvgDrawingArea) + Clone + 'static> PartialEq for DioxusSvgPlotterProps<F> {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        true
    }
}

#[allow(non_snake_case)]
pub fn DioxusSvgPlotter<F: Fn(&mut DioxusSvgDrawingArea) + Clone + 'static>(props: DioxusSvgPlotterProps<F>) -> Element {
    let mut data = String::new();

    {
        let backend = SVGBackend::with_string(&mut data, props.size);
        let mut drawing_area = backend.into_drawing_area();
        (props.init)(&mut drawing_area);
        drawing_area.present().expect("Could not present image.");
    }

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
            div {dangerous_inner_html: data}
        }
    }
}
