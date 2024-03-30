use cursive::{traits::*, views::*, Cursive, Rect};

pub fn show_login(s: &mut Cursive) {
    let input_rect = Rect::from_size((0, 0), (40, 1));
    let label_rect = Rect::from_size((0, 0), (10, 1));

    let make_edit = |name: &str, content: String, secret: bool| -> FixedLayout {
        let mut edit = EditView::new().content(content).filler(" ");
        if secret {
            edit = edit.secret();
        }
        let named_edit = edit.with_name(name);
        FixedLayout::new().child(input_rect, named_edit)
    };

    let make_field_with_secret =
        |name: &str, label: &str, initial_content: String, secret: bool| -> LinearLayout {
            LinearLayout::horizontal()
                .child(FixedLayout::new().child(label_rect, TextView::new(label)))
                .child(make_edit(name, initial_content, secret))
        };

    let make_field = |name: &str, label: &str, initial_content: String| -> LinearLayout {
        make_field_with_secret(name, label, initial_content, false)
    };

    let host = crate::controller::MINERVAHOST.lock().unwrap().clone();
    let layout = LinearLayout::vertical()
        .child(make_field("input_login_host", "Host:", host))
        .child(make_field("input_login_login", "Login:", String::new()))
        .child(make_field_with_secret(
            "input_login_password",
            "Senha:",
            String::new(),
            true,
        ));

    let dialog = Dialog::around(layout)
        .title("Minerva System")
        .button("Entrar", |s| {
            let host: String = match s
                .call_on_name("input_login_host", |view: &mut EditView| view.get_content())
            {
                Some(s) => s,
                None => return,
            }
            .to_string()
            .trim()
            .into();

            if host.is_empty() {
                return;
            }

            *crate::controller::MINERVAHOST.lock().unwrap() = host;

            s.pop_layer();
            super::menubar::show_menubar(s);
        })
        .button("Sair", |s| s.quit());

    s.add_layer(dialog);

    let _ = s.focus_name("input_login_login");
}
