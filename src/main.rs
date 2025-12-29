use floem::{
    Application, IntoView,
    kurbo::{Point, Size},
    peniko::{self, Color, Gradient, GradientKind},
    prelude::*,
    taffy::{AlignItems, JustifyContent},
    text::Weight,
    views::{Button, Decorators, Stack, h_stack, label, stack, text_input, v_stack, container},
    window::WindowConfig,
};

fn render_toolbar() -> Stack {
    let text = RwSignal::new(String::new());

    let toolbar_gradient = Gradient {
        kind: GradientKind::Linear {
            start: Point::new(0.0, 0.0),
            end: Point::new(0.0, 60.0),
        },
        stops: vec![
            peniko::ColorStop {
                offset: 0.0,
                color: Color::rgb8(26, 31, 58),
            },
            peniko::ColorStop {
                offset: 1.0,
                color: Color::rgb8(21, 25, 41),
            },
        ]
        .into(),
        extend: peniko::Extend::Pad,
    };

    h_stack((
        label(|| "⚡ CodeFlow").style(|s| {
            s.font_size(24.0)
                .font_weight(Weight::BOLD)
                .color(Color::rgb8(102, 126, 234))
        }),
        h_stack((
            h_stack((
                Button::new("File").style(|s| {
                    s.border_radius(8.0)
                        .padding(8.0)
                        .padding_left(16.0)
                        .padding_right(16.0)
                        .font_size(14.0)
                        .color(Color::rgba8(168, 179, 207, 255))
                        .background(Color::rgba8(255, 255, 255, 13))
                        .border(0.0)
                }),
                Button::new("Edit").style(|s| {
                    s.border_radius(8.0)
                        .padding(8.0)
                        .padding_left(16.0)
                        .padding_right(16.0)
                        .font_size(14.0)
                        .color(Color::rgba8(168, 179, 207, 255))
                        .background(Color::rgba8(255, 255, 255, 13))
                        .border(0.0)
                }),
                Button::new("View").style(|s| {
                    s.border_radius(8.0)
                        .padding(8.0)
                        .padding_left(16.0)
                        .padding_right(16.0)
                        .font_size(14.0)
                        .color(Color::rgba8(168, 179, 207, 255))
                        .background(Color::rgba8(255, 255, 255, 13))
                        .border(0.0)
                }),
                Button::new("Tools").style(|s| {
                    s.border_radius(8.0)
                        .padding(8.0)
                        .padding_left(16.0)
                        .padding_right(16.0)
                        .font_size(14.0)
                        .color(Color::rgba8(168, 179, 207, 255))
                        .background(Color::rgba8(255, 255, 255, 13))
                        .border(0.0)
                }),
            ))
            .style(|s| s.gap(8.0)),
            h_stack((
                text_input(text).placeholder("Search files...").style(|s| {
                    s.border_radius(10.0)
                        .padding(8.0)
                        .padding_left(16.0)
                        .padding_right(16.0)
                        .font_size(14.0)
                        .width(250.0)
                        .color(Color::rgba8(168, 179, 207, 255))
                        .background(Color::rgba8(255, 255, 255, 20))
                        .border_color(Color::rgba8(255, 255, 255, 26))
                        .border(1.0)
                }),
                Button::new("Settings").style(|s| {
                    s.border_radius(10.0)
                        .padding(8.0)
                        .padding_left(20.0)
                        .padding_right(20.0)
                        .font_size(14.0)
                        .font_weight(Weight::SEMIBOLD)
                        .color(Color::WHITE)
                        .background(Color::rgba8(255, 255, 255, 26))
                        .border(0.0)
                }),
                Button::new("Run Code").style(|s| {
                    s.border_radius(10.0)
                        .padding(8.0)
                        .padding_left(20.0)
                        .padding_right(20.0)
                        .font_size(14.0)
                        .font_weight(Weight::SEMIBOLD)
                        .color(Color::WHITE)
                        .background(Color::rgb8(102, 126, 234))
                        .border(0.0)
                }),
            ))
            .style(|s| s.gap(16.0).align_items(AlignItems::Center)),
        ))
        .style(|s| {
            s.width_full()
                .justify_content(JustifyContent::SpaceBetween)
                .gap(312)
        }),
    ))
    .style(|s| {
        s.width_full()
            .gap(24.0)
            .align_items(AlignItems::Center)
            .padding_left(24.0)
            .padding_right(24.0)
    })
    .style(move |s| {
        s.width_full()
            .height(60.0)
            .background(toolbar_gradient.clone())
            .border_bottom(2.0)
            .border_color(Color::rgb8(102, 126, 234))
    })
}

fn render_explorer() -> Stack {
    v_stack((
        h_stack((
            container(label(|| "Files").style(|s| {
                s.font_size(13.0)
                    .font_weight(Weight::SEMIBOLD)
                    .color(Color::rgb8(102, 126, 234))
            }))
            .style(|s| {
                s.flex_grow(1.0)
                    .padding(14.0)
                    .border_bottom(3.0)
                    .border_color(Color::rgb8(102, 126, 234))
            }),
            container(label(|| "Search").style(|s| {
                s.font_size(13.0)
                    .font_weight(Weight::SEMIBOLD)
                    .color(Color::rgb8(107, 122, 153))
            }))
            .style(|s| s.flex_grow(1.0).padding(14.0)),
            container(label(|| "Git").style(|s| {
                s.font_size(13.0)
                    .font_weight(Weight::SEMIBOLD)
                    .color(Color::rgb8(107, 122, 153))
            }))
            .style(|s| s.flex_grow(1.0).padding(14.0)),
        ))
        .style(|s| {
            s.background(Color::rgb8(21, 25, 41))
                .border_bottom(1.0)
                .border_color(Color::rgb8(26, 31, 58))
        }),
    ))
    .style(|s| {
        s.width(280.0)
            .height_full()
            .background(Color::rgb8(15, 19, 32))
            .border_right(1.0)
            .border_color(Color::rgb8(26, 31, 58))
    })
}

fn app_view() -> impl IntoView {
    v_stack((
        render_toolbar(),
        h_stack((
            render_explorer(),
            container(label(|| "")).style(|s| {
                s.flex_grow(1.0)
                    .background(Color::rgb8(10, 14, 26))
            }),
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
