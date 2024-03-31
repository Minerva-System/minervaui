use super::common::CommonColumn;
use cursive::{align::HAlign, traits::*, views::*, Cursive, Rect};
use cursive_table_view::TableView;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref INPUT_RECT: Rect = Rect::from_size((0, 0), (20, 1));
    pub static ref LABEL_RECT: Rect = Rect::from_size((0, 0), (16, 1));
    pub static ref LABEL_CHANGE_RECT: Rect = Rect::from_size((0, 0), (30, 1));
}

// const INPUT_RECT: Rect = Rect::from_size((0, 0), (40, 1));
//const LABEL_RECT: Rect = Rect::from_size((0, 0), (11, 1));
//const LABEL_CHANGE_RECT: Rect = Rect::from_size((0, 0), (9, 1));

fn make_select_info<S>(name: &str, label: &str, content: &str, cb: S) -> LinearLayout
where
    S: 'static + Fn(&mut Cursive),
{
    LinearLayout::horizontal()
        .child(FixedLayout::new().child(*LABEL_RECT, TextView::new(label)))
        .child(
            FixedLayout::new().child(
                *LABEL_CHANGE_RECT,
                TextView::new(content)
                    .h_align(HAlign::Right)
                    .with_name(name),
            ),
        )
        .child(FixedLayout::new().child(Rect::from_size((0, 0), (1, 1)), FixedLayout::new()))
        .child(Button::new("*", cb))
}

pub fn show_pdv(s: &mut Cursive) {
    let make_info = |name: &str, label: &str, content: &str| -> LinearLayout {
        LinearLayout::horizontal()
            .child(FixedLayout::new().child(*LABEL_RECT, TextView::new(label)))
            .child(
                FixedLayout::new().child(
                    *INPUT_RECT,
                    TextView::new(content)
                        .h_align(HAlign::Right)
                        .with_name(name),
                ),
            )
    };

    let make_edit = |name: &str, content: String, secret: bool| -> FixedLayout {
        let mut edit = EditView::new().content(content).filler(" ");
        if secret {
            edit = edit.secret();
        }
        let named_edit = edit.with_name(name);
        FixedLayout::new().child(*INPUT_RECT, named_edit)
    };

    let make_field_with_secret =
        |name: &str, label: &str, initial_content: String, secret: bool| -> LinearLayout {
            LinearLayout::horizontal()
                .child(FixedLayout::new().child(*LABEL_RECT, TextView::new(label)))
                .child(make_edit(name, initial_content, secret))
        };

    let make_field = |name: &str, label: &str, initial_content: String| -> LinearLayout {
        make_field_with_secret(name, label, initial_content, false)
    };

    use crate::model::sales::SaleItem;

    let mut table = TableView::<SaleItem, CommonColumn>::new()
        .column(CommonColumn::Description, "Produto", |c| {
            c.width_percent(50).align(HAlign::Left)
        })
        .column(CommonColumn::Unit, "Unid.", |c| {
            c.width_percent(4).align(HAlign::Center)
        })
        .column(CommonColumn::Price, "Preço", |c| {
            c.width_percent(8).align(HAlign::Right)
        })
        .column(CommonColumn::Amount, "Qtde.", |c| {
            c.width_percent(8).align(HAlign::Right)
        })
        .column(CommonColumn::Total, "Total", |c| {
            c.width_percent(8).align(HAlign::Right)
        });

    let layout = LinearLayout::vertical()
        .child(make_select_info(
            "input_pdv_salesperson",
            "Vendedor:",
            "(selecione)",
            |_| {},
        ))
        .child(make_select_info(
            "input_pdv_client",
            "Cliente:",
            "(selecione)",
            |_| {},
        ))
        .child(
            Panel::new(table.with_name("sale_items_table").min_size((100, 20)))
                .title("Itens")
                .title_position(HAlign::Left),
        )
        .child(make_field(
            "input_pdv_discount",
            "Descontos:",
            String::new(),
        ))
        .child(make_info("input_pdv_saletotal", "Total da venda:", "0.00"));

    let dialog = Dialog::around(layout)
        .title("Ponto de Venda")
        .button("Faturar", |_| {})
        .button("Salvar", |_| {})
        .button("Cancelar", |s| {
            s.pop_layer();
        });

    s.add_layer(dialog);
}
