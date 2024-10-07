use crate::parser::Token;
use crate::parser::TokenType;
use std::cmp::max;
use std::io;
use std::io::Read;

const MEMORY_LEN: usize = 30_000;
const EOF_CHAR: u8 = '&' as u8;
struct State {
    instruction_pointer: usize,
    memory: [u8; MEMORY_LEN],
    data_pointer: usize,
}
pub(crate) fn interpret(tokens: &Vec<Token>) {
    let mut state = State {
        instruction_pointer: 0,
        memory: [0; MEMORY_LEN],
        data_pointer: 0,
    };

    let mut max_memory_access = state.data_pointer;
    let token_len = tokens.len();
    loop {
        let cur_token = &tokens[state.instruction_pointer];

        match (*cur_token).token_type {
            TokenType::IncrementCellPointer => {
                state.data_pointer += 1;
            }
            TokenType::DecrementCellPointer => {
                state.data_pointer -= 1;
            }
            TokenType::Increment => {
                if state.memory[state.data_pointer] == 255 {
                    state.memory[state.data_pointer] = 0;
                } else {
                    state.memory[state.data_pointer] += 1;
                }
            }
            TokenType::Decrement => {
                if state.memory[state.data_pointer] == 0 {
                    state.memory[state.data_pointer] = 255;
                } else {
                    state.memory[state.data_pointer] -= 1;
                }
            }
            TokenType::Output => {
                print!("{}", state.memory[state.data_pointer] as char);
            }
            TokenType::Input => {
                let mut inp_buffer = [0u8; 1];
                io::stdin()
                    .read_exact(&mut inp_buffer)
                    .expect("failed to read inp buffer");
                if inp_buffer[0] == EOF_CHAR {
                    state.memory[state.data_pointer] = 0
                } else {
                    state.memory[state.data_pointer] = inp_buffer[0];
                }
            }
            TokenType::LoopStart => {
                if state.memory[state.data_pointer] == 0 {
                    state.instruction_pointer = cur_token.matching_paren_pos.unwrap();
                }
            }
            TokenType::LoopEnd => {
                if state.memory[state.data_pointer] != 0 {
                    state.instruction_pointer = cur_token.matching_paren_pos.unwrap();
                }
            }
            TokenType::PrintMemory => {
                print_state(&state, token_len, max_memory_access);
            }
        }
        max_memory_access = max(max_memory_access, state.data_pointer);

        state.instruction_pointer += 1;

        if state.instruction_pointer >= token_len {
            log::info!("Finished execution");
            print_state(&state, token_len, max_memory_access);
            return;
        }
    }

    fn print_state(state: &State, token_len: usize, max_memory_access: usize) {
        log::debug!("State:");
        log::debug!("Input len: {}", token_len);
        log::debug!("Instruction Pointer: {}", state.instruction_pointer);
        log::debug!("Data Pointer: {}", state.data_pointer);
        print!("Memory: [");
        for i in 0..=max_memory_access {
            print!("{}, ", state.memory[i]);
        }
        println!("]");
        return;
    }
}
