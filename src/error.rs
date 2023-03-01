use std::io;
use thiserror::Error;
use tokio::task::JoinError;

///# 线程事件
#[derive(Debug, Error)]
pub enum ThreadEvents {
    //未知错误
    #[error("UnrecognizedTypeError{0:#?}")]
    UnknownError(#[from] anyhow::Error),
    //线程运行错误
    #[error("ThreadCrashError{0:#?}")]
    ThreadRunError(#[from] JoinError),
    //运行错误
    #[error("MainCrashError{0:#?}")]
    MainError(#[from] LogicalEvent),
    //数据错误
    #[error("IoError{0:#?}")]
    IoError(#[from] io::Error),
}
///# 逻辑事件
#[derive(Debug, Error)]
pub enum LogicalEvent {
    //严重错误
    #[error("PromptError{0:#?}")]
    PromptError(#[from] anyhow::Error),
}
