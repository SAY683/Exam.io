use crate::{Colour, Information, ViewDrive};
use comfy_table::Table;
use std::fmt::Debug;
use std::sync::atomic::{AtomicI64, Ordering};

///# ident
pub fn id_in(t: &str, i: bool, e: &AtomicI64) -> Table {
    Colour::Monitoring.table(Information {
        list: [t, "Status"],
        data: [[
            format!("INTER:{}", {
                let x = e.load(Ordering::Acquire);
                e.store(x + 1, Ordering::Release);
                x
            })
            .as_str(),
            format!("{}", if i { "Ok" } else { "Error" }).as_str(),
        ]],
    })
}
///# 翻译
pub fn translation<U: Debug>(e: U) {}
///# 命令表
pub fn mandate() -> Information<&'static str, Vec<&'static str>, Vec<Vec<&'static str>>> {
    Information {
        list: vec!["功能", "命令", "参数"],
        data: vec![vec!["", "", ""]],
    }
}
