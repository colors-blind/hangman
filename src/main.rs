mod word;
mod file_reader;

use eframe::egui;
use word::{Word, CheckLetter, CheckComplete};
use file_reader::select_word;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Hangman",
        options,
        Box::new(|_cc| Ok(Box::new(HangmanApp::new()))),
    )
}

struct HangmanApp {
    word: Word,
    wrong_count: usize,
    max_wrong: usize,
    guessed_letters: Vec<char>,
    input_letter: String,
    message: String,
    game_over: bool,
    won: bool,
}

impl HangmanApp {
    fn new() -> Self {
        let answer = select_word();
        let length = answer.len();
        Self {
            word: Word {
                length,
                representation: "_".repeat(length),
                answer,
                correct_count: 0,
            },
            wrong_count: 0,
            max_wrong: 10,
            guessed_letters: Vec::new(),
            input_letter: String::new(),
            message: String::new(),
            game_over: false,
            won: false,
        }
    }

    fn reset(&mut self) {
        let answer = select_word();
        let length = answer.len();
        self.word = Word {
            length,
            representation: "_".repeat(length),
            answer,
            correct_count: 0,
        };
        self.wrong_count = 0;
        self.guessed_letters.clear();
        self.input_letter.clear();
        self.message.clear();
        self.game_over = false;
        self.won = false;
    }

    fn guess(&mut self, c: char) {
        let c = c.to_ascii_lowercase();
        if !c.is_ascii_alphabetic() || self.game_over {
            return;
        }
        if self.guessed_letters.contains(&c) {
            self.message = format!("'{}' 已经猜过了", c);
            return;
        }
        self.guessed_letters.push(c);
        if self.word.check_for_letter(c) {
            self.message = format!("正确！包含字母 '{}'", c);
            if self.word.check_complete() {
                self.game_over = true;
                self.won = true;
                self.message = format!("恭喜！答案是 \"{}\"", self.word.answer);
            }
        } else {
            self.wrong_count += 1;
            if self.wrong_count >= self.max_wrong {
                self.game_over = true;
                self.message = format!("游戏结束！答案是 \"{}\"", self.word.answer);
            } else {
                self.message = format!("错误！剩余机会: {}", self.max_wrong - self.wrong_count);
            }
        }
    }
}

