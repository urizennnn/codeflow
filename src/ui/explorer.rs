use floem::{
    peniko::Color,
    text::Weight,
    views::{container, h_stack, label, v_stack, Decorators, Stack},
};

pub fn render_explorer() -> Stack {
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
        container(label(|| "")).style(|s| s.padding(8.0)),
    ))
    .style(|s| {
        s.width(280.0)
            .background(Color::rgb8(15, 19, 32))
            .border_right(1.0)
            .border_color(Color::rgb8(26, 31, 58))
    })
}
