pub mod show;

use anyhow::Result;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use opener::open;
use std::fmt::Display;

///#风格
pub enum Colour {
    ///错误
    Error,
    ///输出
    Output,
    ///命令
    Order,
    ///监控
    Monitoring,
    ///函数
    Function,
}

///#数据
pub struct Information<EF: Display, ER: IntoIterator<Item = EF>, GL: IntoIterator<Item = ER>> {
    ///列表
    pub list: ER,
    ///数据
    pub data: GL,
}

///# 显示
pub trait ViewDrive {
    fn view(&self) -> Frames;
    fn table<EF: Display, ER: IntoIterator<Item = EF>, GL: IntoIterator<Item = ER>>(
        &self,
        _: Information<EF, ER, GL>,
    ) -> Table;
    ///# 错误显示
    fn error_display<G: Sized>(r: &str, e: Result<G>) -> G {
        e.unwrap_or_else(|x| {
            panic!(
                "{}",
                Colour::Error.table(Information {
                    list: [r],
                    data: [[format!("{x:#?}").as_str()]]
                })
            )
        })
    }
    ///# 空值显示
    fn operation<G: Sized>(r: &str, e: Option<G>) -> G {
        e.unwrap_or_else(|| {
            panic!(
                "{}",
                Colour::Error.table(Information {
                    list: [r],
                    data: [["None"]]
                })
            )
        })
    }
    ///# 默认开启
    fn operation_browser(ulr: &str) -> Result<()> {
        Ok(open(ulr)?)
    }
}
impl ViewDrive for Colour {
    fn view(&self) -> Frames {
        match self {
            Colour::Error => Frames {
                text: Attribute::Italic,
                frames: Color::DarkRed,
            },
            Colour::Output => Frames {
                text: Attribute::Bold,
                frames: Color::DarkGreen,
            },
            Colour::Order => Frames {
                text: Attribute::RapidBlink,
                frames: Color::DarkYellow,
            },
            Colour::Monitoring => Frames {
                text: Attribute::Underlined,
                frames: Color::DarkCyan,
            },
            Colour::Function => Frames {
                text: Attribute::Reverse,
                frames: Color::DarkGrey,
            },
        }
    }
    fn table<EF: Display, ER: IntoIterator<Item = EF>, GL: IntoIterator<Item = ER>>(
        &self,
        e: Information<EF, ER, GL>,
    ) -> Table {
        let i = Colour::view(self);
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(40);
        table.set_header(
            e.list
                .into_iter()
                .map(|x| Cell::new(x).add_attribute(i.text).fg(i.frames))
                .into_iter()
                .collect::<Vec<_>>(),
        );
        e.data.into_iter().for_each(|x| {
            table.add_row(
                x.into_iter()
                    .map(|x| Cell::new(x).add_attribute(i.text).fg(i.frames))
                    .into_iter()
                    .collect::<Vec<_>>(),
            );
        });
        table
    }
}

///#画面数据
pub struct Frames {
    //文本
    text: Attribute,
    //单元格前景色
    frames: Color,
}
