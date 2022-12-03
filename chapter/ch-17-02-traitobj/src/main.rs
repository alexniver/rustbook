use ch_17_02_traitobj::*;

fn main() {
    let mut screen = Screen::new();
    screen.add_comp(Box::new(Button::new(2, 2, String::from("abc"))));
    screen.add_comp(Box::new(SelectBox::new(3, 3, vec![String::from("a")])));

    screen.run();
}
