extern crate winapi;

use std::ffi::c_void;
use std::ptr;
use winapi::shared::minwindef::{LPARAM, LRESULT, WPARAM};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{
    CallNextHookEx, GetMessageW, SetWindowsHookExW, UnhookWindowsHookEx, HC_ACTION,
    KBDLLHOOKSTRUCT, MSLLHOOKSTRUCT, WH_KEYBOARD_LL, WH_MOUSE_LL, WM_LBUTTONDOWN, WM_LBUTTONUP,
    WM_MBUTTONDOWN, WM_MBUTTONUP, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_XBUTTONDOWN, WM_XBUTTONUP,
};

static mut KEYBOARD_HOOK: *mut c_void = ptr::null_mut();
static mut MOUSE_HOOK: *mut c_void = ptr::null_mut();

unsafe extern "system" fn keyboard_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code == HC_ACTION {
        let kb_data = *(l_param as *mut KBDLLHOOKSTRUCT);
        println!("{}", kb_data.vkCode);
    }
    CallNextHookEx(ptr::null_mut(), n_code, w_param, l_param)
}

unsafe extern "system" fn mouse_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code == HC_ACTION {
        let mouse_data = *(l_param as *mut MSLLHOOKSTRUCT);
        let button_id = match w_param as u32 {
            WM_LBUTTONDOWN | WM_LBUTTONUP => Some(1),
            WM_RBUTTONDOWN | WM_RBUTTONUP => Some(2),
            WM_MBUTTONDOWN | WM_MBUTTONUP => Some(3),
            WM_XBUTTONDOWN | WM_XBUTTONUP => {
                let button_id = (mouse_data.mouseData >> 16) & 0xFFFF;
                Some((button_id + 3) as u32)
            }
            _ => None,
        };

        if let Some(id) = button_id {
            println!("{}", id);
        }
    }
    CallNextHookEx(ptr::null_mut(), n_code, w_param, l_param)
}

fn main() {
    unsafe {
        let module_handle = GetModuleHandleW(ptr::null());
        let keyboard_hook =
            SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), module_handle, 0);
        KEYBOARD_HOOK = keyboard_hook as *mut c_void;

        let mouse_hook = SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_proc), module_handle, 0);
        MOUSE_HOOK = mouse_hook as *mut c_void;

        let mut msg = std::mem::zeroed();
        while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) > 0 {}

        UnhookWindowsHookEx(KEYBOARD_HOOK as *mut _);
        UnhookWindowsHookEx(MOUSE_HOOK as *mut _);
    }
}
