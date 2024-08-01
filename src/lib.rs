#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, TRUE, WPARAM};
use windows::Win32::UI::Shell::{DefSubclassProc, SetWindowSubclass};
use windows::Win32::UI::WindowsAndMessaging::{HTBOTTOM, HTBOTTOMLEFT, HTBOTTOMRIGHT, HTCLIENT, HTRIGHT, HTTOPRIGHT, WM_NCHITTEST};

fn inject(handle: HWND) -> bool {
  unsafe {
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

  return DefSubclassProc(hwnd, msg, wparam, lparam);
}

#[napi]
pub fn restrict_resize(handle: i32) -> bool {
  inject(HWND(handle as _))
}
