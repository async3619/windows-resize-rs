#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use windows::core::{s};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, TRUE, WPARAM};
use windows::Win32::UI::Shell::{DefSubclassProc, RemoveWindowSubclass, SetWindowSubclass};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowExA, HTBOTTOM, HTBOTTOMLEFT, HTBOTTOMRIGHT, HTCLIENT, HTRIGHT, HTTOPRIGHT, WM_NCDESTROY, WM_NCHITTEST};

fn inject(mut handle: HWND) -> bool {
  unsafe {
    let legacy_window_handle = FindWindowExA(handle, None, s!("Chrome_RenderWidgetHostHWND"), None);
    if legacy_window_handle.is_ok() {
      handle = legacy_window_handle.unwrap();
    }

    SetWindowSubclass(handle, Some(subclass_proc), 0, 0) == TRUE
  }
}

unsafe extern "system" fn subclass_proc(
  hwnd: HWND,
  msg: u32,
  wparam: WPARAM,
  lparam: LPARAM,
  _: usize,
  _: usize,
) -> LRESULT {
  if msg == WM_NCHITTEST {
    let hittest = DefSubclassProc(hwnd, msg, wparam, lparam);

    if hittest == LRESULT(HTRIGHT as _) ||
      hittest == LRESULT(HTBOTTOM as _) ||
      hittest == LRESULT(HTBOTTOMRIGHT as _) ||
      hittest == LRESULT(HTTOPRIGHT as _) ||
      hittest == LRESULT(HTBOTTOMLEFT as _) {
      return LRESULT(HTCLIENT as _);
    }
  }

  if msg == WM_NCDESTROY {
    let _ = RemoveWindowSubclass(hwnd, Some(subclass_proc), 0);
  }

  return DefSubclassProc(hwnd, msg, wparam, lparam);
}

#[napi]
pub fn restrict_resize(handle: i32) -> bool {
  inject(HWND(handle as _))
}
