use crate::parser::Token;

/**
 *
 */
pub fn run(
    byte_array: &mut [u8; 10000],
    commands: &Vec<Token>,
    memory_ptr: usize,
    command_ptr: usize,
    loop_back_ptr: Option<usize>,
) {
    if command_ptr >= commands.len() {
        return ();
    }
    match commands[command_ptr] {
        Token::Forward => run(
            byte_array,
            commands,
            memory_ptr + 1,
            command_ptr + 1,
            loop_back_ptr,
        ),
        Token::Backward => run(
            byte_array,
            commands,
            memory_ptr - 1,
            command_ptr + 1,
            loop_back_ptr,
        ),
        Token::Increment => {
            byte_array[memory_ptr] = byte_array[memory_ptr].wrapping_add(1);
            run(
                byte_array,
                commands,
                memory_ptr,
                command_ptr + 1,
                loop_back_ptr,
            );
        }
        Token::Decrement => {
            byte_array[memory_ptr] = byte_array[memory_ptr].wrapping_sub(1);
            run(
                byte_array,
                commands,
                memory_ptr,
                command_ptr + 1,
                loop_back_ptr,
            );
        }
        Token::LoopBegin => run(
            byte_array,
            commands,
            memory_ptr,
            command_ptr + 1,
            Some(command_ptr),
        ),
        Token::LoopEnd => {
            if byte_array[memory_ptr] != 0 {
                run(
                    byte_array,
                    commands,
                    memory_ptr,
                    loop_back_ptr.expect(""),
                    loop_back_ptr,
                );
            } else {
                run(byte_array, commands, memory_ptr, command_ptr + 1, None);
            }
        }
        Token::Print => {
            print!("{}", byte_array[memory_ptr]);
            run(
                byte_array,
                commands,
                memory_ptr,
                command_ptr + 1,
                loop_back_ptr,
            );
        }
        Token::No => {}
    }
}
