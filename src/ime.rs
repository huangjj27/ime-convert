// use winapi::shared::minwindef::HIMC;
// use winapi::shared::minwindef::HWND;

// use std::raw::CString;

// pub struct Ime {
//     handle: ImeHandle,
//     desc: ImeDescription,
//     status: ConversionStatus
// }

// type ImeHandle = HIMC;
// type ImeDescription = CString;

// pub enum ConversionStatus {
//     Native(Locale),
//     AlphaNumeric,
// }

// impl Ime {
//     pub fn new() -> Self;
//     pub fn desc(&self) -> &str;
//     pub fn conversion(&self) -> &ConversionStatus;

//     // when we set conversion, we use self::handle to modify the Window's conversion.
//     pub fn set_conversion(&self, cs: ConversionStatus);

//     fn for_window(w: HWND) -> Self;
// }

// impl Drop for Ime {
//     fn drop(&mut self) {

//     }
// }
