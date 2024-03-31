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
                        .leaf("Ver usuários...", |s| {
                            super::user::show_user_list(s);
                        })
                        .leaf("Novo usuário...", |s| {
                            super::user::show_user_form(s, None);
                        }),
                )
                .subtree(
                    "Produtos",
                    menu::Tree::new()
                        .leaf("Ver produtos...", |s| {
                            super::products::show_product_list(s);
                        })
                        .leaf("Novo produto...", |s| {
                            super::products::show_product_form(s, None);
                        }),
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
        )
        .add_leaf("PDV", |s| {
            super::pdv::show_pdv(s);
        });
    siv.set_autohide_menu(false);

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
}

fn show_about(s: &mut Cursive) {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let license = env!("CARGO_PKG_LICENSE");
    let authors = env!("CARGO_PKG_AUTHORS");
    s.add_layer(Dialog::info(format!(
        "Minerva System v{} — {}\nFront-End para Console\n\nCopyright (c) 2024 {}\nDistribuído sob a licença {}.",
        version, name, authors, license
    )));
}
