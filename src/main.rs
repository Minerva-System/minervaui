mod controller;
mod model;
mod view;

fn main() {
    let mut siv = cursive::default();

    view::login::show_login(&mut siv);

    siv.run();
}
