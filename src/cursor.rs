use std::fs::OpenOptions;
use std::io::{self, Write};
use std::{thread, time};

use windows::Win32::{Foundation::POINT, UI::WindowsAndMessaging::GetCursorPos};

pub fn cursor() -> io::Result<()> {
    let mut cursor_pos: POINT = POINT { x: 0, y: 0 };
    let file_path = "cursor.txt";
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)
        .unwrap();

    loop {
        unsafe { GetCursorPos(&mut cursor_pos) }?;

        let x = cursor_pos.x;
        let y = cursor_pos.y;
        let content = format!("Cursor position: x={}, y={}\n", x, y);
        //file.write_all(content.as_bytes())?;
        //file.flush()?;

        print!("{}", content);

        thread::sleep(time::Duration::from_millis(50))
    }
}
