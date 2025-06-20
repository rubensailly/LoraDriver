mod lora;
use lora::packet::*;

fn main() {
    let mut pck = Packet::new();
    
    unsafe {
        pck.header.parts.from_address = 254;
        pck.header.parts.packet_number = 10;
        let data: Vec<u8> = Vec::from([12, 13, 25, 89, 123]);

        pck.set_to_address(12);
        pck.set_type(PacketTypes::DATA);
        pck.set_data(data);
        pck.print_header();

        let correct_cksum: bool = pck.verify_checksum();
        println!("{}", correct_cksum);
        println!("Hello world");   
    }
}
