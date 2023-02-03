use console::Term;

pub fn print_brackets(x: usize, y: usize, term: &Term) {
    term.move_cursor_to(x * 2, y).unwrap();
    print!("[");
    term.move_cursor_right(1).unwrap();
    print!("]");
    term.move_cursor_right(1).unwrap();
}

pub fn clear_brackets(term: &Term) {
    term.move_cursor_left(4).unwrap();
    print!(" ");
    term.move_cursor_right(1).unwrap();
    print!(" ");
}

pub fn init_term() -> Term {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    term.hide_cursor().unwrap();

    term
}

pub fn cleanup_term(term: Term) {
    term.show_cursor().unwrap();
    term.clear_screen().unwrap();
}

pub fn await_input(term: &Term) -> char {
    term.read_char().unwrap()
}
