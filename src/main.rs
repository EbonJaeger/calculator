use calc::eval;
use decimal::d128;
use iced::Element;
use iced::HorizontalAlignment;
use iced::Length;
use iced::Sandbox;
use iced::Settings;
use iced::Text;
use iced::VerticalAlignment;

#[derive(Default)]
struct Calculator {
    tokens: String,
    has_err: bool,
    result: Option<d128>,

    // WIdgets
    button_1: iced::button::State,
    button_2: iced::button::State,
    button_3: iced::button::State,
    button_4: iced::button::State,
    button_5: iced::button::State,
    button_6: iced::button::State,
    button_7: iced::button::State,
    button_8: iced::button::State,
    button_9: iced::button::State,
    button_0: iced::button::State,
    button_dot: iced::button::State,
    button_equal: iced::button::State,
    button_plus: iced::button::State,
    button_minus: iced::button::State,
    button_multiply: iced::button::State,
    button_divide: iced::button::State,
    button_del: iced::button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum CalculatorMessage {
    ClearPressed,
    DeletePressed,
    NumberPressed(&'static str),
    OperatorPressed(&'static str),
    ResultPressed,
}

impl Sandbox for Calculator {
    type Message = CalculatorMessage;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Calculator")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            CalculatorMessage::ClearPressed => self.tokens = String::new(),
            CalculatorMessage::DeletePressed => {
                self.has_err = false;

                // Clear a result if we have one stored
                if self.result.is_some() {
                    self.result = None;
                    self.tokens = String::new();
                } else {
                    self.tokens.pop();
                }
            }
            CalculatorMessage::NumberPressed(num) => {
                self.has_err = false;

                // If we have a result already, clear it and start new
                if self.result.is_some() {
                    self.result = None;
                    self.tokens = num.to_string();
                } else {
                    self.tokens.push_str(num);
                }
            }
            CalculatorMessage::OperatorPressed(operator) => {
                // Only push an operator if there is actually a number there already
                if !self.tokens.is_empty() {
                    // Reset the result so the display and tokens don't get reset
                    self.result = None;
                    self.tokens.push_str(operator);
                }
            }
            CalculatorMessage::ResultPressed => {
                let result = eval(&self.tokens);
                self.has_err = result.is_err();

                match result {
                    Ok(val) => {
                        // We have a result, set it as the current token
                        let num = val.as_float().unwrap_or(d128!(NaN));
                        self.tokens = num.to_string();
                    }
                    Err(e) => {
                        eprintln!("calc error: {}", e);
                    }
                };
            }
        }
    }

    fn view(&mut self) -> Element<CalculatorMessage> {
        let display_text = match self.has_err {
            false => self.tokens.clone(),
            true => String::from("INVALID"),
        };
        let display = iced::Container::new(
            iced::Text::new(display_text)
                .horizontal_alignment(HorizontalAlignment::Right)
                .vertical_alignment(VerticalAlignment::Center)
                .size(30),
        )
        .height(Length::Units(70))
        .width(Length::Fill)
        .padding(5);

        let button_grid = iced::Column::new()
            .padding(5)
            .spacing(5)
            .push(
                iced::Row::new()
                    .spacing(5)
                    .push(
                        iced::Button::new(
                            &mut self.button_1,
                            Text::new("1")
                                .vertical_alignment(VerticalAlignment::Center)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .size(25),
                        )
                        .on_press(CalculatorMessage::NumberPressed("1"))
                        .width(Length::Units(65))
                        .height(Length::Units(55)),
                    )
                    .push(
                        iced::Button::new(
                            &mut self.button_2,
                            Text::new("2")
                                .vertical_alignment(VerticalAlignment::Center)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .size(25),
                        )
                        .on_press(CalculatorMessage::NumberPressed("2"))
                        .width(Length::Units(65))
                        .height(Length::Units(55)),
                    )
                    .push(
                        iced::Button::new(
                            &mut self.button_3,
                            Text::new("3")
                                .vertical_alignment(VerticalAlignment::Center)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .size(25),
                        )
                        .on_press(CalculatorMessage::NumberPressed("3"))
                        .width(Length::Units(65))
                        .height(Length::Units(55)),
                    ),
            )
            .push(
                iced::Row::new()
                    .spacing(5)
                    .push(
                        iced::Button::new(
                            &mut self.button_4,
                            Text::new("4")
                                .vertical_alignment(VerticalAlignment::Center)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .size(25),
                        )
                        .on_press(CalculatorMessage::NumberPressed("4"))
                        .width(Length::Units(65))
                        .height(Length::Units(55)),
                    )
                    .push(
                        iced::Button::new(
                            &mut self.button_5,
                            Text::new("5")
                                .vertical_alignment(VerticalAlignment::Center)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .size(25),
                        )
                        .on_press(CalculatorMessage::NumberPressed("5"))
                        .width(Length::Units(65))
                        .height(Length::Units(55)),
                    )
                    .push(
                        iced::Button::new(
                            &mut self.button_6,
                            Text::new("6")
                                .vertical_alignment(VerticalAlignment::Center)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .size(25),
                        )
                        .on_press(CalculatorMessage::NumberPressed("6"))
                        .width(Length::Units(65))
                        .height(Length::Units(55)),
                    ),
            )
            .push(
                iced::Row::new()
                    .spacing(5)
                    .push(
                        iced::Button::new(
                            &mut self.button_7,
                            Text::new("7")
                                .vertical_alignment(VerticalAlignment::Center)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .size(25),
                        )
                        .on_press(CalculatorMessage::NumberPressed("7"))
                        .width(Length::Units(65))
                        .height(Length::Units(55)),
                    )
                    .push(
                        iced::Button::new(
                            &mut self.button_8,
                            Text::new("8")
                                .vertical_alignment(VerticalAlignment::Center)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .size(25),
                        )
                        .on_press(CalculatorMessage::NumberPressed("8"))
                        .width(Length::Units(65))
                        .height(Length::Units(55)),
                    )
                    .push(
                        iced::Button::new(
                            &mut self.button_9,
                            Text::new("9")
                                .vertical_alignment(VerticalAlignment::Center)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .size(25),
                        )
                        .on_press(CalculatorMessage::NumberPressed("9"))
                        .width(Length::Units(65))
                        .height(Length::Units(55)),
                    ),
            )
            .push(
                iced::Row::new()
                    .spacing(5)
                    .push(
                        iced::Button::new(
                            &mut self.button_dot,
                            Text::new(".")
                                .vertical_alignment(VerticalAlignment::Center)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .size(25),
                        )
                        .on_press(CalculatorMessage::NumberPressed("."))
                        .width(Length::Units(65))
                        .height(Length::Units(55)),
                    )
                    .push(
                        iced::Button::new(
                            &mut self.button_0,
                            Text::new("0")
                                .vertical_alignment(VerticalAlignment::Center)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .size(25),
                        )
                        .on_press(CalculatorMessage::NumberPressed("0"))
                        .width(Length::Units(65))
                        .height(Length::Units(55)),
                    )
                    .push(
                        iced::Button::new(
                            &mut self.button_equal,
                            Text::new("=")
                                .vertical_alignment(VerticalAlignment::Center)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .size(25),
                        )
                        .on_press(CalculatorMessage::ResultPressed)
                        .width(Length::Units(65))
                        .height(Length::Units(55)),
                    ),
            );

        let operator_grid = iced::Column::new()
            .padding(5)
            .spacing(5)
            .push(
                iced::Button::new(
                    &mut self.button_del,
                    Text::new("⌫")
                        .vertical_alignment(VerticalAlignment::Center)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .size(25),
                )
                .on_press(CalculatorMessage::DeletePressed)
                .width(Length::Units(65))
                .height(Length::Units(43)),
            )
            .push(
                iced::Button::new(
                    &mut self.button_divide,
                    Text::new("÷")
                        .vertical_alignment(VerticalAlignment::Center)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .size(25),
                )
                .on_press(CalculatorMessage::OperatorPressed("/"))
                .width(Length::Units(65))
                .height(Length::Units(43)),
            )
            .push(
                iced::Button::new(
                    &mut self.button_multiply,
                    Text::new("x")
                        .vertical_alignment(VerticalAlignment::Center)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .size(25),
                )
                .on_press(CalculatorMessage::OperatorPressed("*"))
                .width(Length::Units(65))
                .height(Length::Units(43)),
            )
            .push(
                iced::Button::new(
                    &mut self.button_minus,
                    Text::new("-")
                        .vertical_alignment(VerticalAlignment::Center)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .size(25),
                )
                .on_press(CalculatorMessage::OperatorPressed("-"))
                .width(Length::Units(65))
                .height(Length::Units(43)),
            )
            .push(
                iced::Button::new(
                    &mut self.button_plus,
                    Text::new("+")
                        .vertical_alignment(VerticalAlignment::Center)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .size(25),
                )
                .on_press(CalculatorMessage::OperatorPressed("+"))
                .width(Length::Units(65))
                .height(Length::Units(43)),
            );

        let button_container =
            iced::Row::with_children(vec![button_grid.into(), operator_grid.into()])
                .width(Length::Fill);

        iced::Column::with_children(vec![display.into(), button_container.into()])
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.min_size = Some((325, 500));
    settings.window.size = (325, 500);

    Calculator::run(settings)
}
