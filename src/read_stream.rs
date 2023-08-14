use byteorder::{ByteOrder, LittleEndian};
use std::str;

pub(crate) struct Stream {
    buffer: Vec<u8>,
}

impl Stream {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self { buffer }
    }

    pub fn debug(&self) {
        log::info!("{:#?}", self.buffer);
    }

    pub fn next_item(&mut self, size: u32) -> Vec<u8> {
        let binding = self.buffer.clone();
        let (current, buf) = binding.split_at(size.try_into().unwrap());
        self.buffer = buf.to_vec();
        current.to_vec()
    }

    pub fn read(&mut self) -> u8 {
        *self.next_item(1).first().unwrap()
    }

    pub fn read_u16(&mut self) -> u16 {
        LittleEndian::read_u16(&self.next_item(2))
    }

    pub fn read_u32(&mut self) -> u32 {
        LittleEndian::read_u32(&self.next_item(4))
    }

    pub fn read_bool(&mut self) -> bool {
        *self.next_item(1).first().unwrap() == 1
    }

    pub fn read_float(&mut self) -> f32 {
        LittleEndian::read_f32(&self.next_item(4))
    }

    pub fn read_str(&mut self) -> String {
        let string_size = self.read_u16();
        if string_size > 0 {
            return match str::from_utf8(&self.next_item(string_size.into())) {
                Ok(v) => v.to_owned(),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
        }
        "".into()
    }
}
