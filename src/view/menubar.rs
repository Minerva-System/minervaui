use cursive::{event::Key, menu, views::Dialog, Cursive};

pub fn show_menubar(siv: &mut Cursive) {
    siv.menubar()
        .add_subtree(
            "Ações",
            menu::Tree::new()
                .leaf("Sobre o Minerva System...", |s| {
                    show_about(s);
                })
                .delimiter()
                .leaf("Sair", |s| s.quit()),
        )
        .add_subtree(
            "Cadastros",
            menu::Tree::new()
                .subtree(
                    "Usuários",
                    menu::Tree::new()
                        .leaf("Lista de usuários...", |s| {
                            s.set_autohide_menu(true);
                            super::user::show_user_list(s);
                        })
                        .leaf("Cadastrar usuários...", |s| {
                            super::user::show_user_form(s, None);
                        }),
                )
                .subtree(
                    "Produtos",
                    menu::Tree::new()
                        .leaf("Lista de produtos...", |_s| {})
                        .leaf("Cadastrar produtos...", |_s| {}),
                ),
        )
        .add_subtree(
            "Relatórios",
            menu::Tree::new().subtree(
                "Curvas ABC",
                menu::Tree::new()
                    .leaf("Curva ABC de Produtos...", |_s| {})
                    .leaf("Curva ABC de Clientes...", |_s| {}),
            ),
        );
    siv.set_autohide_menu(false);
    // use cursive::event::{EventResult, EventTrigger};
    // siv.set_on_pre_event_inner(EventTrigger::any(), |_| {
    //     if *LOADING.lock().unwrap() {
    //         Some(EventResult::Ignored)
    //     } else {
    //         None
    //     }
    // });

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
}

fn show_about(s: &mut Cursive) {
    s.add_layer(Dialog::info(
        "Minerva System v0.0.0\nCopyright (c) 2024 Lucas S. Vieira",
    ));
}
