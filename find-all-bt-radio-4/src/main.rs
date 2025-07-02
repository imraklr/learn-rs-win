use windows::Win32::{
    Devices::Bluetooth::*,
    Foundation::{CloseHandle, GetLastError, HANDLE},
};

fn main() {

    let mut n_radios = 0;

    let p_btfrp: *const BLUETOOTH_FIND_RADIO_PARAMS = &BLUETOOTH_FIND_RADIO_PARAMS {dwSize: std::mem::size_of::<BLUETOOTH_FIND_RADIO_PARAMS>() as u32};
    let mut h_radio = HANDLE::default();

    let handle_enumeration = unsafe { match BluetoothFindFirstRadio(p_btfrp, &mut h_radio) {
        Ok(handle_value) => {
            println!("Found first radio!!");
            n_radios+=1;
            Some(handle_value)
        },
        Err(_) => {
            println!("ERROR: Either radio switched off or NA!!!\n ERROR CODE: {}", GetLastError().0);
            return
        },
    }};

    println!("Searching for other radios...");

    let h_find = handle_enumeration.unwrap();
    loop {
        match unsafe { BluetoothFindNextRadio(h_find, &mut h_radio) } {
            Ok(_) => n_radios += 1,
            Err(_) => {
                println!("No more radios found.");
                println!("Total number of radios = {}", n_radios);
                break
            },
        }
    }

    
    // Close all the resources in reverse initiation order
    unsafe {
        match BluetoothFindRadioClose(h_find) {
            Ok(_) => {},
            Err(err) => {
                println!("Couldn't close find radio. ERROR CODE: {}", err);
            }
        }

        match CloseHandle(h_radio) {
            Ok(_) => {},
            Err(err) => {
                println!("Couldn't close handle to the radio. ERROR CODE: {}", err);
            }
        }
    }
}