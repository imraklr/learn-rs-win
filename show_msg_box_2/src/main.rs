use windows::{core::*, Win32::UI::WindowsAndMessaging::*};

fn main() {
    unsafe {
        MessageBoxA(None, s!("Ansi"), s!("World"), MB_OK | MB_HELP);
        MessageBoxW(None, w!("Wide"), w!("World"), MB_OK);
    }
}
