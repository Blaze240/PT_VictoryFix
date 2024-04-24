#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
	unused_macros,
	unused_variables,
	unused_assignments,
	unused_unsafe,
	non_upper_case_globals,
	non_snake_case,
    clippy::borrow_interior_mutable_const
)]

mod win1a_fushigisou;
mod win1a_lizardon;
mod win1a_zenigame;
mod win1b_fushigisou;
mod win1b_lizardon;
mod win1b_zenigame;

mod win2a_fushigisou;
mod win2a_lizardon;
mod win2a_zenigame;
mod win2b_fushigisou;
mod win2b_lizardon;
mod win2b_zenigame;

mod win3a_fushigisou;
mod win3a_lizardon;
mod win3a_zenigame;
mod win3b_fushigisou;
mod win3b_lizardon;
mod win3b_zenigame;

#[skyline::main(name = "ptrainer_victoryfix_SL2")]
pub fn main() {
    win1a_fushigisou::install();
    win1a_lizardon::install();
    win1a_zenigame::install();
    win1b_fushigisou::install();
    win1b_lizardon::install();
    win1b_zenigame::install();

    win2a_fushigisou::install();
    win2a_lizardon::install();
    win2a_zenigame::install();
    win2b_fushigisou::install();
    win2b_lizardon::install();
    win2b_zenigame::install();

    win3a_fushigisou::install();
    win3a_lizardon::install();
    win3a_zenigame::install();
    win3b_fushigisou::install();
    win3b_lizardon::install();
    win3b_zenigame::install();
}