use std::ffi::CString;
use std::mem::MaybeUninit;

use windows::core::PCSTR;
use windows::Win32::Storage::FileSystem::GetDiskFreeSpaceA;

pub fn get_disk_free_space_a() {
    unsafe {
        // You can change the "C:\\" to your disk's name
        // If you want to do it automatically:
        //let lprootpathname = PCSTR:null()
        let lprootpathname = CString::new("C:\\").expect("Failed to create CString");
        let mut lpsectorspercluster = MaybeUninit::<u32>::uninit();
        let mut lpbytespersector = MaybeUninit::<u32>::uninit();
        let mut lpnumberoffreeclusters = MaybeUninit::<u32>::uninit();
        let mut lptotalnumberofclusters = MaybeUninit::<u32>::uninit();

        GetDiskFreeSpaceA(
            PCSTR(lprootpathname.as_ptr() as *const u8),
            Some(lpsectorspercluster.as_mut_ptr()),
            Some(lpbytespersector.as_mut_ptr()),
            Some(lpnumberoffreeclusters.as_mut_ptr()),
            Some(lptotalnumberofclusters.as_mut_ptr()),
        )
        .expect("couldn't get disk space");

        // Calculate the space of the disk in GB
        let disk_space =
            lptotalnumberofclusters.assume_init() / 1024 * lpsectorspercluster.assume_init() / 1024
                * lpbytespersector.assume_init()
                / 1024;

        // Calculate the free space of the disk in GB
        let free_space: f64 = lpsectorspercluster.assume_init() as f64 / 1024.0
            * lpbytespersector.assume_init() as f64
            / 1024.0
            * lpnumberoffreeclusters.assume_init() as f64
            / 1024.0;

        println!("Total disk space: {} GB", disk_space);
        println!("Free disk space: {} GB", free_space);
    };
}
