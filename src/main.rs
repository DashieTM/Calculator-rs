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

use adw::prelude::*;
use glib::clone;
use gtk4 as gtk;
use std::collections::HashMap;
use std::{fs, rc::Rc};
use toml;

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
    variables: HashMap<String, f64>,
    toml_path: String,
}

enum ErrorMessages {
    ZeroDivisionError,
    NotANumberError,
    NotAnOperatorError,
    BrackedError,
    TangentOutOfScope,
    NegativeLogException,
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
        calc_ref.borrow_mut().toml_path = String::from("/home/dashie/.config/calc/variables.toml");
        calc_ref.borrow_mut().read_toml();
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
        let calc_sin = calc_ref.clone();
        let calc_cos = calc_ref.clone();
        let calc_tan = calc_ref.clone();
        let calc_log = calc_ref.clone();
        let calc_exp = calc_ref.clone();
        let calc_enter = calc_ref.clone();
        let calc_delete = calc_ref.clone();
        let calc_clear = calc_ref.clone();
        let calc_vars = calc_ref.clone();
        let calc_tree = calc_ref.clone();
        // let buffer = Rc::new(std::cell::RefCell::new(&input_text));
        let input_field_ref = Rc::new(std::cell::RefCell::new(
            gtk::Entry::builder()
                .placeholder_text("Enter an expression")
                .margin_top(10)
                .margin_start(10)
                .margin_end(10)
                .vexpand(true)
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
        let buffer_refsin = input_field_ref.clone();
        let buffer_refcos = input_field_ref.clone();
        let buffer_reftan = input_field_ref.clone();
        let buffer_reflog = input_field_ref.clone();
        let buffer_refexp = input_field_ref.clone();
        let buffer_refenter = input_field_ref.clone();
        let buffer_refdelete = input_field_ref.clone();
        let buffer_refclear = input_field_ref.clone();
        let history_buffer = Box::new(std::cell::RefCell::new(String::from("")));
        let main_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        let number_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_end(23)
            .build();
        let operator_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        let utility_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_end(10)
            .build();
        let button_box = gtk::Box::builder().margin_start(10).margin_end(10).build();
        let io_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_top(5)
            .build();
        let number_row1 = gtk::Box::builder().build();
        let number_row2 = gtk::Box::builder().build();
        let number_row3 = gtk::Box::builder().build();
        let number_row4 = gtk::Box::builder().build();
        let operator_row1 = gtk::Box::builder().build();
        let operator_row2 = gtk::Box::builder().build();
        let operator_row3 = gtk::Box::builder().build();
        let operator_row4 = gtk::Box::builder().build();
        let utility_row1 = gtk::Box::builder().margin_start(10).build();
        let utility_row2 = gtk::Box::builder().margin_start(10).build();

        let result_window = gtk::ScrolledWindow::builder()
            .margin_start(13)
            .margin_end(13)
            .margin_top(5)
            .margin_bottom(10)
            .hscrollbar_policy(gtk::PolicyType::Automatic)
            .has_frame(true)
            .height_request(80)
            .vexpand(true)
            .build();

        let history = Rc::new(std::cell::RefCell::new(gtk::TextView::builder().build()));
        let history_instance = history.borrow();
        history_instance.set_editable(false);
        let result_buffer = gtk::TextBuffer::builder().build();
        let history_ref = history.clone();
        let variable_menu = gtk::PopoverMenu::builder().build();
        let variable_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let labels = std::cell::RefCell::new(Vec::<gtk::Label>::new());
        let listbox = Rc::new(std::cell::RefCell::new(gtk::ListBox::new()));
        let list_box_ref = listbox.borrow();
        let list_box_ref_mut = listbox.clone();

        let variable_input = Rc::new(std::cell::RefCell::new(
            gtk::Entry::builder()
                .placeholder_text("Enter a variable command")
                .margin_top(10)
                .margin_bottom(10)
                .margin_start(10)
                .margin_end(10)
                .build(),
        ));
        let variable_input_ref = variable_input.borrow();

        let num_1 = gtk::Button::builder()
            .label("1")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let num_2 = gtk::Button::builder()
            .label("2")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let num_3 = gtk::Button::builder()
            .label("3")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let num_4 = gtk::Button::builder()
            .label("4")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let num_5 = gtk::Button::builder()
            .label("5")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let num_6 = gtk::Button::builder()
            .label("6")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let num_7 = gtk::Button::builder()
            .label("7")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let num_8 = gtk::Button::builder()
            .label("8")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let num_9 = gtk::Button::builder()
            .label("9")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let num_0 = gtk::Button::builder()
            .label("0")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let dot = gtk::Button::builder()
            .label(".")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let add = gtk::Button::builder()
            .label("+")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let sub = gtk::Button::builder()
            .label("-")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let mul = gtk::Button::builder()
            .label("*")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let div = gtk::Button::builder()
            .label("/")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let modu = gtk::Button::builder()
            .label("%")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let fact = gtk::Button::builder()
            .label("!")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let leftbrace = gtk::Button::builder()
            .label("(")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let rightbrace = gtk::Button::builder()
            .label(")")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let sin = gtk::Button::builder()
            .label("sin")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let cos = gtk::Button::builder()
            .label("cos")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let tan = gtk::Button::builder()
            .label("tan")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let log = gtk::Button::builder()
            .label("log")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let exp = gtk::Button::builder()
            .label("^")
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let enter = gtk::Button::builder()
            .label("calculate")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(3)
            .margin_end(12)
            .vexpand(true)
            .hexpand(true)
            .build();
        let delete = gtk::Button::builder()
            .label("delete")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(16)
            .margin_end(16)
            .vexpand(true)
            .hexpand(true)
            .build();
        let clear = gtk::Button::builder()
            .label("clear")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .build();
        let variables = gtk::MenuButton::builder()
            .label("x")
            .direction(gtk::ArrowType::None)
            .margin_top(3)
            .margin_bottom(3)
            .margin_start(3)
            .margin_end(3)
            .vexpand(true)
            .hexpand(true)
            .popover(&variable_menu)
            .build();

        let enter_closure = Rc::new(std::cell::RefCell::new(move || {
            let mut calc = calc_enter.borrow_mut();
            let input = buffer_refenter.borrow_mut();
            let history_borrow = history_ref.borrow_mut();
            let mut history_buffer_ref = history_buffer.borrow_mut();

            calc.entry = String::from(input.text().clone());

            if calc.entry == "" {
                return;
            }

            calc.tokens = split_string(&calc.entry);
            negative_clean(&mut calc.tokens);
            calc.parse_tokens();
            calc.next();
            let maybe_result = calc.handle_expression();
            match maybe_result {
                Ok(res) => calc.result = res,
                Err(error) => {
                    input.set_text("");
                    match error {
                        ErrorMessages::NotANumberError => input.set_placeholder_text(Some(
                            format!("Expected number at: {}", calc.current).as_str(),
                        )),
                        ErrorMessages::NotAnOperatorError => input.set_placeholder_text(Some(
                            format!("Expected operator at: {}", calc.current).as_str(),
                        )),
                        ErrorMessages::BrackedError => input.set_placeholder_text(Some(
                            format!("Expected bracket at: {}", calc.current).as_str(),
                        )),
                        ErrorMessages::ZeroDivisionError => {
                            input.set_placeholder_text(Some("Division by zero!"))
                        }
                        _ => input
                            .set_placeholder_text(Some(format!("todo: {}", calc.current).as_str())),
                    };
                    return;
                }
            }
            *history_buffer_ref +=
                &(calc.entry.clone() + " = " + calc.result.to_string().as_str() + "\n");

            result_buffer.set_text(&*history_buffer_ref);
            history_borrow.set_buffer(Some(&result_buffer));

            input.set_placeholder_text(Some(&("Result: ".to_string() + &calc.result.to_string())));
            input.set_text("");
            calc.entry = String::from("");
            calc.result = 0.0;
        }));
        let enter_closure_wrapper =
            clone!(@strong enter_closure => move || enter_closure.borrow_mut()());
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
        sin.connect_clicked(move |_sin| {
            let mut calc = calc_sin.borrow_mut();
            let input = buffer_refsin.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "sin";
            input.set_text(calc.entry.as_str());
        });
        cos.connect_clicked(move |_cos| {
            let mut calc = calc_cos.borrow_mut();
            let input = buffer_refcos.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "cos";
            input.set_text(calc.entry.as_str());
        });
        tan.connect_clicked(move |_tan| {
            let mut calc = calc_tan.borrow_mut();
            let input = buffer_reftan.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "tan";
            input.set_text(calc.entry.as_str());
        });
        log.connect_clicked(move |_log| {
            let mut calc = calc_log.borrow_mut();
            let input = buffer_reflog.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "log";
            input.set_text(calc.entry.as_str());
        });
        exp.connect_clicked(move |_exp| {
            let mut calc = calc_exp.borrow_mut();
            let input = buffer_refexp.borrow_mut();
            calc.entry = String::from(input.text().clone());
            calc.entry += "^";
            input.set_text(calc.entry.as_str());
        });
        enter.connect_clicked(
            clone!(@strong enter_closure_wrapper => move |_| enter_closure_wrapper()),
        );
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
            input.set_placeholder_text(Some("Enter an Expression"))
        });
        input_field.connect_activate(
            clone!(@strong enter_closure_wrapper => move |_| enter_closure_wrapper()),
        );
        variable_menu.connect_show(move |_| {
            let calc = calc_tree.borrow_mut();
            let list_box = list_box_ref_mut.borrow_mut();
                while let Some(label) = list_box.first_child() {
                    list_box.remove(&label);
                }

            for (var, val) in calc.variables.iter() {
                let label = gtk::Label::builder()
                    .label(&format!("{} = {}", var, val))
                    .build();
                list_box.append(&label);
            }
        });
        variable_input
            .borrow()
            .connect_activate(move |variable_input| {
                let mut calc = calc_vars.borrow_mut();
                let command = variable_input.text();
                let mut tokens = command.split(" ");
                match tokens.next().unwrap() {
                    "add" => {
                        let key = tokens.next();
                        if key.is_none() {
                            variable_input.set_placeholder_text(Some("Could not parse command"));
                            variable_input.set_text("");
                        }
                        let val = tokens.next();
                        if val.is_none() {
                            variable_input.set_placeholder_text(Some("Could not parse command"));
                            variable_input.set_text("");
                        }
                        let val_parse = String::from(val.unwrap()).parse::<f64>();
                        if val_parse.is_err() {
                            variable_input.set_placeholder_text(Some("Could not parse variable"));
                            variable_input.set_text("");
                            return;
                        }
                        let end_var = val_parse.ok().unwrap();
                        let end_key = key.unwrap();

                        calc.variables.insert(String::from(end_key), end_var);
                        variable_input.set_placeholder_text(Some(
                            format!("Added variable {} with value {}", end_key, end_var).as_str(),
                        ));
                        variable_input.set_text("");
                        calc.write_toml();
                    }
                    "delete" => {
                        let key = tokens.next();
                        if key.is_none() {
                            variable_input
                                .set_placeholder_text(Some("Please enter variable to delete"));
                            variable_input.set_text("");
                        }
                        let key_end = key.unwrap();
                        let mut should_delete = false;
                        for (calc_key, _) in calc.variables.iter() {
                            if *calc_key == key_end {
                                should_delete = true;
                            }
                        }
                        if !should_delete {
                            variable_input.set_placeholder_text(Some("Variable not found!"));
                            variable_input.set_text("");
                        }
                        calc.variables.remove(key_end).unwrap();
                        variable_input.set_placeholder_text(Some(
                            format!("Deleted variable {}", key_end).as_str(),
                        ));
                        variable_input.set_text("");
                        calc.write_toml();
                    }
                    _ => {
                        variable_input.set_text("");
                        variable_input.set_placeholder_text(Some("Could not parse command"));
                    }
                }
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
        number_row4.append(&variables);
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
        utility_row2.append(&sin);
        utility_row2.append(&cos);
        utility_row2.append(&tan);
        utility_row2.append(&log);
        utility_row2.append(&exp);
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
        result_window.set_child(Some(&*history_instance));
        main_box.append(&io_box);
        main_box.append(&utility_box);
        main_box.append(&button_box);
        main_box.append(&result_window);

        variable_menu.set_child(Some(&variable_box));
        variable_box.append(&*variable_input_ref);
        variable_box.append(&*list_box_ref);

        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("My GTK App")
            .content(&main_box)
            .default_width(300)
            .default_height(400)
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
            variables: Default::default(),
            toml_path: String::from(""),
        }
    }

