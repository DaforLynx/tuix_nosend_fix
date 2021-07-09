/*
use tuix::*;

fn main() {
    let app = Application::new(|state, window| {
        window
            .set_title("Flex Grow")
            .set_background_color(state, Color::white())
            .set_flex_direction(state, FlexDirection::Row);

        Label::new("Flex Basis: 0px, Flex Grow: 1.0").build(state, window.entity(), |builder| {
            builder
                .set_flex_grow(1.0)
                .set_background_color(Color::rgb(50, 50, 120))
        });

        Label::new("Flex Basis: 200px, Flex Grow: 0.0").build(state, window.entity(), |builder| {
            builder
                .set_flex_basis(Units::Pixels(200.0))
                .set_background_color(Color::rgb(120, 50, 50))
        });

        Label::new("Flex Basis: 100px, Flex Grow: 2.0").build(state, window.entity(), |builder| {
            builder
                .set_flex_grow(2.0)
                .set_flex_basis(Units::Pixels(100.0))
                .set_background_color(Color::rgb(50, 120, 50))
        });

        Label::new("Flex Basis: 100px, Flex Grow: 0.0").build(state, window.entity(), |builder| {
            builder
                .set_flex_basis(Units::Pixels(200.0))
                .set_background_color(Color::rgb(120, 120, 50))
        });
    });
    app.run();
}
*/

fn main() {
    
}