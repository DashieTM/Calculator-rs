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

use crate::calculator::*;
use adw::prelude::*;
use directories_next as dirs;
use glib::clone;
use gtk4 as gtk;
use std::{fs, rc::Rc};

pub fn run_gui() {
    let calc_ref = Rc::new(std::cell::RefCell::new(Calculator::new()));
    let app = adw::Application::builder()
        .application_id("org.dashie.OxiCalc")
        .build();

    app.connect_startup(|_| {
        adw::init().unwrap();
    });
    app.connect_activate(move |app| {
        let maybe_config_dir = dirs::ProjectDirs::from("com", "dashie", "calc");
        if maybe_config_dir.is_none() {
            panic!("Could not get config directory");
        }
        let config = maybe_config_dir.unwrap();
        let config_dir = config.config_dir();
        if !config_dir.exists() {
            fs::create_dir(config_dir).expect("Could not create config directory");
        }
        let metadata = fs::metadata(config_dir);
        if metadata.is_err() {
            panic!("Could not check directory metadata for config file");
        }
        let file_path = config_dir.join("variables.toml");
        let file = metadata.unwrap();
        if !file.is_file() {
            fs::File::create(&file_path).expect("Could not write config file");
        }
        calc_ref.borrow_mut().toml_path = String::from(file_path.to_str().unwrap());
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
        let title_bar = adw::HeaderBar::builder().build();
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
        main_box.append(&title_bar);
        main_box.append(&io_box);
        main_box.append(&utility_box);
        main_box.append(&button_box);
        main_box.append(&result_window);

        variable_menu.set_child(Some(&variable_box));
        variable_box.append(&*variable_input_ref);
        variable_box.append(&*list_box_ref);

        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("OxiCalc")
            .content(&main_box)
            .default_width(300)
            .default_height(450)
            .build();

        // Present window
        window.present();
    });
    app.run();
}
