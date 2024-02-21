mod get_user_name;
mod cursor;
mod get_disk_free_space_a;
mod file_manager::file_manager_main;

use get_user_name::get_user_name;
use cursor::cursor;
use get_disk_free_space_a::get_disk_free_space_a;
use file_manager::file_manager_main::file_manager;

fn main() {
    //get_user_name();
    //cursor();
    //get_disk_free_space_a()
    file_manager();
}
