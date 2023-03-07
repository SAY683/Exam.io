use crate::{Colour, Information, ViewDrive};
use comfy_table::Table;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::DerefMut;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::RwLock;

///#命令/函数名称
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct Order(pub &'static str, pub &'static str);

pub trait Mandate {
    fn set_order(&self) -> Self;
}
///命令集
pub static ORDER: Lazy<RwLock<BTreeMap<&'static str, &'static str>>> =
    Lazy::new(|| RwLock::new(BTreeMap::new()));

impl Mandate for Order {
    fn set_order(&self) -> Self {
        let mut x = ORDER.write().unwrap();
        x.deref_mut().insert(self.0, self.1);
        *self
    }
}
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
///# 命令表
pub fn mandate(e: bool) -> Information<&'static str, Vec<&'static str>, Vec<Vec<&'static str>>> {
    if e {
        Information {
            list: vec!["功能", "命令", "参数"],
            data: vec![vec!["", "", ""]],
        }
    } else {
        Information {
            list: vec!["function", "command", "parameter"],
            data: vec![vec!["", "", ""]],
        }
    }
}
//const YANDEX_API_KEY: &str = "";
//use text_translator::{Api, InputLanguage, Language, Yandex};
/////# 翻译
//pub fn translation(text: &str) -> anyhow::Result<String> {
//    let translator: Yandex = Yandex::with_key(YANDEX_API_KEY);
//    Ok(translator.translate(
//        text.to_string(),
//        InputLanguage::Defined(Language::Chinese),
//        Language::English,
//    )?)
//}

impl From<(&'static str, &'static str)> for Order {
    fn from(value: (&'static str, &'static str)) -> Self {
        Order(value.0, value.1)
    }
}

impl From<(String, String)> for Order {
    ///危险转换
    fn from(value: (String, String)) -> Self {
        unsafe {
            Order(
                mem::transmute::<&'_ str, &'static str>(value.0.as_str()),
                mem::transmute::<&'_ str, &'static str>(value.0.as_str()),
            )
        }
    }
}

impl Hash for Order {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
