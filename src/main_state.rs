use calc::eval;
use decimal::d128;
use orbtk::prelude::*;

use crate::MainView;

#[derive(Debug, Copy, Clone)]
pub enum Action {
    Digit(char),
    Operator(char),
}

#[derive(Default, AsAny)]
pub struct MainState {
    input: String,
    result: String,
}

impl State for MainState {
    fn messages(
        &mut self,
        mut messages: MessageReader,
        _registry: &mut Registry,
        ctx: &mut Context,
    ) {
        for message in messages.read::<Action>() {
            match message {
                Action::Digit(digit) => {
                    if !self.result.is_empty() {
                        self.clear_all(ctx);
                    }

                    self.input.push(digit);
                    TextBlock::text_mut(&mut ctx.child("input")).push(digit);
                }
                Action::Operator(operator) => match operator {
                    'C' => {
                        self.clear_all(ctx);
                    }
                    '\u{232B}' => {
                        self.input.pop();
                        self.result.clear();
                        MainView::text_mut(&mut ctx.widget()).clear();
                        TextBlock::text_mut(&mut ctx.child("input")).pop();
                    }
                    '=' => {
                        // Calculate the result, and set the display text
                        self.calculate(ctx);
                    }
                    _ => {
                        if self.input.is_empty() {
                            return;
                        }

                        // If we have a stored result, set that as the first element of the new equation
                        if !self.result.is_empty() {
                            MainView::text_mut(&mut ctx.widget()).clear();
                            TextBlock::text_set(&mut ctx.child("input"), self.result.clone());
                            self.result.clear();
                        }

                        self.input.push(operator);
                        TextBlock::text_mut(&mut ctx.child("input")).push(operator);
                    }
                },
            }
        }
    }
}

impl MainState {
    /// Clears the input and result displays as well as their state tracking.
    fn clear_all(&mut self, ctx: &mut Context) {
        self.input.clear();
        self.result.clear();
        MainView::text_mut(&mut ctx.widget()).clear();
        TextBlock::text_mut(&mut ctx.child("input")).clear();
    }

    /// Computes the result of the current equation using Redox OS's calc
    /// crate. If the equation can be solved, shove the result into the
    /// tracked input in case the user wants to use the result in another
    /// calculation.
    ///
    /// If the calulation results in an error, the error will be printed
    /// to the console (if possible) and the display text will be set to
    /// "INVALID".
    fn calculate(&mut self, ctx: &mut Context) {
        let result = eval(&self.input);
        match result {
            Ok(val) => {
                let num = val.as_float().unwrap_or(d128!(NaN));

                self.input = num.to_string();
                self.result = num.to_string();

                MainView::text_set(&mut ctx.widget(), format!("{:.18}", num));
            }
            Err(e) => {
                eprintln!("calc error: {}", e);
                MainView::text_set(&mut ctx.widget(), "INVALID");
            }
        }
    }
}