use std::sync::atomic::{AtomicUsize, Ordering};

use crate::{parser::Action, token::Basic};

const STD: &str = include_str!("std.asm");

pub fn compile(code: &[Action]) -> String {
    let main = compile_impl(code, &AtomicUsize::new(0));
    format!(
        "\
{STD}
section .text
run:
{main}\
ret
"
    )
}

fn compile_impl(code: &[Action], id_counter: &AtomicUsize) -> String {
    let mut asm = String::new();
    for action in code {
        match action {
            Action::Basic(b) => asm.push_str(match b {
                Basic::MoveRight => "inc rbx\n",
                Basic::MoveLeft => "dec rbx\n",
                Basic::Increment => "inc byte [rbx]\n",
                Basic::Decrement => "dec byte [rbx]\n",
                Basic::Output => "call output\n",
                Basic::Input => "call input\n",
            }),
            Action::Loop(inner) => {
                let id = id_counter.fetch_add(1, Ordering::Relaxed);
                let inner_asm = compile_impl(inner, id_counter);
                asm.push_str(&format!(
                    "\
.loop{id}_start:
cmp byte [rbx], 0
je .loop{id}_exit
{inner_asm}\
jmp .loop{id}_start
.loop{id}_exit:
"
                ));
            }
        };
    }
    asm
}
