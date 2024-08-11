use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};
use druid::widget::{Label, Flex};

#[derive(Clone, Data, Lens)]
struct AppState {
    name: String,
}

fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("Hello, Druid!")
        .window_size((400.0, 300.0));

    let initial_state = AppState {
        name: "World".into(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn ui_builder() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new(|data: &AppState, _env: &_| format!("Hello, {}!", data.name)))
        .padding(10.0)
}
