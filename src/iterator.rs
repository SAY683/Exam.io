use async_stream::stream;
use futures::{pin_mut, StreamExt};
pub use intrusive_collections::{
    intrusive_adapter, KeyAdapter, LinkedListLink, PointerOps, RBTreeLink, SinglyLinkedListLink,
    XorLinkedListLink,
};
use itertools::Itertools;
use rayon::iter::{
    IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};
use std::fmt::Debug;
use std::mem::{transmute_copy, MaybeUninit};
use std::ops::{Index, IndexMut};
use std::sync::Arc;
use sync_cow::SyncCow;

///# 微型迭代器
#[derive(Clone)]
pub struct Epsilon<RME: Sized, const GN: usize>(pub [RME; GN]);

///# 迭代器特性
pub trait EpsilonIter<GP: Sized>: FromIterator<GP> {
    ///迭代
    fn into_init<const GN: usize>(self, i: fn(GP) -> ())
    where
        GP: Send,
        Self: Into<[GP; GN]>,
    {
        self.into().into_par_iter().for_each(|x| i(x));
    }
    ///引用迭代
    fn iter_init<const GN: usize>(&self, i: fn(&GP) -> ())
    where
        GP: Send + Sync,
        Self: AsRef<[GP; GN]>,
    {
        self.as_ref().par_iter().for_each(|x| i(x));
    }
    ///可变引用迭代
    fn mut_init<const GN: usize>(&mut self, i: fn(&GP) -> ())
    where
        GP: Send + Sync,
        Self: AsMut<[GP; GN]>,
    {
        self.as_mut().par_iter_mut().for_each(|x| i(x));
    }
    ///# 异步迭代
    async fn async_start<const GN: usize, UF: Sized>(
        self,
        ie: fn(GP) -> UF,
        op: Option<fn(UF) -> ()>,
    ) where
        Self: Into<[GP; GN]>,
    {
        let et = stream! {
            for i in self.into() {
                yield ie(i);
            }
        };
        pin_mut!(et);
        match op {
            None => while let Some(_value) = et.next().await {},
            Some(ie) => {
                while let Some(value) = et.next().await {
                    ie(value);
                }
            }
        }
    }
    ///转换vec
    fn to_vec(self) -> Vec<<Self as IntoIterator>::Item>
    where
        Self: IntoIterator,
    {
        self.into_iter().map(|x| x).collect_vec()
    }
    ///异步锁定
    fn arc_cow(self) -> Arc<SyncCow<Self>>
    where
        Self: Clone,
    {
        Arc::new(SyncCow::new(self))
    }
    ///裸指针
    fn ptr(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }
    ///释放
    unsafe fn raw(e: *mut Self) -> Box<Self> {
        Box::from_raw(e)
    }
    ///复制操作
    unsafe fn copy<AR: Sized, NT: Sized>(&self, e: fn(&Self) -> AR) -> NT {
        transmute_copy(&e(self))
    }
}
impl<RME: Sized, const GN: usize> EpsilonIter<RME> for Epsilon<RME, GN> {}
impl<RME: Sized, const GN: usize> IntoIterator for Epsilon<RME, GN> {
    type Item = RME;
    type IntoIter = std::array::IntoIter<RME, GN>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<RME: Sized, const GN: usize> FromIterator<RME> for Epsilon<RME, GN> {
    fn from_iter<T: IntoIterator<Item = RME>>(iter: T) -> Self {
        let mut mc: [RME; GN] = unsafe { MaybeUninit::uninit().assume_init() };
        iter.into_iter().enumerate().for_each(|(i, x)| {
            mc[i] = x;
        });
        Epsilon(mc)
    }
}
impl<RME: Sized, const GN: usize> Index<usize> for Epsilon<RME, GN> {
    type Output = RME;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<RME: Sized, const GN: usize> IndexMut<usize> for Epsilon<RME, GN> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}
impl<RME: Sized, const GN: usize> AsRef<[RME; GN]> for Epsilon<RME, GN> {
    fn as_ref(&self) -> &[RME; GN] {
        &self.0
    }
}
impl<RME: Sized, const GN: usize> AsMut<[RME; GN]> for Epsilon<RME, GN> {
    fn as_mut(&mut self) -> &mut [RME; GN] {
        &mut self.0
    }
}
impl<RME: Sized, const GN: usize> Into<[RME; GN]> for Epsilon<RME, GN> {
    fn into(self) -> [RME; GN] {
        self.0
    }
}
///# 聚合数据
#[derive(Debug, Default)]
pub struct Zeta<UC: Sized + Default + Send + Copy> {
    pub vec: SinglyLinkedListLink,
    pub btree: XorLinkedListLink,
    pub units: RBTreeLink,
    //内容
    pub project: UC,
}

impl<'life> From<&'life [u8]> for Zeta<&'life [u8]> {
    fn from(value: &'life [u8]) -> Self {
        Zeta {
            vec: Default::default(),
            btree: Default::default(),
            units: Default::default(),
            project: value,
        }
    }
}
//设置类型
intrusive_adapter!(pub Units = Arc<Zeta<&'static [u8]>>: Zeta<&'static [u8]> { units: RBTreeLink });
intrusive_adapter!(pub Vector= Arc<Zeta<&'static [u8]>>: Zeta<&'static [u8]> { vec: SinglyLinkedListLink});
intrusive_adapter!(pub Btree = Arc<Zeta<&'static [u8]>>: Zeta<&'static [u8]> { btree: XorLinkedListLink});
impl<'life> KeyAdapter<'life> for Units {
    type Key = &'life [u8];
    fn get_key(&self, value: &'life <Self::PointerOps as PointerOps>::Value) -> Self::Key {
        value.project
    }
}
impl<'life> KeyAdapter<'life> for Vector {
    type Key = &'life [u8];
    fn get_key(&self, value: &'life <Self::PointerOps as PointerOps>::Value) -> Self::Key {
        value.project
    }
}
impl<'life> KeyAdapter<'life> for Btree {
    type Key = &'life [u8];
    fn get_key(&self, value: &'life <Self::PointerOps as PointerOps>::Value) -> Self::Key {
        value.project
    }
}
