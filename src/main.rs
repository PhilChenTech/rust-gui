use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};
use druid::widget::{Label, Flex, TextBox, Button};
use meval::eval_str; // Import eval_str from meval

#[derive(Clone, Data, Lens)]
struct AppState {
    display: String,
    result: String,
}

fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("Simple Calculator")
        .window_size((300.0, 400.0));

    let initial_state = AppState {
        display: "".into(),
        result: "".into(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn ui_builder() -> impl Widget<AppState> {
    let display = TextBox::new()
        .with_placeholder("0")
        .lens(AppState::display)
        .fix_height(50.0);

    let result = Label::new(|data: &AppState, _env: &_| format!("{}", data.result))
        .fix_height(30.0);

    let button_grid = Flex::column()
        .with_child(Flex::row()
            .with_child(button("7"))
            .with_child(button("8"))
            .with_child(button("9"))
            .with_child(button("/"))
            .padding(5.0)
        )
        .with_child(Flex::row()
            .with_child(button("4"))
            .with_child(button("5"))
            .with_child(button("6"))
            .with_child(button("*"))
            .padding(5.0)
        )
        .with_child(Flex::row()
            .with_child(button("1"))
            .with_child(button("2"))
            .with_child(button("3"))
            .with_child(button("-"))
            .padding(5.0)
        )
        .with_child(Flex::row()
            .with_child(button("C"))
            .with_child(button("0"))
            .with_child(button("="))
            .with_child(button("+"))
            .padding(5.0)
        )
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start);

    Flex::column()
        .with_child(display)
        .with_child(result)
        .with_child(button_grid)
        .padding(10.0)
}

fn button(label: &'static str) -> impl Widget<AppState> {
    Button::new(label).on_click(move |ctx, data: &mut AppState, _env| {
        match label {
            "C" => {
                data.display.clear();
                data.result.clear();
            }
            "=" => {
                // Evaluate the expression and update the result
                let expr = &data.display;
                let result = evaluate_expression(expr);
                data.result = result.to_string();
                data.display.clear();
            }
            _ => {
                data.display.push_str(label);
            }
        }
    })
        .fix_size(50.0, 50.0)
}

fn evaluate_expression(expression: &str) -> f64 {
    // Use meval to evaluate the expression
    eval_str(expression).unwrap_or_else(|_| 0.0)
}
