use dioxus::prelude::*;

#[component]
pub(crate) fn PlotEventWrapper(props: BitmapPlotterProps, child: Element) -> Element {
    rsx!(div {
        onclick: move |e| {
            if let Some(callback) = props.on_click.as_ref() {
                callback.call(e)
            };
        },
        ondoubleclick: move |e| {
            if let Some(callback) = props.on_dblclick.as_ref() {
                callback.call(e)
            };
        },
        onmousemove: move |e| {
            if let Some(callback) = props.on_mousemove.as_ref() {
                callback.call(e)
            };
        },
        onmousedown: move |e| {
            if let Some(callback) = props.on_mousedown.as_ref() {
                callback.call(e)
            };
        },
        onmouseup: move |e| {
            if let Some(callback) = props.on_mouseup.as_ref() {
                callback.call(e)
            };
        },
        onmouseout: move |e| {
            if let Some(callback) = props.on_mouseout.as_ref() {
                callback.call(e)
            };
        },
        onmouseover: move |e| {
            if let Some(callback) = props.on_mouseover.as_ref() {
                callback.call(e)
            };
        },
        onwheel: move |e| {
            if let Some(callback) = props.on_wheel.as_ref() {
                callback.call(e)
            };
        },
        ondrag: move |e| {
            if let Some(callback) = props.on_drag.as_ref() {
                callback.call(e)
            };
        },
        ondragend: move |e| {
            if let Some(callback) = props.on_dragend.as_ref() {
                callback.call(e)
            };
        },
        ondragenter: move |e| {
            if let Some(callback) = props.on_dragenter.as_ref() {
                callback.call(e)
            };
        },
        ondragleave: move |e| {
            if let Some(callback) = props.on_dragleave.as_ref() {
                callback.call(e)
            };
        },
        ondragover: move |e| {
            if let Some(callback) = props.on_dragover.as_ref() {
                callback.call(e)
            };
        },
        ondragstart: move |e| {
            if let Some(callback) = props.on_dragstart.as_ref() {
                callback.call(e)
            };
        },
        ondrop: move |e| {
            if let Some(callback) = props.on_drop.as_ref() {
                callback.call(e)
            };
        },
        onscroll: move |e| {
            if let Some(callback) = props.on_scroll.as_ref() {
                callback.call(e)
            };
        },
        draggable: props.draggable,
        {child}
    })
}

#[derive(Props, Clone, Default, PartialEq)]
pub struct BitmapPlotterProps {
    pub size: (u32, u32),
    pub draggable: bool,
    pub on_click: Option<EventHandler<MouseEvent>>,
    pub on_dblclick: Option<EventHandler<MouseEvent>>,
    pub on_mousemove: Option<EventHandler<MouseEvent>>,
    pub on_mouseout: Option<EventHandler<MouseEvent>>,
    pub on_mouseup: Option<EventHandler<MouseEvent>>,
    pub on_mousedown: Option<EventHandler<MouseEvent>>,
    pub on_mouseover: Option<EventHandler<MouseEvent>>,
    pub on_wheel: Option<EventHandler<WheelEvent>>,
    pub on_drag: Option<EventHandler<DragEvent>>,
    pub on_dragend: Option<EventHandler<DragEvent>>,
    pub on_dragenter: Option<EventHandler<DragEvent>>,
    pub on_dragleave: Option<EventHandler<DragEvent>>,
    pub on_dragover: Option<EventHandler<DragEvent>>,
    pub on_dragstart: Option<EventHandler<DragEvent>>,
    pub on_drop: Option<EventHandler<DragEvent>>,
    pub on_scroll: Option<EventHandler<ScrollEvent>>,
}
