mod arrays;
mod conditionals;
mod enums;
mod functions;
mod loops;
mod ownership_borrowing;
mod pointer_ref;
mod strings;
mod structs;
mod tuples;
mod types;
mod vars;
mod vectors;

fn main() {
    vars::run();
    types::run();
    strings::run();
    tuples::run();
    conditionals::run();
    arrays::run();
    vectors::run();
    loops::run();
    functions::run();
    enums::run();
    structs::run();
    pointer_ref::run();
    ownership_borrowing::run();
}
