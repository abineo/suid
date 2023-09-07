use suid::Suid;

fn main() {
    println!("{}", u32::suid());
    println!("{}", u64::suid());
    println!("{}", u128::suid());
    println!();

    println!("{:032b}", u32::suid());
    println!("{:064b}", u64::suid());
    println!("{:0128b}", u128::suid());
    println!();

    println!("{:0b}", u32::suid());
    println!("{:0b}", u64::suid());
    println!("{:0b}", u128::suid());
    println!();

    println!("{:x}", u32::suid());
    println!("{:x}", u64::suid());
    println!("{:x}", u128::suid());
    println!();

    println!("{:?}", u32::suid().to_be_bytes());
    println!("{:?}", u64::suid().to_be_bytes());
    println!("{:?}", u128::suid().to_be_bytes());
}
