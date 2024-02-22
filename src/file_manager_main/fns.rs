use std::collections::HashMap;
use std::mem::MaybeUninit;

use windows::core::s;
use windows::core::PCSTR;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::Foundation::MAX_PATH;
use windows::Win32::Storage::FileSystem::{CreateDirectoryA, GetDiskFreeSpaceA};
use windows::Win32::UI::Shell::PathFileExistsA;

pub fn options() {
    println!("1. Get Disk Space");
    println!("2. Get Free Space");
    println!("3. Create Directory");
    println!("4. QUIT");
}

fn get_disk_details() -> HashMap<&'static str, u32> {
    let lprootpathname = s!("C:\\");
    let mut lpsectorspercluster = MaybeUninit::<u32>::uninit();
    let mut lpbytespersector = MaybeUninit::<u32>::uninit();
    let mut lpnumberoffreeclusters = MaybeUninit::<u32>::uninit();
    let mut lptotalnumberofclusters = MaybeUninit::<u32>::uninit();

    unsafe {
        GetDiskFreeSpaceA(
            lprootpathname,
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
    let full_path_string = format!("{}{}", path, name);
    let full_path = PCSTR::from_raw(full_path_string.as_ptr());
    let exists = unsafe { PathFileExistsA(full_path) };

    match exists {
        Ok(()) => {
            println!("Already exists!");
            return;
        }
        Err(_) => {}
    }

    unsafe {
        let result = CreateDirectoryA(full_path, None);

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