impl eframe::App for HangmanApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Hangman 猜单词游戏");
                ui.add_space(10.0);

                // 绘制火柴人
                let (response, painter) =
                    ui.allocate_painter(egui::vec2(200.0, 200.0), egui::Sense::hover());
                let rect = response.rect;
                let stroke = egui::Stroke::new(2.0, egui::Color32::WHITE);
                let base_x = rect.left() + 40.0;
                let base_y = rect.bottom() - 10.0;

                // 底座
                painter.line_segment(
                    [egui::pos2(base_x - 30.0, base_y), egui::pos2(base_x + 60.0, base_y)],
                    stroke,
                );
                // 立柱
                painter.line_segment(
                    [egui::pos2(base_x, base_y), egui::pos2(base_x, rect.top() + 10.0)],
                    stroke,
                );
                // 横梁
                painter.line_segment(
                    [egui::pos2(base_x, rect.top() + 10.0), egui::pos2(base_x + 80.0, rect.top() + 10.0)],
                    stroke,
                );
                // 绳子
                let rope_x = base_x + 80.0;
                let rope_top = rect.top() + 10.0;
                painter.line_segment(
                    [egui::pos2(rope_x, rope_top), egui::pos2(rope_x, rope_top + 20.0)],
                    stroke,
                );

                let body_stroke = egui::Stroke::new(2.0, egui::Color32::LIGHT_RED);
                let head_center = egui::pos2(rope_x, rope_top + 35.0);

                if self.wrong_count >= 1 {
                    // 头
                    painter.circle_stroke(head_center, 15.0, body_stroke);
                }
                if self.wrong_count >= 2 {
                    // 脖子
                    painter.line_segment(
                        [egui::pos2(rope_x, rope_top + 50.0), egui::pos2(rope_x, rope_top + 60.0)],
                        body_stroke,
                    );
                }
                if self.wrong_count >= 3 {
                    // 躯干
                    painter.line_segment(
                        [egui::pos2(rope_x, rope_top + 60.0), egui::pos2(rope_x, rope_top + 100.0)],
                        body_stroke,
                    );
                }
                if self.wrong_count >= 4 {
                    // 左臂
                    painter.line_segment(
                        [egui::pos2(rope_x, rope_top + 70.0), egui::pos2(rope_x - 25.0, rope_top + 90.0)],
                        body_stroke,
                    );
                }
                if self.wrong_count >= 5 {
                    // 右臂
                    painter.line_segment(
                        [egui::pos2(rope_x, rope_top + 70.0), egui::pos2(rope_x + 25.0, rope_top + 90.0)],
                        body_stroke,
                    );
                }
                if self.wrong_count >= 6 {
                    // 左腿
                    painter.line_segment(
                        [egui::pos2(rope_x, rope_top + 100.0), egui::pos2(rope_x - 20.0, rope_top + 130.0)],
                        body_stroke,
                    );
                }
                if self.wrong_count >= 7 {
                    // 右腿
                    painter.line_segment(
                        [egui::pos2(rope_x, rope_top + 100.0), egui::pos2(rope_x + 20.0, rope_top + 130.0)],
                        body_stroke,
                    );
                }
                if self.wrong_count >= 8 {
                    // 左脚
                    painter.line_segment(
                        [egui::pos2(rope_x - 20.0, rope_top + 130.0), egui::pos2(rope_x - 30.0, rope_top + 130.0)],
                        body_stroke,
                    );
                }
                if self.wrong_count >= 9 {
                    // 右脚
                    painter.line_segment(
                        [egui::pos2(rope_x + 20.0, rope_top + 130.0), egui::pos2(rope_x + 30.0, rope_top + 130.0)],
                        body_stroke,
                    );
                }
                if self.wrong_count >= 10 {
                    // 眼睛 X_X
                    let eye_y = head_center.y - 3.0;
                    let left_eye_x = head_center.x - 5.0;
                    let right_eye_x = head_center.x + 5.0;
                    let s = 3.0;
                    let eye_stroke = egui::Stroke::new(1.5, egui::Color32::LIGHT_RED);
                    painter.line_segment([egui::pos2(left_eye_x - s, eye_y - s), egui::pos2(left_eye_x + s, eye_y + s)], eye_stroke);
                    painter.line_segment([egui::pos2(left_eye_x + s, eye_y - s), egui::pos2(left_eye_x - s, eye_y + s)], eye_stroke);
                    painter.line_segment([egui::pos2(right_eye_x - s, eye_y - s), egui::pos2(right_eye_x + s, eye_y + s)], eye_stroke);
                    painter.line_segment([egui::pos2(right_eye_x + s, eye_y - s), egui::pos2(right_eye_x - s, eye_y + s)], eye_stroke);
                }

                ui.add_space(10.0);

                // 显示单词（带空格分隔下划线）
                let display: String = self.word.representation.chars()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                ui.label(
                    egui::RichText::new(&display)
                        .size(28.0)
                        .monospace()
                );

                ui.add_space(10.0);

                // 已猜字母
                let guessed: String = self.guessed_letters.iter().collect();
                ui.label(format!("已猜字母: {}", guessed));

                ui.add_space(5.0);
                ui.label(format!("错误次数: {} / {}", self.wrong_count, self.max_wrong));

                ui.add_space(10.0);

                // 消息
                if !self.message.is_empty() {
                    let color = if self.won {
                        egui::Color32::GREEN
                    } else if self.game_over {
                        egui::Color32::RED
                    } else {
                        egui::Color32::YELLOW
                    };
                    ui.label(egui::RichText::new(&self.message).color(color).size(16.0));
                }

                ui.add_space(10.0);

                if !self.game_over {
                    // 键盘按钮布局
                    let rows = [
                        "qwertyuiop",
                        "asdfghjkl",
                        "zxcvbnm",
                    ];
                    for row in rows {
                        ui.horizontal(|ui| {
                            for c in row.chars() {
                                let already_guessed = self.guessed_letters.contains(&c);
                                let btn = ui.add_enabled(
                                    !already_guessed,
                                    egui::Button::new(c.to_string().to_uppercase()).min_size(egui::vec2(30.0, 30.0)),
                                );
                                if btn.clicked() {
                                    self.guess(c);
                                }
                            }
                        });
                    }

                    // 也支持键盘输入
                    for event in ui.input(|i| i.events.clone()) {
                        if let egui::Event::Text(text) = event {
                            if let Some(c) = text.chars().next() {
                                if c.is_ascii_alphabetic() {
                                    self.guess(c);
                                }
                            }
                        }
                    }
                } else {
                    ui.add_space(10.0);
                    if ui.button("再来一局").clicked() {
                        self.reset();
                    }
                }
            });
        });
    }
}
