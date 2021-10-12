use orbtk::prelude::themes::material_icons_font;
use orbtk::prelude::*;

use crate::main_state::Action;
use crate::MainState;

/// Generate a button for a numerical digit, attaching it to the given
/// row and column.
fn generate_digit_button(
    ctx: &mut BuildContext,
    id: Entity,
    display: char,
    primary: bool,
    column: usize,
    column_span: usize,
    row: usize,
) -> Entity {
    let style = if primary {
        "button_calculator_digit"
    } else {
        "button_calculator"
    };

    let button = Button::new()
        .style(style)
        .min_size(65.0, 55)
        .text(display.to_string())
        .font_size(26)
        .on_click(move |states, _| -> bool {
            states.send_message(Action::Digit(display), id);
            true
        })
        .attach(Grid::column(column))
        .attach(Grid::row(row))
        .attach(Grid::column_span(column_span));

    button.build(ctx)
}

/// Generates a button for an operator or function, attaching it to the given
/// row and column.
fn generate_operator_button(
    ctx: &mut BuildContext,
    id: Entity,
    display: char,
    primary: bool,
    column: usize,
    column_span: usize,
    row: usize,
) -> Entity {
    let style = if primary {
        "button_calculator_primary"
    } else {
        "button_calculator"
    };

    let button = Button::new()
        .style(style)
        .min_size(65.0, 55)
        .text(display.to_string())
        .font_size(26)
        .on_click(move |states, _| -> bool {
            let operator = match display {
                '÷' => '/',
                'x' => '*',
                '−' => '-',
                _ => display,
            };

            states.send_message(Action::Operator(operator), id);
            true
        })
        .attach(Grid::column(column))
        .attach(Grid::column_span(column_span))
        .attach(Grid::row(row));
    button.build(ctx)
}

widget!(
    MainView<MainState> {
        text: String
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").text("").child(
            Grid::new()
                .rows("100, *")
                .child(
                    Container::new()
                        .padding(8)
                        .style("header_area")
                        .attach(Grid::row(0))
                        .child(
                            Grid::new()
                                .child(
                                    ScrollViewer::new()
                                        .mode(("custom", "disabled"))
                                        .child(
                                            TextBlock::new()
                                                .width(0)
                                                .height(24)
                                                .text("0")
                                                .style("input")
                                                .id("input")
                                                .v_align("start")
                                                .build(ctx),
                                        )
                                        .build(ctx),
                                )
                                .child(
                                    TextBlock::new()
                                        .style("result")
                                        .text(id)
                                        .v_align("end")
                                        .h_align("end")
                                        .build(ctx),
                                )
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .child(
                    Container::new()
                        .padding(5)
                        .attach(Grid::row(1))
                        .child(
                            Grid::new()
                                .columns("72, 4, 72, 4, 72, 4, 72")
                                .rows("55, 4, 55, 4, 55, 4, 55, 4, 55")
                                // row 0
                                .child(generate_operator_button(ctx, id, 'C', true, 0, 3, 0))
                                .child(generate_operator_button(ctx, id, '\u{232B}', true, 4, 1, 0))
                                .child(
                                    Button::new()
                                        .style("button_calculator_primary")
                                        .min_size(65.0, 55)
                                        .icon(material_icons_font::MD_BACKSPACE)
                                        .on_click(move |states, _| -> bool {
                                            states.send_message(Action::Operator('\u{232B}'), id);
                                            true
                                        })
                                        .attach(Grid::column(4))
                                        .attach(Grid::column_span(1))
                                        .attach(Grid::row(0))
                                        .build(ctx),
                                )
                                .child(generate_operator_button(ctx, id, '÷', true, 6, 1, 0))
                                // row 2
                                .child(generate_digit_button(ctx, id, '7', true, 0, 1, 2))
                                .child(generate_digit_button(ctx, id, '8', true, 2, 1, 2))
                                .child(generate_digit_button(ctx, id, '9', true, 4, 1, 2))
                                .child(generate_operator_button(ctx, id, 'x', true, 6, 1, 2))
                                // row 4
                                .child(generate_digit_button(ctx, id, '4', true, 0, 1, 4))
                                .child(generate_digit_button(ctx, id, '5', true, 2, 1, 4))
                                .child(generate_digit_button(ctx, id, '6', true, 4, 1, 4))
                                .child(generate_operator_button(ctx, id, '−', true, 6, 1, 4))
                                // row 6
                                .child(generate_digit_button(ctx, id, '1', true, 0, 1, 6))
                                .child(generate_digit_button(ctx, id, '2', true, 2, 1, 6))
                                .child(generate_digit_button(ctx, id, '3', true, 4, 1, 6))
                                .child(generate_operator_button(ctx, id, '+', true, 6, 1, 6))
                                // row 8
                                .child(generate_digit_button(ctx, id, '±', false, 0, 1, 8))
                                .child(generate_digit_button(ctx, id, '0', true, 2, 1, 8))
                                .child(generate_operator_button(ctx, id, '.', false, 4, 1, 8))
                                .child(generate_operator_button(ctx, id, '=', true, 6, 1, 8))
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}
