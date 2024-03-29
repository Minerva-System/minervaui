mod model;

use cursive::align::HAlign;
use cursive::traits::*;
use cursive::{event::Key, menu, views::*, Cursive, Rect};
use cursive_table_view::TableView;
use lazy_static::lazy_static;
use model::{User, UserListColumn};
use std::sync::Mutex;
use uuid::uuid;

lazy_static! {
    static ref LOADING: Mutex<bool> = Mutex::new(false);
}

fn show_user_list(s: &mut Cursive) {
    let items = vec![
        User {
            id: uuid!("e7a26ab6-ea51-11ee-8525-ea46167f49a2"),
            name: "Fulano de Tal 1".into(),
            login: "fulano1".into(),
            email: Some("fulano1@exemplo.com".into()),
            ..User::default()
        },
        User {
            id: uuid!("e7a26ab6-ea51-11ee-8525-ea46167f49a2"),
            name: "Fulano de Tal 2".into(),
            login: "fulano2".into(),
            email: None,
            ..User::default()
        },
        User {
            id: uuid!("e7a26ab6-ea51-11ee-8525-ea46167f49a2"),
            name: "Fulano de Tal 3 Da Silva, o Consagrado da Galera".into(),
            login: "fulano3".into(),
            email: Some("fulano3@exemplo.com".into()),
            ..User::default()
        },
        User {
            id: uuid!("e7a26ab6-ea51-11ee-8525-ea46167f49a2"),
            name: "Fulano de Tal 4".into(),
            login: "fulano4".into(),
            email: None,
            ..User::default()
        },
        User {
            id: uuid!("e7a26ab6-ea51-11ee-8525-ea46167f49a2"),
            name: "Fulano de Tal 5".into(),
            login: "fulano5".into(),
            email: Some("fulano5@exemplo.com".into()),
            ..User::default()
        },
    ];

    let show_list = move |s: &mut Cursive| {
        let size = (100, 20);

        let mut table = TableView::<User, UserListColumn>::new()
            .column(UserListColumn::Login, "Login", |c| c.width_percent(10))
            .column(UserListColumn::Name, "Nome", |c| c.width_percent(20))
            .column(UserListColumn::Email, "E-mail", |c| c)
            .column(UserListColumn::ID, "UUID", |c| {
                c.align(HAlign::Left).width_percent(38)
            });

        table.set_items(items);

        table.set_on_submit(|s: &mut Cursive, _row: usize, index: usize| {
            let mut user = None;
            s.call_on_name(
                "user_table",
                |table: &mut TableView<User, UserListColumn>| {
                    user = Some(table.borrow_item(index).unwrap().clone());
                },
            )
            .unwrap();

            show_user_form(s, user);
        });

        s.add_layer(
            Dialog::around(table.with_name("user_table").min_size(size))
                .title("Lista de Usuários")
                .button("Fechar", |s| {
                    s.pop_layer();
                }),
        );
    };

    // Loading screen
    *LOADING.lock().unwrap() = true;
    s.add_layer(
        Dialog::around(
            TextView::new("Carregando...")
                .h_align(HAlign::Center)
                .min_size((20, 1)),
        )
        .with_name("loading_user_list_view"),
    );

    // Get sink
    let cb = s.cb_sink().clone();

    // Spawn thread
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(3000));
        cb.send(Box::new(|s| {
            if s.focus_name("loading_user_list_view").is_ok() {
                s.pop_layer();
                s.set_autohide_menu(false);
            }
            // s.pop_layer();
        }))
        .unwrap();

        *LOADING.lock().unwrap() = false;
        cb.send(Box::new(show_list)).unwrap();
    });

    // //std::thread::sleep(std::time::Duration::from_millis(2000));
    // s.pop_layer();
}

fn show_user_form(s: &mut Cursive, user: Option<User>) {
    let input_rect = Rect::from_size((0, 0), (40, 1));
    let label_rect = Rect::from_size((0, 0), (10, 1));

    let make_info = |label: &str, content: &str| -> LinearLayout {
        LinearLayout::horizontal()
            .child(FixedLayout::new().child(label_rect, TextView::new(label)))
            .child(FixedLayout::new().child(input_rect, TextView::new(content)))
    };

    let make_edit = |content: String, secret: bool| -> FixedLayout {
        let mut edit = EditView::new().content(content).filler(" ");
        edit.set_secret(secret);
        FixedLayout::new().child(input_rect, edit)
    };

    let make_field_with_secret =
        |label: &str, initial_content: String, secret: bool| -> LinearLayout {
            LinearLayout::horizontal()
                .child(FixedLayout::new().child(label_rect, TextView::new(label)))
                .child(make_edit(initial_content, secret))
        };

    let make_field = |label: &str, initial_content: String| -> LinearLayout {
        make_field_with_secret(label, initial_content, false)
    };

    let layout = match user {
        Some(user) => {
            let uuid_str = user.id.to_string();
            LinearLayout::vertical()
                .child(make_info("UUID:", &uuid_str))
                .child(make_info("Login:", &user.login))
                .child(make_field("Nome:", user.name))
                .child(make_field("E-mail:", user.email.unwrap_or(String::new())))
        }
        None => LinearLayout::vertical()
            // .child(make_info("UUID:", "<gerado automaticamente>"))
            .child(make_field("Login:", String::new()))
            .child(make_field("Nome:", String::new()))
            .child(make_field("E-mail:", String::new()))
            .child(make_field_with_secret("Senha:", String::new(), true)),
    };

    s.add_layer(
        Dialog::around(layout)
            .title("Edição de Usuário")
            .button("Salvar", |_| {})
            .button("Remover", |_| {})
            .button("Cancelar", |s| {
                s.pop_layer();
            }),
    );
}

fn main() {
    let mut siv = cursive::default();

    siv.menubar()
        .add_subtree(
            "Ações",
            menu::Tree::new()
                .leaf("Sobre o Minerva System...", |s| {
                    s.add_layer(Dialog::info(
                        "Minerva System v0.0.0\nCopyright (c) 2024 Lucas S. Vieira",
                    ));
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
                            show_user_list(s);
                        })
                        .leaf("Cadastrar usuários...", |s| {
                            show_user_form(s, None);
                        }),
                )
                .leaf("Produtos...", |_s| {}),
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
    use cursive::event::{EventResult, EventTrigger};
    siv.set_on_pre_event_inner(EventTrigger::any(), |_| {
        if *LOADING.lock().unwrap() {
            Some(EventResult::Ignored)
        } else {
            None
        }
    });

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    siv.run();
}
