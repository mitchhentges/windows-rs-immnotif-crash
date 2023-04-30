use std::thread::sleep;
use std::time::Duration;
use windows::core::PCWSTR;
use windows::Win32::Media::Audio;
use windows::Win32::Media::Audio::{
    EDataFlow, ERole, IMMNotificationClient, IMMNotificationClient_Impl,
};
use windows::Win32::System::Com;

fn main() {
    unsafe {
        Com::CoInitializeEx(None, Com::COINIT_APARTMENTTHREADED).unwrap();
        let device_enumerator: Audio::IMMDeviceEnumerator =
            Com::CoCreateInstance(&Audio::MMDeviceEnumerator, None, Com::CLSCTX_INPROC_SERVER)
                .unwrap();

        let vcallback = CustomImmNotificationClient {};
        let interface: IMMNotificationClient = vcallback.into();

        device_enumerator
            .RegisterEndpointNotificationCallback(&interface)
            .unwrap();
    }

    println!("Registered notification client - disable/enable a device to cause a crash in windbg");
    sleep(Duration::from_secs(9999));
    println!("Sleep completed without crash");
}

#[windows::core::implement(IMMNotificationClient)]
pub struct CustomImmNotificationClient;

impl IMMNotificationClient_Impl for CustomImmNotificationClient {
    fn OnDeviceStateChanged(&self, _: &PCWSTR, _: u32) -> windows::core::Result<()> {
        Ok(())
    }

    fn OnDeviceAdded(&self, _: &PCWSTR) -> windows::core::Result<()> {
        Ok(())
    }

    fn OnDeviceRemoved(&self, _: &PCWSTR) -> windows::core::Result<()> {
        Ok(())
    }

    fn OnDefaultDeviceChanged(
        &self,
        _: EDataFlow,
        _: ERole,
        _: &PCWSTR,
    ) -> windows::core::Result<()> {
        Ok(())
    }

    fn OnPropertyValueChanged(
        &self,
        _: &PCWSTR,
        _: &windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY,
    ) -> windows::core::Result<()> {
        Ok(())
    }
}
