CONST AF: u8 = 0;
CONST BC: u8 = 2;
CONST HL: u8 = 4;
CONST PC: u8 = 6;
CONST SP: u8 = 8;
CONST IR: u8 = 5;
CONST DE: u8 = 13;

CONST BC_: u8 = 15;
CONST DE_: u8 = 17;
CONST HL_: u8 = 19;
CONST AF_: u8 = 21;

struct RegisterFile {
    registers: [u16; 8]
};

struct CPU {

};
