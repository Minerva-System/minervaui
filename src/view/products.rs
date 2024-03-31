use super::common::CommonColumn;
use crate::model::products::Product;
use cursive::{align::HAlign, traits::*, views::*, Cursive, Rect};
use cursive_table_view::TableView;

pub fn show_product_form(s: &mut Cursive, product: Option<Product>) {
    let input_rect = Rect::from_size((0, 0), (40, 1));
    let label_rect = Rect::from_size((0, 0), (11, 1));

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

    let make_field_unit = |name: &str, label: &str, initial_content: String| -> LinearLayout {
        LinearLayout::horizontal()
            .child(FixedLayout::new().child(label_rect, TextView::new(label)))
            .child(
                FixedLayout::new().child(
                    input_rect,
                    EditView::new()
                        .content(initial_content)
                        .filler(" ")
                        .max_content_width(2)
                        .with_name(name),
                ),
            )
    };

    let make_field = |name: &str, label: &str, initial_content: String| -> LinearLayout {
        make_field_with_secret(name, label, initial_content, false)
    };

    let editing = product.is_some();

    let layout = match product {
        Some(product) => {
            let product_id = product.id.to_string();
            LinearLayout::vertical()
                .child(make_info("info_product_uuid", "UUID:", &product_id))
                .child(make_field(
                    "input_product_description",
                    "Descrição:",
                    product.description,
                ))
                .child(make_field(
                    "input_product_price",
                    "Preço:",
                    product.price.to_string(),
                ))
                .child(make_field_unit(
                    "input_product_unit",
                    "Unidade:",
                    product.unit,
                ))
        }
        None => LinearLayout::vertical()
            .child(make_info(
                "info_product_uuid",
                "UUID:",
                "(será gerado automaticamente)",
            ))
            .child(make_field(
                "input_product_description",
                "Descrição:",
                String::new(),
            ))
            .child(make_field(
                "input_product_price",
                "Preço:",
                "0.00".to_string(),
            ))
            .child(make_field_unit(
                "input_product_unit",
                "Unidade:",
                "UN".to_string(),
            )),
    };

    let mut dialog = Dialog::around(layout).title(if editing {
        "Edição de Produto"
    } else {
        "Novo Produto"
    });

    if editing {
        dialog.add_button("Salvar", |_| {});
        dialog.add_button("Remover", |s| {
            s.add_layer(
                Dialog::around(TextView::new("Deseja realmente remover este produto?"))
                    .button("Não", |s| {
                        s.pop_layer();
                    })
                    .button("Sim", |s| {
                        s.pop_layer();
                        use crate::controller::products as controller;

                        let uuid: String = match s
                            .call_on_name("info_product_uuid", |view: &mut TextView| {
                                view.get_content()
                            }) {
                            Some(s) => s.source().into(),
                            None => return,
                        };

                        let uuid: uuid::Uuid = match uuid::Uuid::parse_str(&uuid) {
                            Ok(u) => u,
                            Err(_) => return,
                        };

                        if let Err(msg) = controller::remove_product(uuid) {
                            s.add_layer(Dialog::info(format!(
                                "Erro ao remover o produto:\nHTTP {}: {}\n{}",
                                msg.status,
                                msg.message,
                                msg.details.unwrap_or("".into())
                            )));
                        } else {
                            // Pop user form
                            s.pop_layer();
                            // Refresh list
                            s.call_on_name(
                                "product_table",
                                |table: &mut TableView<Product, CommonColumn>| {
                                    let items =
                                        controller::get_product_index().expect("Product list");
                                    table.set_items(items);
                                },
                            );
                        }
                    }),
            );
        });
    } else {
        dialog.add_button("Cadastrar", |s| {
            use crate::controller::products as controller;
            use crate::model::products::NewProduct;
            use rust_decimal::prelude::*;

            let description = match s
                .call_on_name("input_product_description", |view: &mut EditView| {
                    view.get_content()
                }) {
                Some(s) => s,
                None => return,
            };

            let price = match s.call_on_name("input_product_price", |view: &mut EditView| {
                view.get_content()
            }) {
                Some(s) => s,
                None => return,
            };
            let price = match Decimal::from_str(&price) {
                Ok(d) => d,
                Err(_) => return,
            };

            let unit = match s.call_on_name("input_product_unit", |view: &mut EditView| {
                view.get_content()
            }) {
                Some(s) => s,
                None => return,
            };

            match controller::create_product(NewProduct {
                description: description.clone().trim().into(),
                price,
                unit: unit.clone().trim().to_uppercase(),
            }) {
                Ok(p) => {
                    s.pop_layer();
                    s.add_layer(Dialog::info(format!(
                        "Produto cadastrado com sucesso!\nUUID: {}",
                        p.id
                    )));
                }
                Err(msg) => {
                    s.add_layer(Dialog::info(format!(
                        "Erro ao cadastrar o produto:\nHTTP {}: {}\n{}",
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

pub fn show_product_list(s: &mut Cursive) {
    // Loading screen
    s.set_autohide_menu(true);
    s.add_layer(
        Dialog::around(
            TextView::new("Carregando...")
                .h_align(HAlign::Center)
                .min_size((20, 1)),
        )
        .with_name("loading_product_list_view"),
    );

    // Get sink
    let cb = s.cb_sink().clone();

    // Spawn thread
    std::thread::spawn(move || {
        use crate::controller::products as controller;
        // Load items
        let items = controller::get_product_index().expect("Product list");

        // Actual table form construction
        let show_list = move |s: &mut Cursive| {
            let size = (100, 20);

            let mut table = TableView::<Product, CommonColumn>::new()
                .column(CommonColumn::Description, "Descrição", |c| {
                    c.width_percent(40).align(HAlign::Left)
                })
                .column(CommonColumn::Unit, "Unid.", |c| {
                    c.width_percent(4).align(HAlign::Center)
                })
                .column(CommonColumn::Price, "Preço", |c| {
                    c.width_percent(8).align(HAlign::Right)
                })
                .column(CommonColumn::CreatedAt, "Criado Em", |c| {
                    c.align(HAlign::Right).width_percent(15)
                })
                .column(CommonColumn::UpdatedAt, "Última Atualiz.", |c| {
                    c.align(HAlign::Right).width_percent(15)
                });

            table.set_items(items);

            table.set_on_submit(|s: &mut Cursive, _row: usize, index: usize| {
                let mut product = None;
                s.call_on_name(
                    "product_table",
                    |table: &mut TableView<Product, CommonColumn>| {
                        product = Some(table.borrow_item(index).unwrap().clone());
                    },
                )
                .unwrap();

                show_product_form(s, product);
            });

            s.add_layer(
                Dialog::around(table.with_name("product_table").min_size(size))
                    .title("Lista de Produtos")
                    .button("Fechar", |s| {
                        s.pop_layer();
                    }),
            );
        };

        // Close loading form
        cb.send(Box::new(|s| {
            if s.focus_name("loading_product_list_view").is_ok() {
                s.pop_layer();
                s.set_autohide_menu(false);
            }
        }))
        .unwrap();

        cb.send(Box::new(show_list)).unwrap();
    });
}
