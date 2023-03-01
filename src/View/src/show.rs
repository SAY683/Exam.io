use crate::{Colour, Information, ViewDrive};
use comfy_table::Table;
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
