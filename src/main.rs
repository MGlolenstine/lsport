use lsport::{get_header, get_serial_ports};

fn main() {
    let ports = get_serial_ports();
    println!("{}", get_header());
    for p in ports {
        println!("{}", p);
    }
}
