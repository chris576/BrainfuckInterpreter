use std::collections::VecDeque;

pub enum Token {
    Forward,
    Backward,
    Increment,
    Decrement,
    LoopBegin,
    LoopEnd,
    Print,
    No,
}

fn assoziate(command: char) -> Token {
    if command == '>' {
        return Token::Forward;
    } else if command == '<' {
        return Token::Backward;
    } else if command == '+' {
        return Token::Increment;
    } else if command == '-' {
        return Token::Decrement;
    } else if command == '[' {
        return Token::LoopBegin;
    } else if command == ']' {
        return Token::LoopEnd;
    } else if command == '.' {
        return Token::Print;
    }
    return Token::No;
}

pub fn parse(commands: &str) -> Vec<Token> {
    return commands
        .chars()
        .map(|c| assoziate(c))
        .collect::<Vec<Token>>();
}

/**
 * Valid commands:
 * > move right, < move left
 * + Increment - Decrement
 * . output
 * [] loop
 */
pub fn is_valid(commands: &str) -> bool {
    let mut stack: VecDeque<char> = VecDeque::from(['#']);
    for command in commands.chars() {
        if
            command != '>' &&
            command != '<' &&
            command != '+' &&
            command != '-' &&
            command != '[' &&
            command != ']' &&
            command != '.'
        {
            return false;
        }
        if command == '[' {
            stack.push_back('[');
        }
        if command == ']' {
            let last = stack.pop_back();
            if last != Some('[') {
                return false;
            }
        }
    }
    return stack.pop_back() == Some('#');
}
