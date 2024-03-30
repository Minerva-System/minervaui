use super::common::CommonColumn;
use crate::model::user::User;
use cursive::{align::HAlign, traits::*, views::*, Cursive, Rect};
use cursive_table_view::TableView;

pub fn show_user_form(s: &mut Cursive, user: Option<User>) {
    let input_rect = Rect::from_size((0, 0), (40, 1));
    let label_rect = Rect::from_size((0, 0), (10, 1));

    let make_info = |name: &str, label: &str, content: &str| -> LinearLayout {
        LinearLayout::horizontal()
            .child(FixedLayout::new().child(label_rect, TextView::new(label)))
            .child(FixedLayout::new().child(input_rect, TextView::new(content).with_name(name)))
    };

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

    let editing = user.is_some();

    let layout = match user {
        Some(user) => {
            let user_id = user.id.to_string();
            LinearLayout::vertical()
                .child(make_info("info_user_uuid", "UUID:", &user_id))
                .child(make_info("info_user_login", "Login:", &user.login))
                .child(make_field("input_user_name", "Nome:", user.name))
                .child(make_field(
                    "input_user_email",
                    "E-mail:",
                    user.email.unwrap_or(String::new()),
                ))
        }
        None => LinearLayout::vertical()
            .child(make_info(
                "info_user_uuid",
                "UUID:",
                "(será gerado automaticamente)",
            ))
            .child(make_field("input_user_login", "Login:", String::new()))
            .child(make_field("input_user_name", "Nome:", String::new()))
            .child(make_field("input_user_email", "E-mail:", String::new()))
            .child(make_field_with_secret(
                "input_user_password",
                "Senha:",
                String::new(),
                true,
            )),
    };

    let mut dialog = Dialog::around(layout).title(if editing {
        "Edição de Usuário"
    } else {
        "Novo Usuário"
    });

    if editing {
        dialog.add_button("Salvar", |_| {});
        dialog.add_button("Remover", |s| {
            s.add_layer(
                Dialog::around(TextView::new("Deseja realmente remover este usuário?"))
                    .button("Não", |s| {
                        s.pop_layer();
                    })
                    .button("Sim", |s| {
                        s.pop_layer();
                        use crate::controller::user as controller;

                        let uuid: String = match s
                            .call_on_name("info_user_uuid", |view: &mut TextView| {
                                view.get_content()
                            }) {
                            Some(s) => s.source().into(),
                            None => return,
                        };

                        let uuid: uuid::Uuid = match uuid::Uuid::parse_str(&uuid) {
                            Ok(u) => u,
                            Err(_) => return,
                        };

                        if let Err(msg) = controller::remove_user(uuid) {
                            s.add_layer(Dialog::info(format!(
                                "Erro ao remover o usuário:\nHTTP {}: {}\n{}",
                                msg.status,
                                msg.message,
                                msg.details.unwrap_or("".into())
                            )));
                        } else {
                            // Pop user form
                            s.pop_layer();
                            // TODO: Refresh list
                        }
                    }),
            );
        });
    } else {
        dialog.add_button("Cadastrar", |s| {
            // Fetch info
            use crate::controller::user as controller;
            use crate::model::user::NewUser;

            let login = match s
                .call_on_name("input_user_login", |view: &mut EditView| view.get_content())
            {
                Some(s) => s,
                None => return,
            };

            let name =
                match s.call_on_name("input_user_name", |view: &mut EditView| view.get_content()) {
                    Some(s) => s,
                    None => return,
                };

            let email = match s
                .call_on_name("input_user_email", |view: &mut EditView| view.get_content())
            {
                Some(s) => s,
                None => return,
            };

            let password = match s.call_on_name("input_user_password", |view: &mut EditView| {
                view.get_content()
            }) {
                Some(s) => s,
                None => return,
            };

            match controller::create_user(NewUser {
                login: login.clone().trim().into(),
                name: name.clone().trim().into(),
                email: if email.trim() != "" {
                    Some(email.clone().trim().into())
                } else {
                    None
                },
                password: password.clone().trim().into(),
            }) {
                Ok(u) => {
                    s.pop_layer();
                    s.add_layer(Dialog::info(format!(
                        "Usuário cadastrado com sucesso!\nLogin: {}\nUUID: {}",
                        u.login, u.id
                    )));
                }
                Err(msg) => {
                    s.add_layer(Dialog::info(format!(
                        "Erro ao cadastrar o usuário:\nHTTP {}: {}\n{}",
                        msg.status,
                        msg.message,
                        msg.details.unwrap_or("".into())
                    )));
                }
            };
        });
    }

    dialog.add_button("Cancelar", |s| {
        s.pop_layer();
    });

    s.add_layer(dialog);
}

pub fn show_user_list(s: &mut Cursive) {
    // Loading screen
    s.set_autohide_menu(true);
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
        use crate::controller::user as controller;
        // Load items
        let items = controller::get_user_index().expect("User list");

        // Actual table form construction
        let show_list = move |s: &mut Cursive| {
            let size = (100, 20);

            let mut table = TableView::<User, CommonColumn>::new()
                .column(CommonColumn::Login, "Login", |c| c.width_percent(10))
                .column(CommonColumn::Name, "Nome", |c| c.width_percent(20))
                .column(CommonColumn::Email, "E-mail", |c| c)
                .column(CommonColumn::ID, "UUID", |c| {
                    c.align(HAlign::Left).width_percent(38)
                });

            table.set_items(items);

            table.set_on_submit(|s: &mut Cursive, _row: usize, index: usize| {
                let mut user = None;
                s.call_on_name("user_table", |table: &mut TableView<User, CommonColumn>| {
                    user = Some(table.borrow_item(index).unwrap().clone());
                })
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

        // Close loading form
        cb.send(Box::new(|s| {
            if s.focus_name("loading_user_list_view").is_ok() {
                s.pop_layer();
                s.set_autohide_menu(false);
            }
        }))
        .unwrap();

        cb.send(Box::new(show_list)).unwrap();
    });
}
