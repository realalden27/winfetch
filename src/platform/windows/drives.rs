use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt as _;
use windows_sys::Win32::Storage::FileSystem::{GetDiskFreeSpaceExW, GetDriveTypeW, GetLogicalDriveStringsW};

/// Drive type constant representing a fixed disk (e.g. hard disk drive or flash drive).
const DRIVE_FIXED: u32 = 3;

#[derive(Debug, Clone, serde::Serialize)]
pub struct DriveInfo {
    pub mount: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub used_percentage: u8,
}

/// Discovers and measures all local fixed hard drives and SSDs without blocking on network drives.
#[must_use]
pub fn get_fixed_drives() -> Vec<DriveInfo> {
    let mut buffer = vec![0u16; 512];
    let len = unsafe { GetLogicalDriveStringsW(buffer.len() as u32, buffer.as_mut_ptr()) };

    if len == 0 || len as usize > buffer.len() {
        return Vec::new();
    }

    buffer.truncate(len as usize);

    let mut drives = Vec::new();
    let mut start = 0;

    for i in 0..buffer.len() {
        if buffer[i] == 0 {
            if i > start {
                let slice = &buffer[start..=i]; // include null terminator for win32 api calls
                let drive_type = unsafe { GetDriveTypeW(slice.as_ptr()) };

                // Strictly filter for local fixed drives only (SSD/NVMe/HDD)
                if drive_type == DRIVE_FIXED {
                    let mut free_bytes_available: u64 = 0;
                    let mut total_number_of_bytes: u64 = 0;
                    let mut total_number_of_free_bytes: u64 = 0;

                    let ok = unsafe {
                        GetDiskFreeSpaceExW(
                            slice.as_ptr(),
                            std::ptr::addr_of_mut!(free_bytes_available),
                            std::ptr::addr_of_mut!(total_number_of_bytes),
                            std::ptr::addr_of_mut!(total_number_of_free_bytes),
                        )
                    };

                    if ok != 0 && total_number_of_bytes > 0 {
                        let mount_str = OsString::from_wide(&slice[..slice.len().saturating_sub(1)])
                            .into_string()
                            .unwrap_or_else(|_| "C:\\".to_string());

                        let used = total_number_of_bytes.saturating_sub(total_number_of_free_bytes);
                        let percentage = ((used as f64 / total_number_of_bytes as f64) * 100.0).round() as u8;

                        drives.push(DriveInfo {
                            mount: mount_str.trim_end_matches('\\').to_string(),
                            total_bytes: total_number_of_bytes,
                            used_bytes: used,
                            free_bytes: total_number_of_free_bytes,
                            used_percentage: percentage.min(100),
                        });
                    }
                }
            }
            start = i + 1;
        }
    }

    drives
}
