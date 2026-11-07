#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt as _;
use windows_sys::Win32::Foundation::ERROR_SUCCESS;
use windows_sys::Win32::System::Registry::{
    HKEY, HKEY_LOCAL_MACHINE, KEY_READ, REG_DWORD, REG_QWORD, REG_SZ, RegCloseKey, RegEnumKeyExW, RegOpenKeyExW,
    RegQueryValueExW,
};

/// Helper to encode a string slice as a null-terminated UTF-16 wide string vector.
fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Safely reads a string (REG_SZ) value from a registry path under HKLM.
#[must_use]
pub fn read_hklm_string(subkey: &str, value_name: &str) -> Option<String> {
    read_string(HKEY_LOCAL_MACHINE, subkey, value_name)
}

/// Safely reads a DWORD (u32) value from a registry path under HKLM.
#[must_use]
pub fn read_hklm_u32(subkey: &str, value_name: &str) -> Option<u32> {
    read_u32(HKEY_LOCAL_MACHINE, subkey, value_name)
}

/// Safely reads a QWORD (u64) value from a registry path under HKLM.
#[must_use]
pub fn read_hklm_u64(subkey: &str, value_name: &str) -> Option<u64> {
    read_u64(HKEY_LOCAL_MACHINE, subkey, value_name)
}

/// Reads a string value from any given HKEY root.
#[must_use]
pub fn read_string(root: HKEY, subkey: &str, value_name: &str) -> Option<String> {
    let subkey_w = to_wide(subkey);
    let mut hkey: HKEY = std::ptr::null_mut();

    let open_status = unsafe { RegOpenKeyExW(root, subkey_w.as_ptr(), 0, KEY_READ, &mut hkey) };

    if open_status != ERROR_SUCCESS || hkey.is_null() {
        return None;
    }

    let val_name_w = to_wide(value_name);
    let mut val_type: u32 = 0;
    let mut byte_len: u32 = 0;

    // First call to determine size in bytes
    let size_status = unsafe {
        RegQueryValueExW(
            hkey,
            val_name_w.as_ptr(),
            std::ptr::null_mut(),
            &mut val_type,
            std::ptr::null_mut(),
            &mut byte_len,
        )
    };

    if size_status != ERROR_SUCCESS || byte_len == 0 {
        unsafe {
            RegCloseKey(hkey);
        }
        return None;
    }

    // Allocate buffer for UTF-16 characters
    let u16_count = (byte_len as usize).div_ceil(2);
    let mut buffer: Vec<u16> = vec![0; u16_count];

    let read_status = unsafe {
        RegQueryValueExW(
            hkey,
            val_name_w.as_ptr(),
            std::ptr::null_mut(),
            &mut val_type,
            buffer.as_mut_ptr().cast(),
            &mut byte_len,
        )
    };

    unsafe {
        RegCloseKey(hkey);
    }

    if read_status != ERROR_SUCCESS || val_type != REG_SZ {
        return None;
    }

    // Trim trailing nulls
    while buffer.last() == Some(&0) {
        buffer.pop();
    }

    OsString::from_wide(&buffer).into_string().ok()
}

/// Reads a 32-bit integer (DWORD) from the registry.
#[must_use]
pub fn read_u32(root: HKEY, subkey: &str, value_name: &str) -> Option<u32> {
    let subkey_w = to_wide(subkey);
    let mut hkey: HKEY = std::ptr::null_mut();

    let open_status = unsafe { RegOpenKeyExW(root, subkey_w.as_ptr(), 0, KEY_READ, &mut hkey) };

    if open_status != ERROR_SUCCESS || hkey.is_null() {
        return None;
    }

    let val_name_w = to_wide(value_name);
    let mut val_type: u32 = 0;
    let mut data: u32 = 0;
    let mut byte_len: u32 = std::mem::size_of::<u32>() as u32;

    let status = unsafe {
        RegQueryValueExW(
            hkey,
            val_name_w.as_ptr(),
            std::ptr::null_mut(),
            &mut val_type,
            std::ptr::addr_of_mut!(data).cast(),
            &mut byte_len,
        )
    };

    unsafe {
        RegCloseKey(hkey);
    }

    if status == ERROR_SUCCESS && val_type == REG_DWORD {
        Some(data)
    } else {
        None
    }
}

/// Reads a 64-bit integer (QWORD) or combined 32-bit integer from the registry.
#[must_use]
pub fn read_u64(root: HKEY, subkey: &str, value_name: &str) -> Option<u64> {
    let subkey_w = to_wide(subkey);
    let mut hkey: HKEY = std::ptr::null_mut();

    let open_status = unsafe { RegOpenKeyExW(root, subkey_w.as_ptr(), 0, KEY_READ, &mut hkey) };

    if open_status != ERROR_SUCCESS || hkey.is_null() {
        return None;
    }

    let val_name_w = to_wide(value_name);
    let mut val_type: u32 = 0;
    let mut data: u64 = 0;
    let mut byte_len: u32 = std::mem::size_of::<u64>() as u32;

    let status = unsafe {
        RegQueryValueExW(
            hkey,
            val_name_w.as_ptr(),
            std::ptr::null_mut(),
            &mut val_type,
            std::ptr::addr_of_mut!(data).cast(),
            &mut byte_len,
        )
    };

    unsafe {
        RegCloseKey(hkey);
    }

    if status == ERROR_SUCCESS && val_type == REG_QWORD {
        Some(data)
    } else {
        None
    }
}

/// Enumerates all immediate subkey names under a given registry path.
#[must_use]
pub fn enum_subkeys(root: HKEY, subkey: &str) -> Vec<String> {
    let subkey_w = to_wide(subkey);
    let mut hkey: HKEY = std::ptr::null_mut();

    let open_status = unsafe { RegOpenKeyExW(root, subkey_w.as_ptr(), 0, KEY_READ, &mut hkey) };

    if open_status != ERROR_SUCCESS || hkey.is_null() {
        return Vec::new();
    }

    let mut subkeys = Vec::new();
    let mut index = 0;

    loop {
        let mut name_buf = vec![0u16; 256];
        let mut name_len: u32 = name_buf.len() as u32;

        let status = unsafe {
            RegEnumKeyExW(
                hkey,
                index,
                name_buf.as_mut_ptr(),
                &mut name_len,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };

        if status != ERROR_SUCCESS {
            break;
        }

        name_buf.truncate(name_len as usize);
        if let Ok(name_str) = OsString::from_wide(&name_buf).into_string() {
            subkeys.push(name_str);
        }

        index = index.saturating_add(1);
    }

    unsafe {
        RegCloseKey(hkey);
    }

    subkeys
}
