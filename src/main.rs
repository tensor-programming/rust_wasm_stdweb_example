#[macro_use]
extern crate stdweb;

use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{self, CanvasRenderingContext2d, IEventTarget, INonElementParentNode};

use stdweb::traits::IMouseEvent;

use stdweb::web::event::{
    IEvent, IKeyboardEvent, KeyDownEvent, KeyUpEvent, KeyboardLocation, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent,
};

fn main() {
    stdweb::initialize();

    let canvas: CanvasElement = web::document()
        .get_element_by_id("viewport")
        .unwrap()
        .try_into()
        .unwrap();

    let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

    draw_box(&ctx, "red", (20.0, 20.0), (150.0, 100.0));
    draw_box(&ctx, "green", (200.0, 20.0), (150.0, 150.0));
    draw_box(&ctx, "blue", (100.0, 20.0), (150.0, 150.0));

    web::window().add_event_listener(|event: KeyDownEvent| {
        if on_key(&event.key(), event.location(), true) {
            event.prevent_default();
        };
    });

    web::window().add_event_listener(|event: KeyUpEvent| {
        if on_key(&event.key(), event.location(), false) {
            event.prevent_default();
        }
    });

    web::window().add_event_listener(|event: MouseDownEvent| {
        if on_mouse_click(
            event.button(),
            true,
            (event.client_x() as f64, event.client_y() as f64),
        ) {
            event.prevent_default();
        }
    });

    web::window().add_event_listener(|event: MouseUpEvent| {
        if on_mouse_click(
            event.button(),
            false,
            (event.client_x() as f64, event.client_y() as f64),
        ) {
            event.prevent_default();
        }
    });

    web::window().add_event_listener(move |event: MouseMoveEvent| {
        if on_mouse_move((event.client_x() as f64, event.client_y() as f64)) {
            draw_box(
                &ctx,
                "orange",
                (event.client_x() as f64, event.client_y() as f64),
                (10.0, 10.0),
            );
        }
    });

    stdweb::event_loop();
}

fn draw_box(ctx: &CanvasRenderingContext2d, color: &str, position: (f64, f64), size: (f64, f64)) {
    ctx.set_fill_style_color(color);
    ctx.fill_rect(position.0, position.1, size.0, size.1);
}

fn on_key(key: &str, location: KeyboardLocation, is_pressed: bool) -> bool {
    let location = format!("{:?}", location);
    console!(
        log,
        "Key: ",
        key,
        ", location: ",
        location,
        ", pressed: ",
        is_pressed
    );
    true
}

fn on_mouse_click(btn: MouseButton, is_pressed: bool, position: (f64, f64)) -> bool {
    let btn = format!("{:?}", btn);
    console!(
        log,
        "Mouse Position: (",
        position.0,
        ", ",
        position.1,
        ") Mouse Button: ",
        btn,
        " pressed: ",
        is_pressed
    );
    true
}

fn on_mouse_move(position: (f64, f64)) -> bool {
    console!(log, "Mouse Position: (", position.0, ", ", position.1, ")");
    true
}
