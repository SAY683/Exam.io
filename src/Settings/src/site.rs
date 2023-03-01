use crate::{Install, LOCAL_SETTINGS_PATH};
use anyhow::Result;
use quick_xml::{de, se};
use serde::{Deserialize, Serialize};
use serde_json_borrow::Value;
use std::fs;
use std::path::Path;

///# 设置
pub fn install() -> Result<Install> {
    Ok(Install::build(LOCAL_SETTINGS_PATH.as_path())?)
}
///# 设置工具
pub trait InstallUtils: Serialize + for<'life> Deserialize<'life> {
    fn build_unknown(e: &str) -> Result<Value> {
        Ok(serde_json::from_str(e)?)
    }
    ///# 构建
    fn build(e: &Path) -> Result<Self> {
        Ok(de::from_str(&fs::read_to_string(e)?)?)
    }
    ///# 更新
    fn update(self, e: &Path) -> Result<Self> {
        fs::write(e, se::to_string(&self)?)?;
        Ok(self)
    }
}
