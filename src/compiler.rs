use std::{
    fmt::Write,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::{parser::Action, token::Basic, IoKind};

const STD: &str = include_str!("std.asm");

pub fn compile(code: &[Action], io: &IoKind) -> String {
    let main = compile_impl(code, io, &AtomicUsize::new(0));
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

fn compile_impl(code: &[Action], io: &IoKind, id_counter: &AtomicUsize) -> String {
    let mut asm = String::new();
    for action in code {
        match action {
            Action::Basic(Basic::MoveRight) => writeln!(asm, "inc rbx"),
            Action::Basic(Basic::MoveLeft) => writeln!(asm, "dec rbx"),
            Action::Basic(Basic::Increment) => writeln!(asm, "inc byte [rbx]"),
            Action::Basic(Basic::Decrement) => writeln!(asm, "dec byte [rbx]"),
            Action::Basic(Basic::Output) => writeln!(asm, "call output_{io}"),
            Action::Basic(Basic::Input) => writeln!(asm, "call input_{io}"),
            Action::Loop(inner) => {
                let id = id_counter.fetch_add(1, Ordering::Relaxed);
                let inner_asm = compile_impl(inner, io, id_counter);
                writeln!(
                    asm,
                    "\
.loop{id}_start:
cmp byte [rbx], 0
je .loop{id}_exit
{inner_asm}\
jmp .loop{id}_start
.loop{id}_exit:
"
                )
            }
        }
        .expect("writing shouldn't fail");
    }
    asm
}
