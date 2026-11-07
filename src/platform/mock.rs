pub mod drives {
    #[derive(Debug, Clone, serde::Serialize)]
    pub struct DriveInfo {
        pub mount: String,
        pub total_bytes: u64,
        pub used_bytes: u64,
        pub free_bytes: u64,
        pub used_percentage: u8,
    }

    #[must_use]
    pub fn get_fixed_drives() -> Vec<DriveInfo> {
        vec![DriveInfo {
            mount: "C:".to_string(),
            total_bytes: 1_000_000_000_000,
            used_bytes: 400_000_000_000,
            free_bytes: 600_000_000_000,
            used_percentage: 40,
        }]
    }
}

pub mod process {
    #[derive(Debug, Clone, Default)]
    pub struct ParentContext {
        pub shell: Option<String>,
        pub terminal: Option<String>,
    }

    #[must_use]
    pub fn detect_shell_and_terminal() -> ParentContext {
        ParentContext {
            shell: Some("PowerShell".to_string()),
            terminal: Some("Windows Terminal".to_string()),
        }
    }
}

pub mod registry {
    #[must_use]
    pub fn read_hklm_string(_subkey: &str, _value_name: &str) -> Option<String> {
        None
    }

    #[must_use]
    pub fn read_hklm_u32(_subkey: &str, _value_name: &str) -> Option<u32> {
        None
    }

    #[must_use]
    pub fn read_hklm_u64(_subkey: &str, _value_name: &str) -> Option<u64> {
        None
    }

    #[must_use]
    pub fn enum_subkeys(_root: *mut std::ffi::c_void, _subkey: &str) -> Vec<String> {
        Vec::new()
    }
}

pub mod system {
    #[derive(Debug, Clone, Copy, Default)]
    pub struct MemoryStats {
        pub total_bytes: u64,
        pub used_bytes: u64,
        pub free_bytes: u64,
    }

    #[must_use]
    pub fn get_memory_stats() -> Option<MemoryStats> {
        Some(MemoryStats {
            total_bytes: 16_000_000_000,
            used_bytes: 8_000_000_000,
            free_bytes: 8_000_000_000,
        })
    }

    #[must_use]
    pub fn get_uptime_seconds() -> u64 {
        86400
    }

    #[derive(Debug, Clone, Copy)]
    pub struct DisplayInfo {
        pub width: u32,
        pub height: u32,
        pub refresh_rate_hz: u32,
    }

    #[must_use]
    pub fn get_display_info() -> Option<DisplayInfo> {
        Some(DisplayInfo {
            width: 1920,
            height: 1080,
            refresh_rate_hz: 60,
        })
    }

    #[derive(Debug, Clone, Copy)]
    pub struct BatteryInfo {
        pub percentage: u8,
        pub is_plugged_in: bool,
    }

    #[must_use]
    pub fn get_battery_info() -> Option<BatteryInfo> {
        None
    }
}
