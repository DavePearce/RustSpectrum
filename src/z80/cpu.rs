// Main register pairs
const AF: u8 = 0;
const BC: u8 = 2;
const DE: u8 = 4;
const HL: u8 = 6;
// Alternate registers
const AF_: u8 = 8;
const BC_: u8 = 10;
const DE_: u8 = 12;
const HL_: u8 = 14;
// Index Registers
const IX: u8 = 16;
const IY: u8 = 18;
const SP: u8 = 20;
const IR: u8 = 22;
const PC: u8 = 24;
const ST: u8 = 26;

/**
 * A collection of the relevant parts of the Z80 cpu.
 */
struct CPU {
    registers: [u16; 14]
}

/**
 * The RAM trait provides an abstraction of memory accessible to the CPU via the address bus.
 */
trait RAM {
    /**
     * Read a single request over the address bus
     */
    fn read(&self, u16) -> u8;

    /**
     * Write a single request to the address bus
     */
    fn write(&self, u16, u8);
}

fn clock(cpu : &mut CPU, mem : &mut RAM) {
    let opcode = mem.read(cpu.registers[PC as usize]);
}
