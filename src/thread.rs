use crate::error::ThreadEvents;
use crate::{Beta, Subject, ThreadActive};
use anyhow::anyhow;
use futures::executor::LocalPool;
use futures::task::SpawnExt;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};
use std::ops::{AddAssign, Index, IndexMut};
use std::sync::{Arc, Condvar, Mutex};
use std::vec;
use sync_cow::SyncCow;
use tokio::spawn;
use tokio::task::JoinHandle;
use uuid::fmt::Urn;
use uuid::Uuid;

///# 特性
///# [derive(Clone, Copy, Default)]
///# struct CoreThread;
///# CoreThread::alliance(CoreThread::aggregate_alliance::<CoreThread>()?).await?;
pub trait Alexa<'life, NTD: Sized + Send + Sync> {
    ///# 控制
    fn description() -> Subject {
        Arc::new((Mutex::new(false), Condvar::new()))
    }
    ///# 单线程运行
    fn submit(e: Vec<std::thread::JoinHandle<NTD>>) -> ThreadActive<Arc<SyncCow<Vec<NTD>>>>
    where
        NTD: Clone + 'static,
    {
        let ert = Arc::new(SyncCow::new(vec![]));
        e.into_par_iter()
            .for_each(|i| ert.edit(|x| x.push(i.join().expect("submit"))));
        Ok(ert)
    }
    //++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    ///# 事件
    fn event() -> Vec<Beta<'life, Subject, ThreadActive<NTD>>>;
    ///# 管理机
    fn alex() -> Alex<'life, NTD> {
        Alex {
            alex: <Self as Alexa<NTD>>::description(),
            execution: BTreeSet::from_iter([Execution {
                id: Urn::from_uuid(Uuid::new_v4()).to_string(),
                execution: <Self as Alexa<NTD>>::event(),
            }]),
        }
    }
    ///# 线程机转换
    fn aggregate<CR: Alexa<'static, NTD>>() -> Aggregation<NTD>
    where
        NTD: 'static,
    {
        <CR as Alexa<NTD>>::alex().into()
    }
    ///# 线程机并发转换
    fn aggregate_all<CR: Alexa<'static, NTD>>() -> ThreadActive<Aggregation<NTD>>
    where
        NTD: 'static,
    {
        let value = <CR as Alexa<NTD>>::alex();
        let ur = value.alex;
        let btree = Mutex::new(vec![]);
        value.execution.into_par_iter().for_each(|i| {
            i.execution.into_iter().for_each(|mut j| {
                let mx = ur.clone();
                btree
                    .lock()
                    .unwrap()
                    .push(spawn(async move { j(mx).await }));
            });
        });
        match btree.into_inner() {
            Ok(e) => Ok(Aggregation(e)),
            Err(e) => Err(ThreadEvents::UnknownError(anyhow!(e))),
        }
    }
    ///# 实体运行
    async fn run(e: Aggregation<NTD>) -> ThreadActive<Vec<NTD>>
    where
        NTD: 'static,
    {
        let mut ert = vec![];
        for i in e.0.into_iter() {
            ert.push(match i.await {
                Ok(e) => Ok(e?),
                Err(e) => Err(ThreadEvents::ThreadRunError(e)),
            }?);
        }
        Ok(ert)
    }
    ///# 虚拟运行
    async fn sync(e: Aggregation<NTD>) -> ThreadActive<Arc<SyncCow<Vec<NTD>>>>
    where
        NTD: 'static + Clone,
    {
        let ert = Arc::new(SyncCow::new(vec![]));
        e.0.into_iter().for_each(|i| {
            let er = ert.clone();
            spawn(async move {
                let o = i.await;
                er.edit(|x| {
                    x.push(o.unwrap().unwrap());
                });
            });
        });
        Ok(ert)
    }
    ///# 并行运行
    async fn alliance(e: Aggregation<NTD>) -> ThreadActive<Arc<SyncCow<Vec<NTD>>>>
    where
        NTD: 'static + Clone,
    {
        let ert = Arc::new(SyncCow::new(vec![]));
        e.0.into_par_iter().for_each(|i| {
            let er = ert.clone();
            let mut u = LocalPool::new();
            u.spawner()
                .spawn(async move {
                    let o = i
                        .await
                        .unwrap_or_else(|x| {
                            panic!("{x}");
                        })
                        .unwrap_or_else(|x| {
                            panic!("{x}");
                        });
                    er.edit(|x| {
                        x.push(o);
                    });
                })
                .unwrap();
            u.run();
        });
        Ok(ert)
    }
    ///# 通知
    fn advice(e: &(Mutex<bool>, Condvar)) -> anyhow::Result<()> {
        *e.0.lock().unwrap() = true;
        e.1.notify_all();
        Ok(())
    }
    ///# 等待
    fn wait(e: &(Mutex<bool>, Condvar)) -> anyhow::Result<bool> {
        Ok(*e.1.wait(e.0.lock().unwrap()).unwrap())
    }
}
///# 聚合
pub struct Aggregation<NT: Sized + Send + Sync>(pub Vec<JoinHandle<ThreadActive<NT>>>);

impl<NT: Sized + Send + Sync> IntoIterator for Aggregation<NT> {
    type Item = JoinHandle<ThreadActive<NT>>;
    type IntoIter = vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<NT: Sized + Send + Sync> AddAssign for Aggregation<NT> {
    fn add_assign(&mut self, rhs: Self) {
        let mut rhs = rhs.0;
        self.0.append(&mut rhs);
    }
}
impl<NT> Into<Aggregation<NT>> for Alex<'static, NT>
where
    NT: Sized + Send + Sync + 'static,
{
    fn into(self) -> Aggregation<NT> {
        let alex = self.alex;
        let mut btree = Vec::default();
        self.execution.into_iter().for_each(|i| {
            i.execution.into_iter().for_each(|mut j| {
                let mc = alex.clone();
                btree.push(spawn(async move { j(mc).await }));
            });
        });
        Aggregation(btree)
    }
}
///# 管理器
pub struct Alex<'life, NT: Sized + Send> {
    pub alex: Subject,
    pub execution: BTreeSet<Execution<'life, NT>>,
}
///# 执行机
pub struct Execution<'life, NT> {
    pub id: String,
    pub execution: Vec<Beta<'life, Subject, ThreadActive<NT>>>,
}
impl<'life, NT> Default for Execution<'life, NT> {
    fn default() -> Self {
        Execution {
            id: String::default(),
            execution: Vec::default(),
        }
    }
}
impl<'life, NT> IntoIterator for Execution<'life, NT> {
    type Item = Beta<'life, Subject, ThreadActive<NT>>;
    type IntoIter = vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.execution.into_iter()
    }
}
impl<'life, NT> Index<usize> for Execution<'life, NT> {
    type Output = Beta<'life, Subject, ThreadActive<NT>>;
    fn index(&self, index: usize) -> &Self::Output {
        self.execution.index(index)
    }
}
impl<'life, NT> IndexMut<usize> for Execution<'life, NT> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.execution.index_mut(index)
    }
}
impl<'life, NT> Eq for Execution<'life, NT> {}
impl<'life, NT> PartialEq<Self> for Execution<'life, NT> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}
impl<'life, NT> PartialOrd<Self> for Execution<'life, NT> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}
impl<'life, NT> Ord for Execution<'life, NT> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl<'life, NT> Hash for Execution<'life, NT> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}
