// Implements necessary memory layout and operations

pub struct Memory {
    pages: [u8; 49152]
}

pub fn create() -> Memory {
    return Memory { pages: [0;49152] };
}

pub fn read(addr: u16, mem : &Memory) -> u8 {
    return (*mem).pages[addr as usize];
}
