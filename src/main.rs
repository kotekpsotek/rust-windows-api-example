use windows::Data::Xml::Dom::{XmlDocument, IXmlNode};
use windows::Win32::System::Power::{ GetSystemPowerStatus, SYSTEM_POWER_STATUS };
use windows::Win32::System::SystemInformation::{ GetSystemTime, GlobalMemoryStatus, MEMORYSTATUS };
use windows::UI::Notifications::{ToastNotification, ToastNotificationManager, ToastTemplateType};
use windows::core::HSTRING;

#[non_exhaustive]
enum NotificationToastNodes {
    Text,
    Action
}

struct NotificatioNodeAccess;
impl NotificatioNodeAccess {
    /// Get simple/clean in code (= for bot convinient) access to element from Toast Notification
    fn access_to_node_name(xml_toast_content: &XmlDocument, r#type: NotificationToastNodes) -> Option<IXmlNode> {
        use NotificationToastNodes::*;

        if matches!(r#type, Action) {
            // Prepare element name
            let el_name = HSTRING::from_wide(
                &b"action".iter()
                .map(|v| (*v) as u16)
                .collect::<Vec<u16>>()[..]
            )
            .map_or_else(|_| None, |el| Some(el))?;
            
            // Get element by it name
            let action_el = xml_toast_content.GetElementsByTagName(&el_name)
                .unwrap()
                .Item(0)
                .map_or_else(|_| None, |el| Some(el))?;

            return Some(action_el);
        } else if matches!(r#type, Text) {
            // Prepare element name
            let el_name = HSTRING::from_wide(
                &b"text".iter()
                .map(|v| (*v) as u16)
                .collect::<Vec<u16>>()[..]
            )
                .unwrap();

            // Get element by it name
            let text_el = xml_toast_content.GetElementsByTagName(&el_name)
                .unwrap()
                .Item(0)
                .unwrap();

            return Some(text_el);
        }

        None
    }

    /// Add text content to element
    /// Return operation Result
    fn add_text_content(element: &IXmlNode, txt: &str) -> Result<(), ()> {
        // Create text "container"
        let new_text = HSTRING::from_wide(
            &txt.as_bytes()
                .iter()
                .map(|v| (*v) as u16)
                .collect::<Vec<u16>>()[..]
        )
            .map_err(|_| ())?;

        // Set text
        element.SetInnerText(&new_text)
            .map_err(|_| ())?;

        Ok(())
    }
}

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

        // Send notification
        let notify = {
            // Get access to notification template (XML)
            let t_content = ToastNotificationManager::GetTemplateContent(ToastTemplateType(4))
                .unwrap();

            // Get access to template element for text
            // "el_name" must refers to existsing toast notification node name from XML specification. Here you have adress to sources defining all XML nodes for 'Toast Notifications': https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/schema-root (xml schema) or https://learn.microsoft.com/en-us/windows/apps/design/shell/tiles-and-notifications/toast-schema (Notification library objects [rather existing in C#])
            let text_el = NotificatioNodeAccess::access_to_node_name(&t_content, NotificationToastNodes::Text)
                .expect("Couldn't get element");

            // Insert text to notification text element
            NotificatioNodeAccess::add_text_content(&text_el, "Hello windows word!")
                .expect("Couldn't insert text to found element!");

            // Get access to "action" element from notification
                // FIXME: This element (like html button) doesn't work
            /* let action_el = NotificatioNodeAccess::access_to_node_name(&t_content, NotificationToastNodes::Action)
                .expect("Couldn't get element");
    
            NotificatioNodeAccess::add_text_content(&action_el, "Do you agree?")
                .expect("Couldn't insert text to found element!"); */

            // Create ToastNotification from modified template
            ToastNotification::CreateToastNotification(&t_content)
                .unwrap()
        };
        
        let location = HSTRING::from_wide(
            &b"../target/debug/rust_windows_api.exe".iter()
            .map(|v| (*v) as u16)
            .collect::<Vec<u16>>()[..]
        )
            .unwrap(); // You must attach correct path to your existing executable file/file or folder location (to which location notification refers - it will be displaying into toast notification top part)
        let _show = ToastNotificationManager::CreateToastNotifierWithId(&location)
            .unwrap()
            .Show(&notify)
            .expect("Couldn't show notification"); // here notification should be displaying
    }
}
