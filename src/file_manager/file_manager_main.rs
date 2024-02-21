use std::io;
use std::io::Write;

mod fns;

use fns::create_dir;
use fns::get_disk_space;
use fns::get_free_space;
use fns::options;
fn main() {
    let mut input = String::new();

    loop {
        options();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<u8>() {
            Ok(num) => match num {
                1 => {
                    let disk_space: u32 = get_disk_space();
                    println!("Disk space: {} GB", disk_space);
                }
                2 => {
                    let free_space: f32 = get_free_space();
                    println!("Free Space: {} GB", free_space)
                }
                3 => {
                    let mut path = String::new();
                    let mut name = String::new();

                    print!("Dictionary path: ");
                    std::io::stdout().flush().unwrap();
                    io::stdin()
                        .read_line(&mut path)
                        .expect("Failed to read line");

                    print!("Dictionary name: ");
                    std::io::stdout().flush().unwrap();
                    io::stdin()
                        .read_line(&mut name)
                        .expect("Failed to read line");

                    path = String::from(path.trim());
                    name = String::from(name.trim());

                    if &path[path.len() - 2..] != "\\" {
                        path.push_str("\\");
                    }

                    create_dir(&path, &name)
                }
                4 => break,
                _ => println!("Invalid input. Please enter 1, 2, 3, or 4."),
            },
            Err(_) => println!("Invalid input. Please enter a number."),
        }

        // Clear the input buffer for the next iteration
        input.clear();
        println!()
    }
}
