pub mod environment;
pub mod hardware;
pub mod os;
pub mod packages;
pub mod storage;

pub use environment::{EnvironmentInfo, collect_environment};
pub use hardware::{HardwareInfo, collect_cpu, collect_gpus, collect_host, collect_memory};
pub use os::{OsInfo, collect_os};
pub use packages::{PackageCounts, collect_packages};
pub use storage::{DiskDisplay, collect_disks};

#[derive(Debug, Clone, serde::Serialize)]
pub struct SystemInfo {
    pub os: OsInfo,
    pub host: Option<String>,
    pub cpu: String,
    pub gpus: Vec<String>,
    pub memory: String,
    pub memory_used_bytes: u64,
    pub memory_total_bytes: u64,
    pub memory_percentage: u8,
    pub environment: EnvironmentInfo,
    pub disks: Vec<DiskDisplay>,
    pub packages: PackageCounts,
}

/// Executes all system collectors in parallel across the rayon thread pool for sub-10ms response.
#[must_use]
pub fn collect_all_parallel() -> SystemInfo {
    let (os, (host, cpu)) = rayon::join(collect_os, || rayon::join(collect_host, collect_cpu));

    let (gpus, (memory, mem_used, mem_total, mem_pct)) = rayon::join(collect_gpus, collect_memory);

    let (environment, ((disks, _), packages)) =
        rayon::join(collect_environment, || rayon::join(collect_disks, collect_packages));

    SystemInfo {
        os,
        host,
        cpu,
        gpus,
        memory,
        memory_used_bytes: mem_used,
        memory_total_bytes: mem_total,
        memory_percentage: mem_pct,
        environment,
        disks,
        packages,
    }
}
