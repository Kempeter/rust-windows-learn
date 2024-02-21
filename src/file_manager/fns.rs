use std::collections::HashMap;
use std::ffi::CString;
use std::mem::MaybeUninit;

use windows::core::PCSTR;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::Foundation::MAX_PATH;
use windows::Win32::Foundation::NOERROR;
use windows::Win32::Storage::FileSystem::{CreateDirectoryA, GetDiskFreeSpaceA};
use windows::Win32::System::WindowsProgramming::FILE_DOES_NOT_EXIST;
use windows::Win32::UI::Shell::PathFileExistsA;

pub fn options() {
    println!("1. Get Disk Space");
    println!("2. Get Free Space");
    println!("3. Create Directory");
    println!("4. QUIT");
}

fn get_disk_details() -> HashMap<&'static str, u32> {
    let lprootpathname = CString::new("C:\\").expect("Failed to create CString");
    let mut lpsectorspercluster = MaybeUninit::<u32>::uninit();
    let mut lpbytespersector = MaybeUninit::<u32>::uninit();
    let mut lpnumberoffreeclusters = MaybeUninit::<u32>::uninit();
    let mut lptotalnumberofclusters = MaybeUninit::<u32>::uninit();

    unsafe {
        GetDiskFreeSpaceA(
            PCSTR(lprootpathname.as_ptr() as *const u8),
            Some(lpsectorspercluster.as_mut_ptr()),
            Some(lpbytespersector.as_mut_ptr()),
            Some(lpnumberoffreeclusters.as_mut_ptr()),
            Some(lptotalnumberofclusters.as_mut_ptr()),
        )
        .expect("couldn't get disk space");

        let mut disk_details = HashMap::new();
        disk_details.insert("lpsectorspercluster", lpsectorspercluster.assume_init());
        disk_details.insert("lpbytespersector", lpbytespersector.assume_init());
        disk_details.insert(
            "lpnumberoffreeclusters",
            lpnumberoffreeclusters.assume_init(),
        );
        disk_details.insert(
            "lptotalnumberofclusters",
            lptotalnumberofclusters.assume_init(),
        );

        disk_details
    }
}
pub fn get_disk_space() -> u32 {
    let details = get_disk_details();

    // Calculate the space of the disk in GB
    let disk_space: u32 = details.get("lptotalnumberofclusters").copied().unwrap_or(0) / 1024
        * details.get("lpsectorspercluster").copied().unwrap_or(0)
        / 1024
        * details.get("lpbytespersector").copied().unwrap_or(0)
        / 1024;

    disk_space
}

pub fn get_free_space() -> f32 {
    let details = get_disk_details();
    let free_space: f32 = details.get("lpsectorspercluster").copied().unwrap_or(0) as f32 / 1024.0
        * details.get("lpbytespersector").copied().unwrap_or(0) as f32
        / 1024.0
        * details.get("lpnumberoffreeclusters").copied().unwrap_or(0) as f32
        / 1024.0;

    free_space
}

pub fn create_dir(path: &str, name: &str) {
    if name.len() as u32 > MAX_PATH {
        panic!("The name is too long! Max length: {}", MAX_PATH)
    }
    let full_path = CString::new(format!("{}{}", path, name)).expect("Failed to create CString");
    let exists = unsafe { PathFileExistsA(PCSTR(full_path.as_ptr() as *const u8)) };

    match exists {
        Ok(_) => {
            println!("Already exists!")
        }
        Err(_) => {
            unsafe {
                let result = CreateDirectoryA(PCSTR(full_path.as_ptr() as *const u8), None);

                match result {
                    Ok(_) => {
                        println!("'{}' created successfully.", name);
                    }
                    Err(_) => {
                        println!("{:?}", GetLastError())
                    }
                }
            };
        }
    }
}
