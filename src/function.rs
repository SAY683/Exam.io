use crate::{Alpha, Beta, Gamma};
use serde::{Deserialize, Serialize};
use std::future::Future;

///# 基本存储函数
pub enum FutureEx<'life, RE, GX>
where
    RE: Serialize + Deserialize<'life>,
    GX: Sized + Send,
{
    ///# 异步函数
    AsyncTrait(Alpha<'life, GX>),
    ///# 异步闭包函数
    AsyncFnTrait(Beta<'life, RE, GX>),
    ///# 普通函数
    FnTrait(Gamma<'life, RE, GX>),
}

impl<'life, RE, GX> FutureEx<'life, RE, GX>
where
    RE: Serialize + Deserialize<'life> + 'life,
    GX: Sized + Send + 'life,
{
    ///# FutureEx::AsyncTrait(Box::pin(e))
    pub fn sync(e: impl Future<Output = GX> + Send + Sync + 'life) -> Self {
        FutureEx::AsyncTrait(Box::pin(e))
    }
    ///# FutureEx::FnTrait(Box::new(e))
    pub fn def(e: fn(RE) -> GX) -> Self {
        FutureEx::FnTrait(Box::new(e))
    }
    ///# FutureEx::AsyncFnTrait(Box::new(e))
    pub fn def_sync(e: fn(RE) -> Alpha<'life, GX>) -> Self {
        FutureEx::AsyncFnTrait(Box::new(e))
    }
    ///# 运行
    pub async fn run_sync(&mut self, arg: RE) -> GX {
        match self {
            FutureEx::AsyncTrait(e) => e.await,
            FutureEx::AsyncFnTrait(e) => e(arg).await,
            FutureEx::FnTrait(e) => e(arg),
        }
    }
}

impl<'life, RE, GX> From<Gamma<'life, RE, GX>> for FutureEx<'life, RE, GX>
where
    RE: Serialize + Deserialize<'life>,
    GX: Sized + Send,
{
    fn from(value: Gamma<'life, RE, GX>) -> Self {
        FutureEx::FnTrait(value)
    }
}
impl<'life, RE, GX> From<Beta<'life, RE, GX>> for FutureEx<'life, RE, GX>
where
    RE: Serialize + Deserialize<'life>,
    GX: Sized + Send,
{
    fn from(value: Beta<'life, RE, GX>) -> Self {
        FutureEx::AsyncFnTrait(value)
    }
}
impl<'life, RE, GX> From<Alpha<'life, GX>> for FutureEx<'life, RE, GX>
where
    RE: Serialize + Deserialize<'life>,
    GX: Sized + Send,
{
    fn from(value: Alpha<'life, GX>) -> Self {
        FutureEx::AsyncTrait(value)
    }
}
