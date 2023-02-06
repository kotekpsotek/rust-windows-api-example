use windows::Win32::System::Power::{ GetSystemPowerStatus, SYSTEM_POWER_STATUS };
use windows::Win32::System::SystemInformation::{ GetSystemTime, GlobalMemoryStatus, MEMORYSTATUS };

#[cfg(target_os = "windows")]
fn main() {
    if cfg!(not(target_os = "windows")) {
        panic!("Ups this program can only interact with windows API. Sorry!");
    };

    unsafe {
        // Get battery percentage status (when you work on laptop)
        let pwr = &mut SYSTEM_POWER_STATUS::default() as *mut SYSTEM_POWER_STATUS;
        let st = GetSystemPowerStatus(pwr).as_bool();

        if st {
            println!("Your battery has got: {:?}%", (*pwr).BatteryLifePercent)
        }
        else {
            panic!("Couldn't get battery status!");
        }

        // Get system time information
        let sys_time = GetSystemTime();
        println!("Today is (from your system time): {y}-{m}-{d}", y = sys_time.wYear, m = sys_time.wMonth, d = sys_time.wDay);

        // Information about disk usage
        let mut mem_stat = MEMORYSTATUS::default();
        GlobalMemoryStatus(&mut mem_stat as *mut MEMORYSTATUS);
        let avaiable_diskspace = mem_stat.dwAvailPhys / 1_024;

        
        println!("You have avaiable: {avaiable_diskspace:?}MB disk space on your device!");
    }
}
