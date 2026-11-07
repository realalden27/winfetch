use windows_sys::Win32::Graphics::Gdi::{DEVMODEW, ENUM_CURRENT_SETTINGS, EnumDisplaySettingsW};
use windows_sys::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};
use windows_sys::Win32::System::SystemInformation::{GetTickCount64, GlobalMemoryStatusEx, MEMORYSTATUSEX};

/// High-level memory statistics in bytes.
#[derive(Debug, Clone, Copy, Default)]
pub struct MemoryStats {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
}

/// Retrieves system memory statistics instantly via `GlobalMemoryStatusEx`.
#[must_use]
pub fn get_memory_stats() -> Option<MemoryStats> {
    unsafe {
        let mut status: MEMORYSTATUSEX = std::mem::zeroed();
        status.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;

        if GlobalMemoryStatusEx(&mut status) == 0 {
            return None;
        }

        let total = status.ullTotalPhys;
        let avail = status.ullAvailPhys;
        let used = total.saturating_sub(avail);

        Some(MemoryStats {
            total_bytes: total,
            used_bytes: used,
            free_bytes: avail,
        })
    }
}

/// Retrieves the total system uptime since boot in seconds via `GetTickCount64`.
#[must_use]
pub fn get_uptime_seconds() -> u64 {
    let millis = unsafe { GetTickCount64() };
    millis / 1000
}

/// Display resolution and refresh rate.
#[derive(Debug, Clone, Copy)]
pub struct DisplayInfo {
    pub width: u32,
    pub height: u32,
    pub refresh_rate_hz: u32,
}

/// Retrieves primary display resolution and refresh rate via `EnumDisplaySettingsW`.
#[must_use]
pub fn get_display_info() -> Option<DisplayInfo> {
    unsafe {
        let mut devmode: DEVMODEW = std::mem::zeroed();
        devmode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;

        let ok = EnumDisplaySettingsW(std::ptr::null(), ENUM_CURRENT_SETTINGS, &mut devmode);
        if ok == 0 {
            return None;
        }

        Some(DisplayInfo {
            width: devmode.dmPelsWidth,
            height: devmode.dmPelsHeight,
            refresh_rate_hz: devmode.dmDisplayFrequency,
        })
    }
}

/// Battery state information.
#[derive(Debug, Clone, Copy)]
pub struct BatteryInfo {
    pub percentage: u8,
    pub is_plugged_in: bool,
}

/// Retrieves battery status if present on device (e.g. laptop or tablet).
#[must_use]
pub fn get_battery_info() -> Option<BatteryInfo> {
    unsafe {
        let mut status: SYSTEM_POWER_STATUS = std::mem::zeroed();
        if GetSystemPowerStatus(&mut status) == 0 {
            return None;
        }

        // 255 indicates unknown battery status or no battery present (desktop)
        if status.BatteryLifePercent > 100 {
            return None;
        }

        Some(BatteryInfo {
            percentage: status.BatteryLifePercent,
            is_plugged_in: status.ACLineStatus == 1,
        })
    }
}
