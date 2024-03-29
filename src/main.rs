mod controller;
mod model;
mod view;

fn main() {
    let mut siv = cursive::default();

    view::menubar::show_menubar(&mut siv);

    siv.run();
}
