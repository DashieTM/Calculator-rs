/*
Copyright © 2023 Fabio Lenherr
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.
You should have received a copy of the GNU Affero General Public License
along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

use std::rc::Rc;
use adw::prelude::*;
use gtk4 as gtk;

fn main() {
    run_gui();
}

#[derive(Clone)]
struct Calculator {
    entry: String,
    result: f64,
    expect_number: bool,
    current: String,
    tokens: Vec<String>,
    next_char: char,
}

enum ErrorMessages {
    ZeroDivisionError,
    NotANumberError,
    NotAnOperatorError,
    BrackedError,
}

fn run_gui() {
    let calc_ref = Rc::new(std::cell::RefCell::new(Calculator::new()));
    let app = adw::Application::builder()
        .application_id("org.dashie.Calculator")
        .build();

    app.connect_startup(|_| {
        adw::init().unwrap();
    });
    app.connect_activate(move |app| {
        let calc1 = calc_ref.clone();
        let calc2 = calc_ref.clone();
        let calc3 = calc_ref.clone();
        let calc4 = calc_ref.clone();
        let calc5 = calc_ref.clone();
        let calc6 = calc_ref.clone();
        let calc7 = calc_ref.clone();
        let calc8 = calc_ref.clone();
        let calc9 = calc_ref.clone();
        let calc0 = calc_ref.clone();
        let calcdot = calc_ref.clone();
        let calc_add = calc_ref.clone();
        let calc_sub = calc_ref.clone();
        let calc_mul = calc_ref.clone();
        let calc_div = calc_ref.clone();
        let calc_modu = calc_ref.clone();
        let calc_fact = calc_ref.clone();
        let calc_leftbrace = calc_ref.clone();
        let calc_rightbrace = calc_ref.clone();
        let calc_enter = calc_ref.clone();
        let calc_delete = calc_ref.clone();
        let calc_clear = calc_ref.clone();
        // let buffer = Rc::new(std::cell::RefCell::new(&input_text));
        let input_field_ref = Rc::new(std::cell::RefCell::new(
            gtk::Entry::builder()
                .placeholder_text("Enter an expression")
                .build(),
        ));
        let input_field = input_field_ref.borrow();
        let buffer_ref1 = input_field_ref.clone();
        let buffer_ref2 = input_field_ref.clone();
        let buffer_ref3 = input_field_ref.clone();
        let buffer_ref4 = input_field_ref.clone();
        let buffer_ref5 = input_field_ref.clone();
        let buffer_ref6 = input_field_ref.clone();
        let buffer_ref7 = input_field_ref.clone();
        let buffer_ref8 = input_field_ref.clone();
        let buffer_ref9 = input_field_ref.clone();
        let buffer_ref0 = input_field_ref.clone();
        let buffer_refdot = input_field_ref.clone();
        let buffer_refadd = input_field_ref.clone();
        let buffer_refsub = input_field_ref.clone();
        let buffer_refmul = input_field_ref.clone();
        let buffer_refdiv = input_field_ref.clone();
        let buffer_refmodu = input_field_ref.clone();
        let buffer_reffact = input_field_ref.clone();
        let buffer_refleftbrace = input_field_ref.clone();
        let buffer_refrightbrace = input_field_ref.clone();
        let buffer_refenter = input_field_ref.clone();
        let buffer_refdelete = input_field_ref.clone();
        let buffer_refclear = input_field_ref.clone();
        let main_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        let number_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        let operator_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        let utility_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        let button_box = gtk::Box::builder().build();
        let io_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        let number_row1 = gtk::Box::builder().build();
        let number_row2 = gtk::Box::builder().build();
        let number_row3 = gtk::Box::builder().build();
        let number_row4 = gtk::Box::builder().build();
        let operator_row1 = gtk::Box::builder().build();
        let operator_row2 = gtk::Box::builder().build();
        let operator_row3 = gtk::Box::builder().build();
        let operator_row4 = gtk::Box::builder().build();
        let utility_row1 = gtk::Box::builder().build();
        let utility_row2 = gtk::Box::builder().build();
        let num_1 = gtk::Button::builder()
            .label("1")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        let num_2 = gtk::Button::builder()
            .label("2")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        let num_3 = gtk::Button::builder()
            .label("3")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        let num_4 = gtk::Button::builder()
            .label("4")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        let num_5 = gtk::Button::builder()
            .label("5")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        let num_6 = gtk::Button::builder()
            .label("6")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        let num_7 = gtk::Button::builder()
            .label("7")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        let num_8 = gtk::Button::builder()
            .label("8")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        let num_9 = gtk::Button::builder()
            .label("9")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        let num_0 = gtk::Button::builder()
            .label("0")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        let dot = gtk::Button::builder()
            .label(".")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        let add = gtk::Button::builder()
            .label("+")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        let sub = gtk::Button::builder()
            .label("-")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        let mul = gtk::Button::builder()
            .label("*")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        let div = gtk::Button::builder()
            .label("/")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        let modu = gtk::Button::builder()
            .label("%")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        let fact = gtk::Button::builder()
            .label("!")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        let leftbrace = gtk::Button::builder()
            .label("(")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        let rightbrace = gtk::Button::builder()
            .label(")")
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(5)
            .margin_end(5)
            .build();
        let enter = gtk::Button::builder()
            .label("calculate")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        let delete = gtk::Button::builder()
            .label("delete")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        let clear = gtk::Button::builder()
            .label("clear")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        num_1.connect_clicked(move |_num_1| {
            let mut calc = calc1.borrow_mut();
            let input = buffer_ref1.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "1";
            input.set_text(calc.entry.as_str());
        });
        num_2.connect_clicked(move |_num_2| {
            let mut calc = calc2.borrow_mut();
            let input = buffer_ref2.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "2";
            input.set_text(calc.entry.as_str());
        });
        num_3.connect_clicked(move |_num_3| {
            let mut calc = calc3.borrow_mut();
            let input = buffer_ref3.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "3";
            input.set_text(calc.entry.as_str());
        });
        num_4.connect_clicked(move |_num_4| {
            let mut calc = calc4.borrow_mut();
            let input = buffer_ref4.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "4";
            input.set_text(calc.entry.as_str());
        });
        num_5.connect_clicked(move |_num_5| {
            let mut calc = calc5.borrow_mut();
            let input = buffer_ref5.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "5";
            input.set_text(calc.entry.as_str());
        });
        num_6.connect_clicked(move |_num_6| {
            let mut calc = calc6.borrow_mut();
            let input = buffer_ref6.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "6";
            input.set_text(calc.entry.as_str());
        });
        num_7.connect_clicked(move |_num_7| {
            let mut calc = calc7.borrow_mut();
            let input = buffer_ref7.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "7";
            input.set_text(calc.entry.as_str());
        });
        num_8.connect_clicked(move |_num_8| {
            let mut calc = calc8.borrow_mut();
            let input = buffer_ref8.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "8";
            input.set_text(calc.entry.as_str());
        });
        num_9.connect_clicked(move |_num_9| {
            let mut calc = calc9.borrow_mut();
            let input = buffer_ref9.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "9";
            input.set_text(calc.entry.as_str());
        });
        num_0.connect_clicked(move |_num_0| {
            let mut calc = calc0.borrow_mut();
            let input = buffer_ref0.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "0";
            input.set_text(calc.entry.as_str());
        });
        dot.connect_clicked(move |_dot| {
            let mut calc = calcdot.borrow_mut();
            let input = buffer_refdot.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += ".";
            input.set_text(calc.entry.as_str());
        });
        add.connect_clicked(move |_add| {
            let mut calc = calc_add.borrow_mut();
            let input = buffer_refadd.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "+";
            input.set_text(calc.entry.as_str());
        });
        sub.connect_clicked(move |_sub| {
            let mut calc = calc_sub.borrow_mut();
            let input = buffer_refsub.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "-";
            input.set_text(calc.entry.as_str());
        });
        mul.connect_clicked(move |_mul| {
            let mut calc = calc_mul.borrow_mut();
            let input = buffer_refmul.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "*";
            input.set_text(calc.entry.as_str());
        });
        div.connect_clicked(move |_div| {
            let mut calc = calc_div.borrow_mut();
            let input = buffer_refdiv.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "/";
            input.set_text(calc.entry.as_str());
        });
        modu.connect_clicked(move |_modu| {
            let mut calc = calc_modu.borrow_mut();
            let input = buffer_refmodu.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "%";
            input.set_text(calc.entry.as_str());
        });
        fact.connect_clicked(move |_fact| {
            let mut calc = calc_fact.borrow_mut();
            let input = buffer_reffact.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "!";
            input.set_text(calc.entry.as_str());
        });
        leftbrace.connect_clicked(move |_leftbrace| {
            let mut calc = calc_leftbrace.borrow_mut();
            let input = buffer_refleftbrace.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "(";
            input.set_text(calc.entry.as_str());
        });
        rightbrace.connect_clicked(move |_rightbrace| {
            let mut calc = calc_rightbrace.borrow_mut();
            let input = buffer_refrightbrace.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += ")";
            input.set_text(calc.entry.as_str());
        });
        enter.connect_clicked(move |_enter| {
            let mut calc = calc_enter.borrow_mut();
            let input = buffer_refenter.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.calculate();
            input.set_text(calc.result.to_string().as_str());
            calc.entry = String::from("");
        });
        delete.connect_clicked(move |_delete| {
            let mut calc = calc_delete.borrow_mut();
            let input = buffer_refdelete.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry.pop();
            input.set_text(calc.entry.to_string().as_str());
        });
        clear.connect_clicked(move |_clear| {
            let mut calc = calc_clear.borrow_mut();
            calc.entry = String::from("");
            let input = buffer_refclear.borrow_mut();
            input.set_text("");
        });

        number_row1.append(&num_1);
        number_row1.append(&num_2);
        number_row1.append(&num_3);
        number_row2.append(&num_4);
        number_row2.append(&num_5);
        number_row2.append(&num_6);
        number_row3.append(&num_7);
        number_row3.append(&num_8);
        number_row3.append(&num_9);
        number_row4.append(&num_0);
        number_row4.append(&dot);
        operator_row1.append(&add);
        operator_row1.append(&modu);
        operator_row2.append(&sub);
        operator_row2.append(&fact);
        operator_row3.append(&mul);
        operator_row3.append(&leftbrace);
        operator_row4.append(&div);
        operator_row4.append(&rightbrace);
        utility_row1.append(&enter);
        utility_row1.append(&delete);
        utility_row1.append(&clear);
        number_box.append(&number_row1);
        number_box.append(&number_row2);
        number_box.append(&number_row3);
        number_box.append(&number_row4);
        operator_box.append(&operator_row1);
        operator_box.append(&operator_row2);
        operator_box.append(&operator_row3);
        operator_box.append(&operator_row4);
        utility_box.append(&utility_row1);
        utility_box.append(&utility_row2);
        button_box.append(&number_box);
        button_box.append(&operator_box);
        io_box.append(&*input_field);
        main_box.append(&io_box);
        main_box.append(&utility_box);
        main_box.append(&button_box);

        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("My GTK App")
            .content(&main_box)
            .build();

        // Present window
        window.present();
    });
    app.run();
}

impl Calculator {
    // then we implement the trait that brings the drawing functionality from iced to our struct
    // type Message = CalculatorMessage;

    fn new() -> Self {
        // initializes the state of your application
        Calculator {
            entry: String::from(""),
            result: 0.0,
            expect_number: true,
            current: String::from(""),
            tokens: Vec::new(),
            next_char: '_',
        }
    }

    fn handle_expression(&mut self) -> Result<f64, ErrorMessages> {
        let maybe_result = self.handle_term();
        if !maybe_result.is_ok() {
            return Err(ErrorMessages::NotANumberError);
        }
        let mut result = maybe_result.ok().unwrap();
        if self.expect_number {
            return Ok(result);
        }
        while self.current != "" && self.current != ")" {
            if !self.next_char() {
                return Ok(result);
            }
            match self.next_char {
                '+' => {
                    self.next();
                    self.expect_number = true;
                    result += self.handle_term().ok().unwrap();
                    return Ok(result);
                }
                '-' => {
                    self.next();
                    self.expect_number = true;
                    result -= self.handle_term().ok().unwrap();
                    return Ok(result);
                }
                _ => return Err(ErrorMessages::NotAnOperatorError),
            }
        }
        Ok(result)
    }

    fn handle_term(&mut self) -> Result<f64, ErrorMessages> {
        let maybe_result = self.handle_factor();
        if !maybe_result.is_ok() {
            return Err(ErrorMessages::NotANumberError);
        }
        let mut result = maybe_result.ok().unwrap();
        if !self.next_char() {
            return Ok(result);
        }
        match self.next_char {
            '!' => {
                self.next();
                self.expect_number = false;
                let maybe_result = factorial(result as i64);
                if maybe_result.is_err() {
                    return maybe_result;
                }
                result = maybe_result.ok().unwrap();
                if !self.next_char() {
                    return Ok(result);
                }
                if is_operator(self.next_char) && self.next_char != ')' {
                    self.tokens.insert(0, self.current.clone());
                    self.current = result.to_string();
                    return self.handle_term();
                }
                return Ok(result);
            }
            '^' => {
                self.next();
                self.expect_number = false;
                let maybe_fact = self.handle_factor();
                if maybe_fact.is_err() {
                    return maybe_fact;
                }
                let maybe_expt = exponential(result, maybe_fact.ok().unwrap());
                if maybe_expt.is_err() {
                    return maybe_expt;
                }
                result = maybe_expt.ok().unwrap();
                if !self.next_char() {
                    return Ok(result);
                }
                if is_operator(self.next_char) && self.next_char != ')' {
                    self.tokens.insert(0, self.current.clone());
                    self.current = result.to_string();
                    return self.handle_term();
                }
                return Ok(result);
            }
            '%' => {
                self.next();
                self.expect_number = true;
                let maybe_div = self.handle_factor();
                if maybe_div.is_err() {
                    return maybe_div;
                }
                let div = maybe_div.ok().unwrap();
                if div == 0.0 {
                    return Err(ErrorMessages::ZeroDivisionError);
                }
                result = (result as i64 % div as i64) as f64;
                if !self.next_char() {
                    return Ok(result);
                }
                if is_operator(self.next_char) && self.next_char != ')' {
                    self.tokens.insert(0, self.current.clone());
                    self.current = result.to_string();
                    return self.handle_term();
                }
                return Ok(result);
            }
            '*' => {
                self.next();
                self.expect_number = true;
                let maybe_mul = self.handle_term();
                if !maybe_mul.is_ok() {
                    return Err(ErrorMessages::NotANumberError);
                }
                result *= maybe_mul.ok().unwrap();
                return Ok(result);
            }
            '/' => {
                self.next();
                self.expect_number = true;
                let maybe_div = self.handle_term();
                if !maybe_div.is_ok() {
                    return Err(ErrorMessages::NotANumberError);
                }
                let div = maybe_div.ok().unwrap();
                if div == 0.0 {
                    return Err(ErrorMessages::ZeroDivisionError);
                }
                result /= div;
                if !self.next_char() {
                    return Ok(result);
                }
                if is_operator(self.current.chars().next().unwrap()) {
                    self.tokens.insert(0, self.current.clone());
                    self.current = result.to_string();
                    result = self.handle_term().ok().unwrap();
                    return Ok(result);
                }
                return Ok(result);
            }
            _ => Ok(result),
        }
    }

    fn handle_factor(&mut self) -> Result<f64, ErrorMessages> {
        // let mut result: f64 = self.handle_term().ok().unwrap();
        let mut result = 0.0;
        if self.current == "(" {
            self.next();
            result = self.handle_expression().ok().unwrap();
            if self.current != ")" {
                return Err(ErrorMessages::BrackedError);
            }
            self.next();
            self.expect_number = false;
        } else {
            if self.current == "" {
                return Ok(result);
            }
            let maybe_result = self.current.parse::<f64>();
            if !maybe_result.is_ok() {
                return Err(ErrorMessages::NotANumberError);
            }
            result = maybe_result.ok().unwrap();
            self.next();
            self.expect_number = false;
        }
        Ok(result)
    }

    fn next(&mut self) {
        if self.has_next() {
            self.current = self.tokens.remove(0);
            return;
        }
        self.current = String::from("");
    }

    fn next_char(&mut self) -> bool {
        let next_char = self.current.chars().next();
        if !next_char.is_some() {
            return false;
        }
        self.next_char = next_char.unwrap();
        return true;
    }

    fn has_next(&self) -> bool {
        if self.tokens.len() > 0 {
            return true;
        }
        return false;
    }

    fn calculate(&mut self) {
        self.tokens = split_string(&self.entry);
        for token in self.tokens.iter() {
            println!("{token}");
        }
        self.next();
        let maybe_result = self.handle_expression();
        match maybe_result {
            Ok(res) => self.result = res,
            Err(error) => match error {
                ErrorMessages::NotANumberError => println!("Expected number at: {}", self.current),
                ErrorMessages::NotAnOperatorError => {
                    println!("Expected operator at: {}", self.current)
                }
                ErrorMessages::BrackedError => println!("Expected bracket at {}", self.current),
                ErrorMessages::ZeroDivisionError => println!("Division by zero at!"),
            },
        }
    }
}

fn split_string(str: &String) -> Vec<String> {
    let mut buffer = String::from("");
    let mut tokens: Vec<String> = Vec::new();

    let mut digit_lock = false;
    let mut char_lock = false;
    let mut dot_lock = false;
    let mut op_lock = true;
    let mut op_current = false;

    for chr in str.chars() {
        if chr.is_numeric() || (chr == '.' && !dot_lock) || (chr == '-' && op_lock) {
            if char_lock {
                char_lock = false;
                tokens.push(buffer.clone());
                buffer.clear();
            }
            if chr == '.' {
                dot_lock = true;
            }
            if chr == '-' {
                op_current = true;
                buffer += &chr.to_string();
                continue;
            } else {
                op_current = false;
            }
            op_lock = false;
            digit_lock = true;
            buffer += &chr.to_string();
            continue;
        } else if digit_lock && !op_current {
            digit_lock = false;
            dot_lock = false;
            tokens.push(buffer.clone());
            buffer.clear();
        }
        if is_operator(chr) {
            if op_current && chr == '-' {
                // handle infinite - cases
                op_lock = false;
                digit_lock = false;
                buffer += &chr.to_string();
                continue;
            }
            op_current = false;
            if char_lock {
                char_lock = false;
                tokens.push(buffer.clone());
                buffer.clear();
            }
            buffer += &chr.to_string();
            tokens.push(buffer.clone());
            op_lock = true;
            buffer.clear();
        } else if chr != ' ' && chr != '_' {
            buffer += &chr.to_string();
            char_lock = true;
            op_current = false;
        } else if char_lock {
            char_lock = false;
            tokens.push(buffer.clone());
            buffer.clear();
        }
    }
    if buffer != "" {
        tokens.push(buffer.clone());
    }
    let mut min_lock = false;
    for token in tokens.iter_mut() {
        if token.len() > 1 {
            // simply iterate over the amount of - and remove them all.
            // at the end we check whether or not we have an even or uneven amount
            // even means no change, uneven means negate the number
            while token.starts_with('-') {
                min_lock = !min_lock;
                token.remove(0);
            }
            if min_lock {
                token.insert(0, '-');
            }
            min_lock = false;
        }
    }
    tokens
}

fn is_operator(chr: char) -> bool {
    match chr {
        '+' => true,
        '-' => true,
        '*' => true,
        '/' => true,
        '%' => true,
        '^' => true,
        '(' => true,
        ')' => true,
        _ => false,
    }
}

fn factorial(num: i64) -> Result<f64, ErrorMessages> {
    if num > 1 {
        let maybe_fact = factorial(num - 1);
        if maybe_fact.is_err() {
            return Err(ErrorMessages::NotANumberError);
        }
        return Ok((num * maybe_fact.ok().unwrap() as i64) as f64);
    }
    if num < 0 {
        return Err(ErrorMessages::NotANumberError);
    }
    Ok(1.0)
}

fn exponential(base: f64, power: f64) -> Result<f64, ErrorMessages> {
    if power > 0.0 {
        return exponential(base, power - 1.0);
    } else if power < 0.0 {
        return exponential(base, power + 1.0);
    } else {
        return Ok(1.0);
    }
}
