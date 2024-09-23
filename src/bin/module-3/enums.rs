// Create enum disk type, has an SSD options and HDD option
enum DiskType {
  SSD,
  HDD
}

// create enum disk size, kb, mb, gb all u32


#[derive(Debug)]
enum DiskSize {
  KB(u32),
  MB(u32),
  GB(u32)
}

fn main() {
  // declare disk_type variable as ssd from disk type
  let disk_type = DiskType::SSD;

  // check disk_type value and compare to two disk type enum options
  match disk_type {
    DiskType::HDD => println!("HDD!"),
    DiskType::SSD => println!("SSD!")
  }
  // declare disk_size var as disk size enum instance
  let disk_size = DiskSize::KB(24);
  // print disk size
  println!("Disk Size: {:?} ", disk_size);
}
