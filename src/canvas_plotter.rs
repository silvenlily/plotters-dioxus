use crate::common::PlotEventWrapper;
use dioxus::prelude::*;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::prelude::{IntoDrawingArea};
use plotters_canvas::CanvasBackend;
use web_sys::{Element as WebElement, HtmlCanvasElement};
use web_sys::wasm_bindgen::JsCast;

pub type DioxusCanvasDrawingArea = DrawingArea<CanvasBackend, Shift>;

#[derive(Props, Clone)]
pub struct DioxusCanvasPlotterProps<F: Fn(&mut DioxusCanvasDrawingArea) + Clone + 'static> {
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

impl<F: Fn(&mut DioxusCanvasDrawingArea) + Clone + 'static> PartialEq for DioxusCanvasPlotterProps<F> {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        true
    }
}

fn redraw<F: Fn(&mut DioxusCanvasDrawingArea) + Clone + 'static>(canvas:Signal<Option<HtmlCanvasElement>>, drawer: Signal<F>) {
    let c = if let Some(canvas) = canvas.read().cloned() {
        canvas
    } else {
        return;
    };

    let backend = CanvasBackend::with_canvas_object(c).expect("Could not get canvas object");
    let mut drawing_area = backend.into_drawing_area();
    drawer.read()(&mut drawing_area);
    drawing_area.present().expect("Could not present image.");
}

#[allow(non_snake_case)]
pub fn DioxusCanvasPlotter<F: Fn(&mut DioxusCanvasDrawingArea) + Clone + 'static>(props: DioxusCanvasPlotterProps<F>) -> Element {

    let mut can:Signal<Option<HtmlCanvasElement>> = use_signal(||None);

    let init:Signal<F> = use_signal(||props.init);

    rsx! {
        PlotEventWrapper {
            size: props.size,
            draggable: props.draggable,
            on_click:move |e| {
                if let Some(f) = props.on_click {
                    f(e);
                    redraw(can,init)
                }},
            on_doubleclick:move |e| {
                if let Some(f) = props.on_doubleclick {
                    f(e);
                    redraw(can,init)
                }},
            on_mousemove:move |e| {
                if let Some(f) = props.on_mousemove {
                    f(e);
                    redraw(can,init)
                }},
            on_mouseout:move |e| {
                if let Some(f) = props.on_mouseout {
                    f(e);
                    redraw(can,init)
                }},
            on_mouseup:move |e| {
                if let Some(f) = props.on_mouseup {
                    f(e);
                    redraw(can,init)
                }},
            on_mousedown:move |e| {
                if let Some(f) = props.on_mousedown {
                    f(e);
                    redraw(can,init)
                }},
            on_mouseover:move |e| {
                if let Some(f) = props.on_mouseover {
                    f(e);
                    redraw(can,init)
                }},
            on_wheel:move |e| {
                if let Some(f) = props.on_wheel {
                    f(e);
                    redraw(can,init)
                }},
            on_drag:move |e| {
                if let Some(f) = props.on_drag {
                    f(e);
                    redraw(can,init)
                }},
            on_dragend:move |e| {
                if let Some(f) = props.on_dragend {
                    f(e);
                    redraw(can,init)
                }},
            on_dragenter:move |e| {
                if let Some(f) = props.on_dragenter {
                    f(e);
                    redraw(can,init)
                }},
            on_dragleave:move |e| {
                if let Some(f) = props.on_dragleave {
                    f(e);
                    redraw(can,init)
                }},
            on_dragover:move |e| {
                if let Some(f) = props.on_dragover {
                    f(e);
                    redraw(can,init)
                }},
            on_dragstart:move |e| {
                if let Some(f) = props.on_dragstart {
                    f(e);
                    redraw(can,init)
                }},
            on_drop:move |e| {
                if let Some(f) = props.on_drop {
                    f(e);
                    redraw(can,init)
                }},
            on_scroll:move |e| {
                if let Some(f) = props.on_scroll {
                    f(e);
                    redraw(can,init)
                }},
            canvas {
                width: props.size.0,
                height: props.size.1,
                onmounted: move |e:Event<MountedData>| {
                    let element = e.data();

                    let canvas = element
                        .downcast::<WebElement>()
                        .expect("Mounted element is not a web_sys::Element")
                        .clone()
                        .dyn_into::<HtmlCanvasElement>()
                        .expect("Mounted element is not a canvas");

                    can.set(Some(canvas));
                    redraw(can,init)
                }
            }
        }
    }
}
