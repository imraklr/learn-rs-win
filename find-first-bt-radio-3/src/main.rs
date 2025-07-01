use windows::{
    Win32::{
        Devices::Bluetooth::*, Foundation::{HANDLE, GetLastError},
        Foundation::CloseHandle,
    },
};

fn main() {
    let radio_params: *const BLUETOOTH_FIND_RADIO_PARAMS = &BLUETOOTH_FIND_RADIO_PARAMS {dwSize: std::mem::size_of::<BLUETOOTH_FIND_RADIO_PARAMS>() as u32};
    let mut handle = HANDLE::default();

    let nexts_handle = match unsafe { BluetoothFindFirstRadio(radio_params, &mut handle) } {
        Ok(o) => Some(o),
        Err(_) => {
            println!("ERROR: No bluetooth radio found. Either bluetooth is not available or switched off.");
            None
        },
    };

    match nexts_handle {
        Some(handle_value) => {
            println!("We found a bluetooth radio!!");

            // Close the handles
            unsafe {
                match BluetoothFindRadioClose(handle_value) {
                    Ok(_) => println!("Bluetooth Find radio closed successfully!"),
                    Err(e) => println!("ERROR: {}", e),
                }

                match CloseHandle(handle) {
                    Ok(_) => println!("Successfully closed handle to the first radio."),
                    Err(e) => println!("Failed to close handle to the first radio!!, cause: {}", e),
                }
            };
        },
        None => {
            unsafe {
                println!("ERROR CODE: {}", GetLastError().0);
                return
            }
        }
    }
}