use dioxus::prelude::*;

#[component]
pub(crate) fn PlotEventWrapper(
    size: (u32, u32),
    draggable: Option<bool>,
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
    children: Element,
) -> Element {
    rsx!(div {
        onclick: move |e| {
            if let Some(callback) = on_click.as_ref() {
                callback.call(e)
            };
        },
        ondoubleclick: move |e| {
            if let Some(callback) = on_doubleclick.as_ref() {
                callback.call(e)
            };
        },
        onmousemove: move |e| {
            if let Some(callback) = on_mousemove.as_ref() {
                callback.call(e)
            };
        },
        onmousedown: move |e| {
            if let Some(callback) = on_mousedown.as_ref() {
                callback.call(e)
            };
        },
        onmouseup: move |e| {
            if let Some(callback) = on_mouseup.as_ref() {
                callback.call(e)
            };
        },
        onmouseout: move |e| {
            if let Some(callback) = on_mouseout.as_ref() {
                callback.call(e)
            };
        },
        onmouseover: move |e| {
            if let Some(callback) = on_mouseover.as_ref() {
                callback.call(e)
            };
        },
        onwheel: move |e| {
            if let Some(callback) = on_wheel.as_ref() {
                callback.call(e)
            };
        },
        ondrag: move |e| {
            if let Some(callback) = on_drag.as_ref() {
                callback.call(e)
            };
        },
        ondragend: move |e| {
            if let Some(callback) = on_dragend.as_ref() {
                callback.call(e)
            };
        },
        ondragenter: move |e| {
            if let Some(callback) = on_dragenter.as_ref() {
                callback.call(e)
            };
        },
        ondragleave: move |e| {
            if let Some(callback) = on_dragleave.as_ref() {
                callback.call(e)
            };
        },
        ondragover: move |e| {
            if let Some(callback) = on_dragover.as_ref() {
                callback.call(e)
            };
        },
        ondragstart: move |e| {
            if let Some(callback) = on_dragstart.as_ref() {
                callback.call(e)
            };
        },
        ondrop: move |e| {
            if let Some(callback) = on_drop.as_ref() {
                callback.call(e)
            };
        },
        onscroll: move |e| {
            if let Some(callback) = on_scroll.as_ref() {
                callback.call(e)
            };
        },
        draggable: draggable.unwrap_or(false),
        {children}
    })
}
