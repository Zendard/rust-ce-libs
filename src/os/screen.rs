use std::ffi::{c_char, CString};

extern "C" {
    fn boot_NewLine();
    fn os_ClrHome();
    fn os_ClrHomeFull();
    fn os_NewLine();
    fn os_DisableCursor();
    fn os_EnableCursor();
    fn os_SetCursorPos(curRow: u8, curCol: u8);
    fn os_GetCursorPos(curRow: *mut usize, curCol: *mut usize);
    fn os_PutStrFull(string: *const c_char) -> u8;
    fn os_PutStrLine(string: *const c_char) -> u8;
    fn os_MoveUp();
    fn os_MoveDown();
    fn os_HomeUp();
    fn os_ClrLCDFull();
    fn os_ClrLCD();
    fn os_ClrTxtShd();
    fn os_DisableHomeTextBuffer();
    fn os_EnableHomeTextBuffer();
    fn os_GetStringInput(prompt: *const c_char, buf: *mut c_char, bufsize: usize);
    fn os_GetTokenInput(prompt: *const c_char, buf: *mut c_char, bufsize: usize) -> usize;
    fn os_FonstSelect(font: usize);
    fn os_FontGetID() -> u8;
    fn os_FontGetWidth(string: *const c_char) -> u32;
    fn os_FontGetHeight() -> u32;
    fn os_FontDrawText(string: *const c_char, col: u16, row: u8) -> u32;
    fn os_FontDrawTransText(string: *const c_char, col: u16, row: u8) -> u32;
    fn os_SetDrawFGColor(color: u32);
    fn os_GetDrawFGColor() -> u32;
    fn os_SetDrawBGColor(color: u32);
    fn os_GetDrawBGColor() -> u32;
}

pub struct CursorPosition {
    pub row: u8,
    pub column: u8,
}

pub fn clear_homescreen() {
    unsafe {
        os_ClrHome();
    }
}

pub fn clear_homescreen_full() {
    unsafe {
        os_ClrHomeFull();
    }
}

pub fn newline_no_scroll() {
    unsafe {
        boot_NewLine();
    }
}

pub fn newline_scroll() {
    unsafe {
        os_NewLine();
    }
}

pub fn disable_cursor() {
    unsafe {
        os_DisableCursor();
    }
}

pub fn enable_cursor() {
    unsafe {
        os_EnableCursor();
    }
}

pub fn set_cursor_position(cursor_position: CursorPosition) {
    unsafe {
        os_SetCursorPos(cursor_position.row, cursor_position.column);
    }
}

pub fn get_cursor_position() -> CursorPosition {
    let row: &mut [usize] = &mut [];
    let column: &mut [usize] = &mut [];
    unsafe {
        os_GetCursorPos(row.as_mut_ptr(), column.as_mut_ptr());
    }
    return CursorPosition {
        row: row[0].to_owned() as u8,
        column: column[0].to_owned() as u8,
    };
}

// Returns true if the string fits on the screen
pub fn print_string(string: &str) -> bool {
    let c_string = CString::new(string).unwrap();
    let c_chars = c_string.as_ptr() as *const c_char;
    let result = unsafe { os_PutStrFull(c_chars) };
    match result {
        0 => false,
        _ => true,
    }
}

// Returns true if the string fits on the screen
pub fn print_string_line(string: &str) -> bool {
    let c_string = CString::new(string).unwrap();
    let c_chars = c_string.as_ptr() as *const c_char;
    let result = unsafe { os_PutStrLine(c_chars) };
    match result {
        0 => false,
        _ => true,
    }
}

pub fn scroll_screen_up() {
    unsafe {
        os_MoveUp();
    }
}

pub fn scroll_screen_down() {
    unsafe {
        os_MoveDown();
    }
}

pub fn clear_lcd_full() {
    unsafe {
        os_ClrLCDFull();
    }
}

pub fn clear_lcd() {
    unsafe {
        os_ClrLCD();
    }
}

pub fn clear_text_shadow_area() {
    unsafe {
        os_ClrTxtShd();
    }
}

pub fn disable_home_text_buffer() {
    unsafe {
        os_DisableHomeTextBuffer();
    }
}

pub fn enable_home_text_buffer() {
    unsafe {
        os_EnableHomeTextBuffer();
    }
}

pub fn get_string_input(prompt: &str, buffer: &mut String) {
    let c_buffer: &mut [c_char] = &mut [];
    let buffer_size = buffer.len();
    let c_string_prompt = CString::new(prompt).unwrap();
    let c_char_prompt = c_string_prompt.as_ptr();
    unsafe {
        os_GetStringInput(c_char_prompt, c_buffer.as_mut_ptr(), buffer_size);
        *buffer = CString::from_raw(c_buffer.as_mut_ptr())
            .to_str()
            .unwrap()
            .to_owned();
    }
}

pub fn get_token_input(prompt: &str, buffer: &mut String) {
    let c_buffer: &mut [c_char] = &mut [];
    let buffer_size = buffer.len();
    let c_string_prompt = CString::new(prompt).unwrap();
    let c_char_prompt = c_string_prompt.as_ptr();
    unsafe {
        os_GetTokenInput(c_char_prompt, c_buffer.as_mut_ptr(), buffer_size);
        *buffer = CString::from_raw(c_buffer.as_mut_ptr())
            .to_str()
            .unwrap()
            .to_owned();
    }
}

pub fn font_get_width(string: &str) -> u32 {
    let c_string = CString::new(string).unwrap();
    let c_char = c_string.as_ptr();

    unsafe { os_FontGetWidth(c_char) }
}

pub fn font_get_height() -> u32 {
    unsafe { os_FontGetHeight() }
}

pub fn font_draw_text(string: &str, position: CursorPosition) -> u32 {
    let c_string = CString::new(string).unwrap();
    let c_char = c_string.as_ptr();

    unsafe { os_FontDrawText(c_char, position.column.into(), position.row) }
}

pub fn font_draw_transparent_text(string: &str, position: CursorPosition) -> u32 {
    let c_string = CString::new(string).unwrap();
    let c_char = c_string.as_ptr();

    unsafe { os_FontDrawTransText(c_char, position.column.into(), position.row) }
}

pub fn set_draw_foreground_color(color: u32) {
    unsafe {
        os_SetDrawFGColor(color);
    }
}

pub fn get_draw_foreground_color() -> u32 {
    unsafe { os_GetDrawFGColor() }
}

pub fn set_draw_background_color(color: u32) {
    unsafe {
        os_SetDrawBGColor(color);
    }
}

pub fn get_draw_background_color() -> u32 {
    unsafe { os_GetDrawBGColor() }
}
