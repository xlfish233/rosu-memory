use std::path::PathBuf;
use std::time::Duration;

/// 内存读取器配置
#[derive(Debug, Clone)]
pub struct MemoryReaderConfig {
    pub osu_path: Option<PathBuf>,
    pub error_interval: Duration,
    pub auto_detect_path: bool,
    pub normal_interval: Duration,
}

impl Default for MemoryReaderConfig {
    fn default() -> Self {
        Self {
            osu_path: None,
            error_interval: Duration::from_secs(3),
            auto_detect_path: true,
            normal_interval: Duration::from_millis(300),
        }
    }
}
