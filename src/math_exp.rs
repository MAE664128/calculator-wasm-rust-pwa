use crate::token;
use crate::token::Weight;


pub struct MathExp {
    tokens: Vec<token::Token>,
    buffer: String,
    output: String,
}

impl Default for MathExp {
    fn default() -> Self {
        Self::new()
    }
}

impl MathExp {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            buffer: String::new(),
            output: String::new(),
        }
    }

    pub fn get_output(&self) -> String {
        self.output.clone()
    }

    fn update_output(&mut self, s: &str) {
        self.output = s.to_string();
    }

    /// Вставка токенов с учетом правил:
    ///
    /// * После Операнда (число) обязательно должен следовать знак операции или знак закрывающейся скобки.
    /// * После знака операции может быть только знак открывающейся скобки или Операнд (число).
    fn push_to_token(&mut self, t: token::Token) {
        fn push(tokens: &mut Vec<token::Token>, t: token::Token) {
            if let token::Token::Function(_) = t {
                tokens.push(t);
                tokens.push(token::Token::Operation(token::Op::ParenLeft));
            } else { tokens.push(t); }
        }
        // Запрещаем вставлять закрывающуюся скобку,
        // если их количество после вставке будет превышать количество открывающихся скобок.
        if let token::Token::Operation(token::Op::ParenRight) = t {
            let mut count_paren: i32 = -1; // Устанавливаем в -1, т.к. в будущем мы хотим вставить одну скобку.
            for token in &self.tokens {
                if let token::Token::Operation(p) = token {
                    match p {
                        token::Op::ParenLeft => { count_paren += 1 }
                        token::Op::ParenRight => { count_paren -= 1 }
                        _ => {}
                    }
                }
            }
            if count_paren < 0 { return; }
        }


        let last_token = self.tokens.last();
        if last_token.is_none() {
            // Когда список токенов пустой,
            // мы будем разрешать вставку новых токенов только
            // если они не являются токенами операции (за исключение открывающейся скобки).
            if !matches!(t,token::Token::Operation(_)) || matches!(t,token::Token::Operation(token::Op::ParenLeft)) {
                push(&mut self.tokens, t);
            }
            return;
        }
        let last_token = last_token.unwrap();


        let allow_insert = match last_token {
            // После числа:
            token::Token::Operand(_) => {
                match t {
                    // Запрещаем вставку функции, чисел или левой скобки после числа.
                    token::Token::Function(_) | token::Token::Operand(_) | token::Token::Operation(token::Op::ParenLeft) => { false }
                    _ => { true }
                }
            }
            // После закрывающейся скобки:
            token::Token::Operation(token::Op::ParenRight) => {
                match t {
                    token::Token::Operation(token::Op::ParenLeft) => { false }
                    token::Token::Operation(_) => { true }
                    _ => { false }
                }
            }
            // После операции кроме закрывающейся скобки:
            token::Token::Operation(_) => {
                match t {
                    token::Token::Operation(token::Op::ParenLeft) => { true }
                    // Запрещаем вставку операций после операции (исключение открывающаяся скобка).
                    token::Token::Operation(_) => { false }
                    _ => { true }
                }
            }

            _ => { true }
        };

        if allow_insert {
            push(&mut self.tokens, t);
        } else {
            self.update_output(format!("Токен {} не может быть добавлен после {}", t, last_token).as_str());
        }
        let mut s = String::new();
        for token in &self.tokens {
            s.push('[');
            s.push_str(token.to_string().as_str());
            s.push(']');
            s.push(',');
        }
    }

    /// Удалить значение из буфера и поместить его в конец вектора с токенами.
    ///
    /// Будет выполнена попытка преобразовать значение хранящиеся в буфере.
    ///
    fn pop_buffer(&mut self) -> bool {
        if self.buffer.is_empty() { return true; }
        if let Ok(val) = self.buffer.parse::<f64>() {
            if val.is_sign_negative() {
                self.tokens.push(token::Token::Operation(token::Op::ParenLeft));
                self.tokens.push(token::Token::Operand(val));
                self.tokens.push(token::Token::Operation(token::Op::ParenRight));
                self.buffer.clear();
            } else {
                self.tokens.push(token::Token::Operand(val));
                self.buffer.clear();
            }
            true
        } else { false }
    }

    /// Удалить последнее значение из вектора с токенами.
    pub fn pop(&mut self) {
        if self.buffer.is_empty() {
            self.tokens.pop();
            // Если после удаленного токена стоял токен функции, то мы удаляем и его.
            if matches!(self.tokens.last(), Some(token::Token::Function(_))) { self.tokens.pop(); }
        } else {
            // Если у нас есть значения в буфере, то мы сначала удаляем значения из него.
            self.buffer.pop();
        }
    }

    /// Очистить буфера и вектор с токенами.
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.tokens.clear();
    }

    /// Создать и добавить новый токен используя указанную строку.
    ///
    /// При любом добавление сначала будет выполняться проверка необходимости
    /// добавления строки в буфер.
    /// Если поступает последовательность символов, которая может интерпретироваться
    /// как операция или функция, то будет выполнена попытка выдавить текущее значение
    /// из буфера и только после будет выполнено добавление нового значения.
    pub fn add(&mut self, s: &str) -> bool {
        let allow_number_input = !matches!(
            self.tokens.last(),
            Some(token::Token::Operation(token::Op::ParenRight))
        );

        if s == "." && allow_number_input {
            if self.buffer.is_empty() {
                self.buffer = "0.".to_string();
                true
            } else if self.buffer.contains('.') {
                // мы не можем разрешить добавить больше чем одну точку.
                false
            } else {
                self.buffer.push('.');
                true
            }
        } else if s.parse::<u8>().is_ok() && allow_number_input {
            self.buffer.push_str(s);
            true
        } else if self.buffer.is_empty() && s == "-" && allow_number_input {
            self.buffer = s.to_string();
            true
        } else if let Ok(t) = token::Token::try_from(s) {
            self.pop_buffer();
            self.push_to_token(t);
            true
        } else { false }
    }

    pub fn calculate(&mut self) {
        self.pop_buffer();
        let rpn = yard(&self.tokens);
        match rpn {
            Err(e) => { self.output = e }
            Ok(tokens) => {
                let mut stack: Vec<token::Token> = Vec::new();
                for t in tokens {
                    if stack.is_empty() {
                        stack.push(t);
                    } else {
                        match t {
                            token::Token::Function(f) => {
                                if let Some(token::Token::Operand(val)) = stack.pop() {
                                    stack.push(match f {
                                        token::Func::Sin => {
                                            token::Token::Operand(val.sin())
                                        }
                                        token::Func::Cos => {
                                            token::Token::Operand(val.cos())
                                        }
                                        token::Func::Tg => {
                                            token::Token::Operand(val.sin() / val.cos())
                                        }
                                        token::Func::Ctg => {
                                            token::Token::Operand(val.cos() / val.sin())
                                        }
                                        token::Func::Sqrt => {
                                            token::Token::Operand(val.sqrt())
                                        }
                                    });
                                } else {
                                    self.output = "Ошибка вычисления".to_string()
                                }
                            }
                            token::Token::Operation(op) => {
                                if let Some(token::Token::Operand(second_val)) = stack.pop() {
                                    if let Some(token::Token::Operand(first_val)) = stack.pop() {
                                        if let Some(val) = match op {
                                            token::Op::Add => {
                                                Some(first_val + second_val)
                                            }
                                            token::Op::Sub => {
                                                Some(first_val - second_val)
                                            }
                                            token::Op::Multi => {
                                                Some(first_val * second_val)
                                            }
                                            token::Op::Div => {
                                                Some(first_val / second_val)
                                            }
                                            token::Op::Exp => {
                                                Some(first_val.powf(second_val))
                                            }
                                            _ => { None }
                                        } { stack.push(token::Token::Operand(val)) }
                                    } else { self.output = "Ошибка вычисления".to_string() }
                                } else { self.output = "Ошибка вычисления".to_string() }
                            }
                            token::Token::Operand(_) => { stack.push(t); }
                        }
                    }
                }
                self.output = if let Some(t) = stack.pop() {
                    self.buffer.clear();
                    self.tokens.clear();
                    t.to_string()
                } else { "Ошибка вычисления".to_string() };
            }
        }
    }
}

