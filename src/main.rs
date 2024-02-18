mod get_user_name;
mod cursor;
mod get_disk_free_space_a;

use get_user_name::get_user_name;
use cursor::cursor;
use get_disk_free_space_a::get_disk_free_space_a;

fn main() {
    //get_user_name();
    //cursor();
    get_disk_free_space_a()
}
