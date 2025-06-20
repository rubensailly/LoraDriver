pub enum PacketTypes {
    UKNOWN,
    ACK,
    NACK,
    INFO,
    DATA,
    REQ,
}

pub fn match_packet_types(packet_type: PacketTypes) -> u8 {
    match packet_type {
        PacketTypes::UKNOWN => 0,
        PacketTypes::ACK => 1,
        PacketTypes::NACK => 2,
        PacketTypes::INFO => 3,
        PacketTypes::DATA => 4,
        PacketTypes::REQ => 5,
    }
}

pub union PacketHeader {
    pub whole: [u8; 5],
    pub parts: PacketParts,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PacketParts {
    pub to_address: u8,
    pub from_address: u8,
    pub packet_size: u8,
    pub packet_type: u8,
    pub packet_number: u8,
}

pub struct Packet {
    pub header: PacketHeader,
    pub data: Vec<u8>,
    pub checksum: u8,
}

impl Packet {
    pub fn new() -> Packet {
        Packet {
            header: PacketHeader { whole: [0; 5] },
            data: Vec::new(),
            checksum: 0,
        }
    }

    pub fn print_header(&self) {
        unsafe {
            for byte in &self.header.whole {
                print!("{:#2x} ", byte);
            }
        }
        println!();
    }

    pub fn set_type(&mut self, packet_type: PacketTypes) {
        self.header.parts.packet_type = match_packet_types(packet_type);
    }

    pub fn set_to_address(&mut self, address: u8) {
        self.header.parts.to_address = address;
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
      let mut size: u8 = 0;
      for byte in data {
        self.data.push(byte);
        size += size;
      }
      self.header.parts.packet_size = size;
      self.compute_checksum();
    }

    pub fn compute_checksum(&mut self) -> u8 {
      if self.data.len() > 0 {
        let mut cksum:u8 = 0;
        unsafe {
          for byte in self.header.whole {
            cksum ^= byte;
          }
          for byte in self.data.iter() {
            cksum ^= byte;
          }
          self.checksum = cksum;
          return cksum; 
        }
      }
      return 0;
    }

    pub fn verify_checksum(&mut self) -> bool {
      let old_cksum: u8 = self.checksum;
      return old_cksum == self.compute_checksum();
    }
}
