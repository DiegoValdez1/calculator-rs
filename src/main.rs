#![allow(unused)] // shutup rust

use druid::{
    widget::{
        Align, Button, Container, CrossAxisAlignment, Flex, FlexParams, Label, List,
        MainAxisAlignment, Split,
    },
    AppLauncher, Application, Data, Lens, Widget, WidgetExt, WindowDesc,
};
use shunting::solve;

mod shunting;

macro_rules! gridbutton {
    ($flex:expr, clear) => {
        $flex.add_flex_child(
            Button::new("C")
                .on_click(|_ctx, data: &mut String, _env| data.clear())
                .expand(),
            1.0,
        )
    };
    ($flex:expr, solve) => {
        $flex.add_flex_child(
            Button::new("=")
                .on_click(|_ctx, data: &mut String, _env| {
                    match solve(data) {
                        Ok(x) => {
                            data.clear();
                            data.push_str(&x.to_string());
                        },
                        Err(_) => {
                            data.clear();
                            data.push_str("ERROR");
                        }
                    }
                })
                .expand(),
            1.0,
        )
    };
    ($flex:expr, del) => {
        $flex.add_flex_child(
            Button::new("\u{232B}")
                .on_click(|_ctx, data: &mut String, _env| {if data.len() != 0 {data.remove(data.len()-1);}})
                .expand(),
            1.0,
        )
    };
    ($flex:expr, $display:expr) => {
        $flex.add_flex_child(
            Button::new($display)
                .on_click(|_ctx, data: &mut String, _env| data.push_str($display))
                .expand(),
            1.0,
        )
    };
    ($flex:expr, $display:expr, $internal:expr) => {
        $flex.add_flex_child(
            Button::new($display)
                .on_click(|_ctx, data: &mut String, _env| data.push_str($internal))
                .expand(),
            1.0,
        )
    };
}

macro_rules! gridspacer {
    ($flex:expr) => {
        $flex.add_flex_spacer(1.0);
    };
}

#[derive(Debug, Clone, Data, Lens)]
struct State {
    display: String,
}

fn build_keypad() -> impl Widget<String> {
    let mut first = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Fill)
        .must_fill_main_axis(true);
    let mut second = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Fill)
        .must_fill_main_axis(true);
    let mut third = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Fill)
        .must_fill_main_axis(true);
    let mut fourth = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Fill)
        .must_fill_main_axis(true);

    // one day I will make a struct or something that does this automatically
    // some day..... but this is fine for now
    gridspacer!(first);
    gridbutton!(first, "(");
    gridbutton!(first, "1");
    gridbutton!(first, "4");
    gridbutton!(first, "7");
    gridbutton!(first, ".");

    gridspacer!(second);
    gridbutton!(second, ")");
    gridbutton!(second, "2");
    gridbutton!(second, "5");
    gridbutton!(second, "8");
    gridbutton!(second, "0");

    gridbutton!(third, clear);
    gridbutton!(third, "^");
    gridbutton!(third, "3");
    gridbutton!(third, "6");
    gridbutton!(third, "9");
    gridbutton!(third, "(-)", "–");

    gridbutton!(fourth, del);
    gridbutton!(fourth, "+");
    gridbutton!(fourth, "-");
    gridbutton!(fourth, "*");
    gridbutton!(fourth, "/");
    gridbutton!(fourth, solve);

    Flex::row()
        .must_fill_main_axis(true)
        .with_flex_child(first, 1.0)
        .with_flex_child(second, 1.0)
        .with_flex_child(third, 1.0)
        .with_flex_child(fourth, 1.0)
}

fn build_root() -> impl Widget<String> {
    Split::rows(
        Label::dynamic(|data: &String, _env| data.clone()).center(),
        build_keypad(),
    )
    .split_point(0.25)
}

fn main() {
    let window = WindowDesc::new(build_root().lens(State::display))
        .window_size((300.0, 400.0))
        .title("Calculator");

    let start_state = State {
        display: "".to_string(),
    };

    AppLauncher::with_window(window)
        .launch(start_state)
        .expect("Unable to launch GUI");
}