    fn handle_expression(&mut self) -> Result<f64, ErrorMessages> {
        let maybe_result = self.handle_term();
        if maybe_result.is_err() {
            return maybe_result;
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
        if maybe_result.is_err() {
            return maybe_result;
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
                self.expect_number = true;
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
        } else if is_special(&self.current) {
            let maybe_result = self.handle_specials();
            if maybe_result.is_err() {
                return maybe_result;
            }
            self.next();
            if self.current != ")" {
                return Err(ErrorMessages::BrackedError);
            }
            self.next();
            self.expect_number = false;
            return Ok(maybe_result.ok().unwrap());
        } else if self.current == "-(" {
            let maybe_result = self.handle_specials();
            if maybe_result.is_err() {
                return maybe_result;
            }
            self.next();
            if self.current != ")" {
                return Err(ErrorMessages::BrackedError);
            }
            self.next();
            self.expect_number = false;
            return Ok(negate(maybe_result.ok().unwrap()));
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

    fn handle_specials(&mut self) -> Result<f64, ErrorMessages> {
        if !self.next_char() {
            return Err(ErrorMessages::NotAnOperatorError);
        }
        if self.next_char == '-' {
            self.current.remove(0);
            let maybe_result = self.handle_specials();
            if maybe_result.is_err() {
                return maybe_result;
            }
            return Ok(negate(maybe_result.ok().unwrap()));
        }
        match self.current.as_str() {
            "cos" => {
                self.next();
                if self.current != "(" {
                    return Err(ErrorMessages::BrackedError);
                }
                self.next();
                let maybe_number = self.current.parse::<f64>();
                if maybe_number.is_err() {
                    return Err(ErrorMessages::NotANumberError);
                }
                return Ok((maybe_number.ok().unwrap() * (std::f64::consts::PI / 180.0)).cos());
            }
            "sin" => {
                self.next();
                if self.current != "(" {
                    return Err(ErrorMessages::BrackedError);
                }
                self.next();
                let maybe_number = self.current.parse::<f64>();
                if maybe_number.is_err() {
                    return Err(ErrorMessages::NotANumberError);
                }
                return Ok((maybe_number.ok().unwrap() * (std::f64::consts::PI / 180.0)).sin());
            }
            "tan" => {
                self.next();
                if self.current != "(" {
                    return Err(ErrorMessages::BrackedError);
                }
                self.next();
                let maybe_number = self.current.parse::<f64>();
                if maybe_number.is_err() {
                    return Err(ErrorMessages::NotANumberError);
                }
                let tannum = maybe_number.clone().ok().unwrap();
                if tannum / 90.0 % 2.0 == 1.0 || tannum / 90.0 % 2.0 == -1.0 {
                    return Err(ErrorMessages::TangentOutOfScope);
                }
                return Ok((maybe_number.ok().unwrap() * std::f64::consts::PI / 180.0).tan());
            }
            _ => {
                let mut logbase = std::f64::consts::E;
                self.next();
                let maybe_base = self.current.parse::<f64>();
                if maybe_base.is_ok() && !is_negative(maybe_base.clone().ok().unwrap()) {
                    logbase = maybe_base.ok().unwrap();
                    self.next();
                    if self.current != "(" {
                        return Err(ErrorMessages::BrackedError);
                    }
                    self.next();
                } else {
                    if self.current != "(" {
                        return Err(ErrorMessages::BrackedError);
                    }
                    self.next();
                }
                let maybe_number = self.current.parse::<f64>();
                if maybe_number.is_err() {
                    return Err(ErrorMessages::NotANumberError);
                }
                let num = maybe_number.ok().unwrap();
                if is_negative(num) {
                    return Err(ErrorMessages::NegativeLogException);
                }
                return Ok(num.log(logbase));
            }
        }
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

    fn read_toml(&mut self) {
        let content = match fs::read_to_string(&self.toml_path) {
            Ok(c) => c,
            Err(_) => panic!("Could not open variable.toml"),
        };
        self.variables = match toml::from_str(&content) {
            Ok(c) => c,
            Err(err) => panic!("error converting toml: {}", err),
        };
    }

    fn write_toml(&self) {
        let toml_string =
            toml::to_string(&self.variables).expect("Error parsing variables to toml");
        println!("{}", toml_string);
        fs::write(&self.toml_path, toml_string).expect("Could not write to toml");
    }

    fn parse_tokens(&mut self) {
        for token in self.tokens.iter_mut() {
            for (key, var) in self.variables.iter() {
                let mut neg = false;
                if token.starts_with('-') {
                    neg = true;
                    token.remove(0);
                }
                if *token == *key {
                    *token = var.to_string();
                }
                if neg {
                    token.insert(0, '-');
                }
            }
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
    tokens
}

fn negative_clean(tokens: &mut Vec<String>) {
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
}

fn negate(num: f64) -> f64 {
    return -num;
}

fn is_negative(num: f64) -> bool {
    if num < 0.0 {
        return true;
    }
    false
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

fn is_special(str: &String) -> bool {
    match str.as_str() {
        "sin" => true,
        "cos" => true,
        "tan" => true,
        "log" => true,
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
    if (power as i64) > 0 {
        match exponential(base, power - 1.0) {
            Ok(res) => Ok(base * res),
            Err(err) => Err(err),
        }
    } else if (power as i64) < 0 {
        match exponential(base, power + 1.0) {
            Ok(res) => Ok(base * res),
            Err(err) => Err(err),
        }
    } else {
        return Ok(1.0);
    }
}
