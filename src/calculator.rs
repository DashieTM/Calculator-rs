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

use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
pub struct Calculator {
    pub entry: String,
    pub result: f64,
    pub expect_number: bool,
    pub current: String,
    pub tokens: Vec<String>,
    pub next_char: char,
    pub variables: HashMap<String, f64>,
    pub toml_path: String,
}

pub enum ErrorMessages {
    ZeroDivisionError,
    NotANumberError,
    NotAnOperatorError,
    BrackedError,
    TangentOutOfScope,
    NegativeLogError,
    NegativeFactorialError,
}

impl Calculator {
    pub fn new() -> Self {
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

    pub fn handle_expression(&mut self) -> Result<f64, ErrorMessages> {
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
                }
                '-' => {
                    self.next();
                    self.expect_number = true;
                    result -= self.handle_term().ok().unwrap();
                }
                _ => return Err(ErrorMessages::NotAnOperatorError),
            }
        }
        Ok(result)
    }

    pub fn handle_term(&mut self) -> Result<f64, ErrorMessages> {
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
                let maybe_div = self.handle_factor();
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
                if is_operator(self.next_char) {
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

    pub fn handle_factor(&mut self) -> Result<f64, ErrorMessages> {
        let mut result = 0.0;
        if self.current == "(" {
            self.next();
            let maybe_result = self.handle_expression();
            if maybe_result.is_err() {
                return maybe_result;
            }
            if self.current != ")" {
                return Err(ErrorMessages::BrackedError);
            }
            self.next();
            self.expect_number = false;
            return Ok(maybe_result.ok().unwrap());
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
            self.next();
            let maybe_result = self.handle_expression();
            if maybe_result.is_err() {
                return maybe_result;
            }
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

    pub fn handle_specials(&mut self) -> Result<f64, ErrorMessages> {
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
                    return Err(ErrorMessages::NegativeLogError);
                }
                return Ok(num.log(logbase));
            }
        }
    }

    pub fn next(&mut self) {
        if self.has_next() {
            self.current = self.tokens.remove(0);
            return;
        }
        self.current = String::from("");
    }

    pub fn next_char(&mut self) -> bool {
        let next_char = self.current.chars().next();
        if !next_char.is_some() {
            return false;
        }
        self.next_char = next_char.unwrap();
        return true;
    }

    pub fn has_next(&self) -> bool {
        if self.tokens.len() > 0 {
            return true;
        }
        return false;
    }

    pub fn read_toml(&mut self) {
        let content = match fs::read_to_string(&self.toml_path) {
            Ok(c) => c,
            Err(_) => panic!("Could not open variables.toml"),
        };
        self.variables = match toml::from_str(&content) {
            Ok(c) => c,
            Err(err) => panic!("error converting toml: {}", err),
        };
    }

    pub fn write_toml(&self) {
        let toml_string =
            toml::to_string(&self.variables).expect("Error parsing variables to toml");
        fs::write(&self.toml_path, toml_string).expect("Could not write to toml");
    }

    pub fn parse_tokens(&mut self) {
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

pub fn split_string(str: &String) -> Vec<String> {
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
        if is_operator(chr) || chr == '!' {
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

pub fn negative_clean(tokens: &mut Vec<String>) {
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

pub fn negate(num: f64) -> f64 {
    return -num;
}

pub fn is_negative(num: f64) -> bool {
    if num < 0.0 {
        return true;
    }
    false
}

pub fn is_operator(chr: char) -> bool {
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

pub fn is_special(str: &String) -> bool {
    match str.as_str() {
        "sin" => true,
        "cos" => true,
        "tan" => true,
        "log" => true,
        _ => false,
    }
}

pub fn factorial(num: i64) -> Result<f64, ErrorMessages> {
    if num > 1 {
        let maybe_fact = factorial(num - 1);
        if maybe_fact.is_err() {
            return Err(ErrorMessages::NotANumberError);
        }
        return Ok((num * maybe_fact.ok().unwrap() as i64) as f64);
    }
    if num < 0 {
        return Err(ErrorMessages::NegativeFactorialError);
    }
    Ok(1.0)
}

pub fn exponential(base: f64, power: f64) -> Result<f64, ErrorMessages> {
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
