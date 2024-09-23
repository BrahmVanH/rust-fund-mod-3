use std::fmt::format;

#[derive(Debug)]

// todo: define enum file size {bytes..gb}(u64)
enum FileSize {
    Bytes(u64),
    KiloBytes(u64),
    MegaBytes(u64),
    GigaBytes(u64),
    TeraBytes(u64),
    AlotaBytes(u64),
}

const KILO: f64 = 1024.0;
const MEGA: f64 = KILO * 1024.0;
const GIGA: f64 = MEGA * 1024.0;
const TERA: f64 = GIGA * 1024.0;

// todo: function calculate file size, takes in size in bytes and returns a file size enum
fn enumerate_file_size(bytes: u64) -> FileSize {
    match bytes {
        0..=999 => FileSize::Bytes(bytes),
        1024..=1_048_575 => FileSize::KiloBytes(bytes),
        1_048_576..=1_073_741_823 => FileSize::MegaBytes(bytes),
        1_073_741_824..=1_099_511_627_775 => FileSize::GigaBytes(bytes),
        1_099_511_627_776..=1_125_899_906_842_623 => FileSize::TeraBytes(bytes),
        _ => FileSize::AlotaBytes(bytes),
    }
}

fn convert_largest_possible_unit(bytes: f64) -> String {
    if bytes >= TERA {
        format!("{:.2} TiB", bytes / TERA)
    } else if bytes >= GIGA {
        format!("{:.2} GiB", bytes / GIGA)
    } else if bytes >= MEGA {
        format!("{:.2} MiB", bytes / MEGA)
    } else if bytes >= KILO {
        format!("{:.2} KiB", bytes / KILO)
    } else {
        format!("{:.2} bytes", bytes)
    }
}

// todo: function format file size, takes in enum and returns the formatted file size

fn format_file_size(size: FileSize) -> String {
    match size {
        FileSize::Bytes(b) => format!("{:} B", b),
        FileSize::KiloBytes(b) => format!("{:.2} KiB", (b as f64) / 1_024.0),
        FileSize::MegaBytes(b) => format!("{:.2} MiB", (b as f64) / 1_048_576.0),
        FileSize::GigaBytes(b) => format!("{:.2} GiB", (b as f64) / 1_073_741_824.0),
        FileSize::TeraBytes(b) => format!("{:.2} TiB", (b as f64) / 1_099_511_627_776.0),
        FileSize::AlotaBytes(b) => format!("Thasss alotabytes {:}", b),
    }
}

// todo: call functions in main

fn main() {
    let bytes = 6933214423188;
    let file_size = enumerate_file_size(bytes);
    let formatted_size = format_file_size(file_size);
    println!("{:?}", formatted_size);

    let test_sizes: Vec<i64> = vec![250, 2500, 2500000, 2500000000, 2500000000000, 6933214423188];

    for &size in &test_sizes {
        println!("{} bytes = {}", size, convert_largest_possible_unit(size as f64));
    }
}
