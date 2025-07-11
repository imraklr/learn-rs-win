use windows::{
    Devices::Bluetooth::*,
};

fn main() {
    let bt_adapter = BluetoothAdapter::GetDefaultAsync().unwrap().get().unwrap();

    println!("Are classic secure connections supported: {}", bt_adapter.AreClassicSecureConnectionsSupported().unwrap());
    println!("Are low energy secure connections supported: {}", bt_adapter.AreLowEnergySecureConnectionsSupported().unwrap());
    println!("Bluetooth Address (MAC): {}", bt_adapter.BluetoothAddress().unwrap());
    println!("Device ID: {}", bt_adapter.DeviceId().unwrap());
    println!("Does adapter support advertisement offload? {}", bt_adapter.IsAdvertisementOffloadSupported().unwrap());
    println!("Does the adapter support low energy central role? {}", bt_adapter.IsCentralRoleSupported().unwrap());
    println!("Does the adapter support class bluetooth transfers? {}", bt_adapter.IsClassicSupported().unwrap());
    println!("Does the adapter support 5.0 Extended Advertising? {}", bt_adapter.IsExtendedAdvertisingSupported().unwrap());
    println!("Is low energy supported? {}", bt_adapter.IsLowEnergySupported().unwrap());
    println!("Does adapter support low energy peripheral role? {}", bt_adapter.IsPeripheralRoleSupported().unwrap());
    println!("Maximum advertisement length that can be published by this adapter = {}", bt_adapter.MaxAdvertisementDataLength().unwrap());
}
