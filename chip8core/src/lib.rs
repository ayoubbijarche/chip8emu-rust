const RAM_SIZE : usize = 4096;
pub const W : usize = 64;
pub const H : usize = 32;
const REG_NUMS : usize = 16;
const STACK_SIZE : usize = 16;
const START_ADRR : u16 = 0x200;
const NUM_KEYS : usize = 16;
const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
0x20, 0x60, 0x20, 0x20, 0x70, // 1
0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
0x90, 0x90, 0xF0, 0x10, 0x10, // 4
0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
0xF0, 0x10, 0x20, 0x40, 0x40, // 7
0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
0xF0, 0x90, 0xF0, 0x90, 0x90, // A
0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
0xF0, 0x80, 0x80, 0x80, 0xF0, // C
0xE0, 0x90, 0x90, 0x90, 0xE0, // D
0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Emu{
  pc : u16, // program counter to store currently executing addresses
  ram : [u8 ; RAM_SIZE], // ram size
  screen : [bool; W*H], //stores an array of boolean(pixels)
  v_reg : [u8 ; REG_NUMS], // V0 to VF registers
  i_reg : u16, //used to store memory addresses
  sp : u16, // stack pointer 
  stack : [u16 ; STACK_SIZE], //stack memory
  keys : [bool ; NUM_KEYS],
  dt : u8, //delay timer
  st : u8,// sound timer
}

impl Emu{
  pub fn new() -> Self{
    Self{
      pc : START_ADRR,
      ram : [0 ; RAM_SIZE],
      screen : [false ; W*H],
      v_reg : [0 ; REG_NUMS],
      i_reg : 0,
      sp : 0,
      stack : [0 ; STACK_SIZE],
      keys : [false ; NUM_KEYS],
      dt : 0,
      st : 0,
    };
    
    let mut new_emu = Self{
      pc : START_ADRR,
      ram : [0 ; RAM_SIZE],
      screen : [false ; W*H],
      v_reg : [0 ; REG_NUMS],
      i_reg : 0,
      sp : 0,
      stack : [0 ; STACK_SIZE],
      keys : [false ; NUM_KEYS],
      dt : 0,
      st : 0,
    };
    
    new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    
    new_emu
    
  }    
  
  fn push(&mut self , val : u16){
    self.stack[self.sp as usize] = val;
    self.sp += 1;
  }
  
  fn pop(&mut self) -> u16 {
    self.sp -= 1;
    self.stack[self.sp as usize]
  }
  
  pub fn reset(&mut self){
    self.pc = START_ADRR;
    self.ram = [0 ; RAM_SIZE];
    self.screen = [false ; W*H];
    self.v_reg = [0 ; REG_NUMS];
    self.i_reg = 0;
    self.sp = 0;
    self.stack = [0 ; STACK_SIZE];
    self.keys = [false ; NUM_KEYS];
    self.dt = 0;
    self.st = 0;
  }
  
}