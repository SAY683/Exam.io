#![feature(async_fn_in_trait)]
use crate::error::LogicalEvent;
use crate::error::ThreadEvents;
use std::future::Future;
use std::pin::Pin;

pub mod error;
pub mod iterator;

fn main() {
    println!("Hello, world!");
}
///# 异步
pub type Alpha<'life, RE> = Pin<Box<dyn Future<Output = RE> + Send + Sync + 'life>>;
///# 异步
pub type Beta<'life, RE, GX> = Box<dyn FnMut(RE) -> Alpha<'life, GX> + Send + Sync + 'life>;
///# 闭包
pub type Gamma<'life, RE, GX> = Box<dyn FnMut(RE) -> GX + Send + Sync + 'life>;
///# 线程返回
///# Result<GX, ThreadEvents>
pub type ThreadActive<GX> = anyhow::Result<GX, ThreadEvents>;
///# 处理
pub type ProcessActive<GX> = anyhow::Result<GX, LogicalEvent>;
