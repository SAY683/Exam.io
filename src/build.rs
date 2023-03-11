use crate::ThreadActive;

///# 初始化
#[cfg(target_os = "linux")]
pub async fn initialize_linux() -> ThreadActive<()> {
    Ok(())
}
#[cfg(target_os = "windows")]
pub async fn initialize_windows() -> ThreadActive<()> {
    Ok(())
}
