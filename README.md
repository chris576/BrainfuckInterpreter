# Brainfuck Interpreter

## Cargo cheat sheet

* Build a cargo project: `cargo build`
* Run: `echo ">++." | ./target/debug/BrainfuckInterpreter`

--- 

## About

This is a modern Implementation of `Brainfuck` written in Rust. Syntax is very simple an clear.
* At startup you get an array of u8 in size of 10.000
* `>` Switch to the left byte.
* `<` Switch to the right element.
* `+` Increment the cell.
* `-` Decrement the cell.
* `.` Print to the std:out

---

## Requirements

* You had to have installed cargo and rust installed. 
* Clone this project. 
* The main branch is protected, unsless you have administration-rights.
* In order to make your solutions accessible for anyone, I would assume that you are opening a new branch. Naming could be as follows: 
  * `your-names+task_name`
* Afterword I would like you to push your branch to this repo.

---

## Tasks

In order to learn more effectivly, organize yourself in groups of 1 to 3. The tasks are categorizes from lover, to middle, to higher level programming challanges. Feel free to solve as many challanges, as you can take
within 1 hour. 

### Input ( Easy mode )

In the original implementation of Brainfuck, there was the ability to take an input from std:in. Implement this functionality to the interpreter.
The functionality should fulfill the following requirements: 
* `,` added as parsable `Token` to the `parser` module (crate).
* On processing `,` the thread waits until the user has given some input.
* Any input is valid input, as long it is representable as `u8`.

### Array overflow ( Easy mode )

Currently an overflow on the byte array is not handled. Be creative and think about how to handle the case of `memory_ptr` going out of scope.

### While - Loop ( Heavy mode )

In the original implementation of Brainfuck, there was an while loop. I consider the original approach is a little bit out of date, so I assume, you would like do implement a little bit more modern approach.

I would assume the following approach for this task: 
* `[` starts the loop.
* `]` ends the loop.
* `;` divides the loop between header and body.
* `>++.;` Could be an example for the header section. The loop will be executed 2 times plus the value of the left cell.
* `<.>+` In any run of the loop:
  * The pointer is moved to right.
  * Output of the current value.
  * Move to right.
  * Increment.

### Byte interpretation ( Evil mode )

On std:in and std:out an `u8` could be interpreted differently. For example in some cases I'd like to have chars on the `std:out`. 
I would assume, that this functionality could be implemented by adding switches to the command. Fo example
* `-in:type` in type of `char` or `int`, or `bin`
* Literals could be implemented for different representatioons of `int`
* Same for `std:in`

### File-Input ( Medium Mode )

In this task you shall make it possible, to make a `.bf` (Text) file parsable and runnable by the interpreter. 
A new switch could be defined ( like `-in=`) that defines the input `.bf` file
