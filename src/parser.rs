use std::process::exit;

use log::error;

#[derive(Debug)]
pub enum TokenType {
    IncrementCellPointer,
    DecrementCellPointer,
    Increment,
    Decrement,
    Output,
    Input,
    LoopStart,
    LoopEnd,
    PrintMemory,
    // Ignoring comments
}

pub struct Token {
    pub token_type: TokenType,
    pub matching_paren_pos: Option<usize>,
}

struct MatchingParen {
    index: usize,
    token_type: TokenType,
}

pub(crate) fn tokenize(inp_string: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::with_capacity(inp_string.len());

    let mut paren_stack: Vec<MatchingParen> = Vec::new();

    let mut i = 0;

    for inp_char in inp_string.chars() {
        let token_type: Option<TokenType> = match inp_char {
            '>' => Some(TokenType::IncrementCellPointer),
            '<' => Some(TokenType::DecrementCellPointer),
            '+' => Some(TokenType::Increment),
            '-' => Some(TokenType::Decrement),
            '.' => Some(TokenType::Output),
            ',' => Some(TokenType::Input),
            '[' => Some(TokenType::LoopStart),
            ']' => Some(TokenType::LoopEnd),
            '#' => Some(TokenType::PrintMemory),
            _ => None,
        };

        if token_type.is_some() {
            let mut cur_matching_paren: Option<usize> = None;
            match token_type.as_ref().unwrap() {
                TokenType::LoopStart => {
                    paren_stack.push(MatchingParen {
                        index: i,
                        token_type: TokenType::LoopStart,
                    });
                }
                TokenType::LoopEnd => {
                    cur_matching_paren =
                        try_get_matching_paren_pos(&mut paren_stack, &mut tokens, i);
                }
                _ => {}
            }
            tokens.push(Token {
                token_type: token_type.unwrap(),
                matching_paren_pos: cur_matching_paren,
            });
            i += 1;
        }
    }
    if !paren_stack.is_empty() {
        let unmatched_brac = paren_stack.pop().expect("failed to pop");
        error!(
            "Unmatched bracket {:?} at {} ",
            unmatched_brac.token_type, unmatched_brac.index
        );
        exit(1);
    }
    tokens
}

fn try_get_matching_paren_pos(
    paren_stack: &mut Vec<MatchingParen>,
    tokens: &mut Vec<Token>,
    cur_index: usize,
) -> Option<usize> {
    let matching_paren = paren_stack.pop();

    if matching_paren.is_none() {
        error!("Unmatched closing bracket at {} ", cur_index);
        exit(1);
    }

    let matching_paren = matching_paren.unwrap();

    match matching_paren.token_type {
        TokenType::LoopStart => {
            tokens[matching_paren.index].matching_paren_pos = Some(cur_index);
            Some(matching_paren.index)
        }
        _ => {
            error!(
                "Unmatched closing bracket at index {}, instead found {:?} at pos {}",
                cur_index, matching_paren.token_type, matching_paren.index
            );
            exit(1);
        }
    }
}
