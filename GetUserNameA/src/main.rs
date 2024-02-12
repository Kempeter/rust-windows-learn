use windows::core::PSTR;
use windows::Win32::System::WindowsProgramming::GetUserNameA;

fn main() {
    unsafe {
        let mut size = 0;
        let result = GetUserNameA(PSTR::null(), &mut size);
        let mut lpbuffer = Vec::with_capacity(size as usize);
        let result = GetUserNameA(PSTR(lpbuffer.as_mut_ptr()), &mut size);

        lpbuffer.set_len(size as usize);

        let user_name = String::from_utf8_lossy(&lpbuffer).to_string();
        println!("{}", user_name);
    };
}
