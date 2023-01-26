use cocoa::appkit::{NSPasteboard, NSPasteboardTypeTIFF, NSApp};
use cocoa::base::nil;
use cocoa::foundation::{NSData, NSString};
use uuid::Uuid;

pub (crate) fn save_clipboard_tiff() -> Option<String> {
    unsafe {
        let app = NSApp();
        let pasteboard = NSPasteboard::generalPasteboard(app);

        let tiff = pasteboard.dataForType(NSPasteboardTypeTIFF);
        if tiff.is_null() {
            return None;
        }

        let id = Uuid::new_v4();
        let destination = std::env::temp_dir();
        let path = destination.join(format!("{}.tiff", id));
        let path_str = match path.to_str() {
            Some(path) => path,
            None => return None,
        };

        if NSData::dataWithData_(app, tiff).writeToFile_atomically_(NSString::alloc(nil).init_str(path_str), true) {
            return Some(path_str.to_string());
        }

        None
    }
}