use std::process::Command;
use std::io::{self, Write};
use std::string::String;
use std::fs;
use std::io::stdout;
use std::io::stdin;

fn main() {
    let name = select_name();
    let ram = select_memory();
    let (disk, hd) = select_hd();
    let iso = select_cdrom();

    // Initialize the VM with the selected ISO or disk and RAM.
    println!();
    println!("{} has been selected with {}B of RAM on the disk {}", iso, ram, disk);
    println!("Starting the VM...");
    Command::new("qemu-system-x86_64")
            .args(&["--enable-kvm", "-smp", "4", "-boot", "d", "-name", &name, "-m", &ram, &hd, &disk, "-cdrom", &iso])
            .status()
            .expect("failed to start the VM");
}

fn select_name() -> String {
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    let name = name.trim();

    if name.is_empty() {
        String::new()
    } else {
        String::from(name)
    }
}


fn select_memory() -> String {
    // Create a numbered menu with the available options of RAM:
    let ram_options = vec!["32M", "64M", "128M", "256M", "512M", "1G", "2G", "3G", "4G"];
    println!();
    println!("Select the amount of RAM to be used:");
    for (index, ram) in ram_options.iter().enumerate() {
        println!("{}. {}", index, ram);
    }
    loop {
        print!(": ");
        io::stdout().flush().unwrap();
        let mut reply = String::new();
        match io::stdin().read_line(&mut reply) {
            Ok(_) => {
                let trimmed = reply.trim();
                if let Ok(index) = trimmed.parse::<usize>() {
                    if index < ram_options.len() {
                        // Selected RAM is valid
                        return String::from(ram_options[index]);
                    }
                }
                println!("Invalid option, please try again:");
            }
            Err(_) => {
                println!("Failed to read input, using default RAM (1G)");
                return String::from("1G");
            }
        }
    }
}

fn select_hd() -> (String, String) {
    // Create a numbered menu with the available options of disk:
    let mut disk_options = vec!["N/A".to_string()];
    let mut img_files = fs::read_dir(".").unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().unwrap_or_default() == "img")
        .map(|entry| entry.path().file_name().unwrap().to_string_lossy().into_owned())
        .collect::<Vec<String>>();
    img_files.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    disk_options.append(&mut img_files);

    println!("\nSelect a hard disk to start:");
    for (index, disk) in disk_options.iter().enumerate() {
        println!("{}. {}", index, disk);
    }

    loop {
        print!(": ");
        stdout().flush().unwrap();
        let mut reply = String::new();
        stdin().read_line(&mut reply).unwrap();
        let reply = reply.trim().parse::<usize>();

        if let Ok(reply) = reply {
            if reply >= 1 && reply <= disk_options.len() - 1 {
                // Selected disk is valid
                return (disk_options[reply].clone(), "-hda".to_string());
            } else if reply == 0 {
                // User chose to not select a hard disk
                return ("".to_string(), "".to_string());
            }
        }

        println!("Invalid option, please try again:");
    }
}


fn select_cdrom() -> String {
    // Print a numbered menu with the available options of ISO:
    let iso_options = fs::read_dir(".")?
    .filter_map(|entry| {
        if let Ok(entry) = entry {
            if let Some(extension) = entry.path().extension() {
                if extension == "iso" || extension == "ISO" {
                    return Some(entry.file_name());
                }
            }
        }
        None
    })
    .map(|os_string| os_string.to_string_lossy().to_string())
    .collect::<Vec<_>>();

iso_options.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    println!("\nSelect an ISO to boot:");
    for (index, iso) in iso_options.iter().enumerate() {
        println!("{}. {}", index, iso);
    }

    loop {
        print!(": ");
        io::stdout().flush().unwrap();

        let mut reply = String::new();
        io::stdin().read_line(&mut reply).unwrap();
        let reply = reply.trim().parse::<usize>();

        match reply {
            Ok(reply) if reply <= iso_options.len() => {
                // Selected ISO is valid
                return iso_options[reply].clone();
            }
            _ => println!("Invalid option, please try again:"),
        }
    }
}