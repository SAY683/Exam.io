#![feature(
    associated_type_defaults,
    async_closure,
    exclusive_wrapper,
    once_cell,
    const_trait_impl,
    const_transmute_copy,
    mem_copy_fn,
    try_trait_v2,
    try_blocks,
    impl_trait_projections,
    box_syntax,
    async_fn_in_trait,
    generic_arg_infer
)]

pub mod function;
pub mod iterator;
pub mod thread;
#[macro_use]
pub mod macros;
pub mod build;
pub mod error;

#[cfg(target_os = "linux")]
use crate::build::initialize_linux;
use crate::build::initialize_windows;
use crate::error::ThreadEvents;
pub use crate::iterator::{Btree, Vector, Zeta};
use anyhow::Result;
use itertools::Itertools;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use spin::RwLock;
use std::collections::BTreeSet;
use std::env;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Condvar, Mutex};
use tokio::main;
use Attribute::command_registration;
use Settings::INSTALL;
use View::{Colour, ViewDrive};

#[main]
pub async fn main() -> Result<()> {
    if !ARGS_SUB.is_empty() {
        #[cfg(target_os = "windows")]
        initialize_windows().await?;
        #[cfg(target_os = "linux")]
        initialize_linux().await?;
    } else {
        println!(
            "{}",
            Colour::Order.table(View::show::mandate(INSTALL.drive.zh))
        );
    }
    Ok(())
}

///# 指令集
pub static INSTRUCTION_SET: Lazy<RwLock<BTreeSet<String>>> =
    Lazy::new(|| RwLock::new(BTreeSet::new()));
///# 主控制
pub static mut MAIN_PROC: Lazy<Subject> =
    Lazy::new(|| Arc::new((Mutex::new(false), Condvar::new())));
/*
初始数据:
*/
lazy_static! {
    //命令参数
    pub static ref ARGS_SUB: Vec<String> = {
        let mut uc = env::args().collect_vec();
        uc.remove(0);
        uc
    };
}
///# 异步
pub type Alpha<'life, RE> = Pin<Box<dyn Future<Output = RE> + Send + Sync + 'life>>;
///# 异步
pub type Beta<'life, RE, GX> = Box<dyn FnMut(RE) -> Alpha<'life, GX> + Send + Sync + 'life>;
///# 闭包
pub type Gamma<'life, RE, GX> = Box<dyn FnMut(RE) -> GX + Send + Sync + 'life>;
///# 控制
///# Arc<(Mutex<bool>, Condvar)>
pub type Subject = Arc<(Mutex<bool>, Condvar)>;
///# 线程返回
///# Result<GX, ThreadEvents>
pub type ThreadActive<GX> = Result<GX, ThreadEvents>;
