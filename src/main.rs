mod ui;

use floem::{
    kurbo::Size,
    peniko::Color,
    views::{container, h_stack, label, v_stack, Decorators},
    window::WindowConfig,
    Application, IntoView,
};

use ui::{render_explorer, render_toolbar};

fn app_view() -> impl IntoView {
    v_stack((
        render_toolbar(),
        h_stack((
            render_explorer(),
            container(label(|| "")).style(|s| s.flex_grow(1.0).background(Color::rgb8(10, 14, 26))),
        )),
    ))
}

fn main() {
    let app = Application::new().window(
        |_| app_view(),
        Some(
            WindowConfig::default()
                .size(Size::new(1000.0, 600.0))
                .title("Code Flow"),
        ),
    );
    app.run();
}