impl std::fmt::Display for MathExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut o = String::new();
        for token in &self.tokens {
            o.push_str(token.to_string().as_str());
            o.push(' ');
        }
        o.push_str(self.buffer.as_str());
        write!(
            f,
            "{}",
            o
        )
    }
}


/// # Алгоритм сортировочной станции
///
/// [Материал из Википедии — свободной энциклопедии](https://en.wikipedia.org/wiki/Shunting_yard_algorithm)
///
/// Алгоритм сортировочной станции — способ разбора математических выражений,
/// представленных в инфиксной нотации. Может быть использован для получения вывода в виде
/// обратной польской нотации или в виде абстрактного синтаксического дерева.
/// Алгоритм предложен Эдсгером Дейкстрой и назван им «алгоритм сортировочной станции»,
/// поскольку напоминает действие железнодорожной сортировочной станции.
///
/// Так же, как и вычисление значений выражений в обратной польской записи,
/// алгоритм работает при помощи стека. Инфиксная запись математических выражений чаще всего
/// используется людьми, её примеры: 2+4 и 3+6*(3-2). Для преобразования в обратную польскую
/// нотацию используется 2 строки: входная и выходная, и стек для хранения операторов,
/// ещё не добавленных в выходную очередь. При преобразовании алгоритм считывает 1 символ и
/// производит действия, зависящие от данного символа.
fn yard(input: &Vec<token::Token>) -> Result<Vec<token::Token>, String> {
    let mut output: Vec<token::Token> = vec![];
    let mut stack: Vec<token::Token> = vec![];
    for token in input {
        match token {
            token::Token::Operand(_o) => {
                // Если токен — число, то добавить его в очередь вывода.
                output.push(token.clone())
            }
            token::Token::Function(_f) => {
                stack.push(token.clone())
            }
            token::Token::Operation(token::Op::ParenLeft) => {
                stack.push(token.clone())
            }
            token::Token::Operation(token::Op::ParenRight) => {
                loop {
                    if let Some(last_token_in_stack) = stack.pop() {
                        match last_token_in_stack {
                            token::Token::Operation(token::Op::ParenLeft) => {
                                break;
                            }
                            _ => {
                                output.push(last_token_in_stack.clone())
                            }
                        }
                    } else {
                        return Err("В выражении отсутствует скобка.".to_string());
                    }
                }
            }
            token::Token::Operation(op1) => {
                if let Some(token::Token::Operation(op2)) = stack.pop() {
                    if op2.weight() >= op1.weight() {
                        output.push(token::Token::Operation(op2))
                    } else {
                        stack.push(token::Token::Operation(op2))
                    }
                }

                stack.push(token.clone())
            }
        }
    }
    while let Some(last_token_in_stack) = stack.pop() {
        match last_token_in_stack {
            token::Token::Operation(token::Op::ParenLeft) => {
                return Err("В выражении отсутствует скобка.".to_string());
            }
            _ => { output.push(last_token_in_stack) }
        }
    }
    Ok(output)
}