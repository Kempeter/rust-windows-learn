mod cursor;
mod file_manager_main;
mod get_disk_free_space_a;
mod get_user_name;

use cursor::cursor;
use file_manager_main::file_manager;
use get_disk_free_space_a::get_disk_free_space_a;
use get_user_name::get_user_name;

fn main() {
    //get_user_name();
    //cursor();
    //get_disk_free_space_a()
    file_manager();
}
