pub mod site;

use crate::site::{install, InstallUtils};
use anyhow::Result;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::path::PathBuf;

pub fn main() -> Result<()> {
    Ok(())
}
/// 设置
pub static INSTALL: Lazy<Install> = Lazy::new(|| install().unwrap());
///#核心设置
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Install {
    ///#数据
    pub data: Data,
    pub drive: Drive,
}
impl InstallUtils for Install {}
///#数据设置
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Data {
    //日志
    pub logs: Logs,
    //服务
    pub server: Server,
}
///#数据设置
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Logs {
    //日志开启
    pub enabled: bool,
    //缓存日期
    pub expiration_day: i64,
}
///#web设置
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Server {
    //root 节点
    pub root_node: String,
}
///#驱动
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Drive {
    //Mysql
    pub mysql: MysqlUlr,
    //Redis
    pub redis: RedisUlr,
}
///#Redis_Ulr
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RedisUlr {
    ///名称
    pub name: Option<String>,
    ///密码
    pub password: Option<String>,
    ///链接端口
    pub host: String,
    ///数据库
    pub database: String,
}

///#Mysql_Ulr
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MysqlUlr {
    ///名称
    pub name: String,
    ///密码
    pub password: String,
    ///链接端口
    pub host: String,
    ///数据库
    pub database: String,
}
lazy_static! {
    pub static ref LOCAL_PATH: Result<PathBuf> = Ok(current_dir()?);
    pub static ref LOCAL_BIN_PATH: PathBuf = {
        let mut x = PathBuf::new();
        x.push(LOCAL_PATH.as_ref().unwrap().as_path());
        x.push("bin");
        x
    };
//    pub static ref LOCAL_LIB_PATH: PathBuf = {
//        let mut x = PathBuf::new();
//        x.push(LOCAL_PATH.as_ref().unwrap().as_path());
//        x.push("lib");
//        x
//    };
//    pub static ref LOCAL_TMP_PATH: PathBuf = {
//        let mut x = PathBuf::new();
//        x.push(LOCAL_PATH.as_ref().unwrap().as_path());
//        x.push("tmp");
//        x
//    };
    pub static ref LOCAL_SETTINGS_PATH: PathBuf = {
        let mut x = PathBuf::new();
        x.push(LOCAL_PATH.as_ref().unwrap().as_path());
        x.push("bin");
        x.push("Settings.xml");
        x
    };
}
