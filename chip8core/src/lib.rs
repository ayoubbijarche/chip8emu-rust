/*
3XNN Skip if VX == 0xNN
4XNN Skip if VX != 0xNN
5XY0 Skip if VX == VY
6XNN VX = 0xNN
7XNN VX += 0xNN Doesn’t affect carry flag
8XY0 VX = VY
8XY1 VX |= VY
8XY2 VX &= VY
8XY3 VX ˆ= VY
8XY4 VX += VY Sets VF if carry
8XY5 VX -= VY Clears VF if borrow
8XY6 VX »= 1 Store dropped bit in VF
8XY7 VX = VY - VX Clears VF if borrow
8XYE VX «= 1 Store dropped bit in VF
9XY0 Skip if VX != VY
ANNN I = 0xNNN
BNNN Jump to V0 + 0xNNN
CXNN VX = rand() & 0xNN
DXYN Draw sprite at (VX, VY) Sprite is 0xN pixels tall, on/off based on value in I, VF set if
any pixels flipped
EX9E Skip if key index in VX is pressed
EXA1 Skip if key index in VX isn’t pressed
FX07 VX = Delay Timer
FX0A Waits for key press, stores index in VX Blocking operation
FX15 Delay Timer = VX
FX18 Sound Timer = VX
FX1E I += VX
FX29 Set I to address of font character in VX*
FX33 Stores BCD encoding of VX into I* 
FX55 Stores V0 thru VX into RAM address starting at I Inclusive range
FX65 Fills V0 thru VX with RAM values starting
at address in I
*/
use rand::random;

const RAM_SIZE : usize = 4096;
pub const W : usize = 64;
pub const H : usize = 32;
const REG_NUMS : usize = 16;
const STACK_SIZE : usize = 16;
const START_ADDR : u16 = 0x200;
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
    let mut new_emu = Self{
      pc : START_ADDR,
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
    self.pc = START_ADDR;
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
  
  pub fn tick(&mut self){
    //fetch instructions
    let op = self.fetch();
    //decode & execute
    self.execute(op);
  }
  
  fn fetch(&mut self)->u16{
    let high = self.ram[self.pc as usize] as u16;
    let low = self.ram[(self.pc + 1) as usize] as u16;
    let op = (high << 8 ) | low;
    self.pc += 2;
    op
  }


  pub fn tick_time(&mut self){
    if self.dt > 0 {
      self.dt -= 1;
    }
    if self.st > 0{
      if self.st == 1 {
        //too lazy to implement sound
      }
      self.st -= 1;
    }
  }
  


  
  pub fn execute(&mut self, op : u16){
    let digit1 = (op & 0xF000) >> 12;
    let digit2 = (op & 0x0F00) >> 8;
    let digit3 = (op & 0x00F0) >> 4;
    let digit4 = op & 0x000F;
    match(digit1 , digit2 , digit3 , digit4){
      (0, 0, 0, 0) => return,
      // clear screen using 00E0 opcode
      (0 , 0 , 0xE , 0) => {
        self.screen = [false; W * H];
      },
      
      // return subroutine using 00EE opcode
      (0 ,0 , 0xE , 0xE) => {
        let ret_address = self.pop();
        self.pc = ret_address;
      }
      
      // jump instruction to the 0xNNN address
      (1 , _ , _ , _) => {
        let nnn = op & 0xFFF;
        self.pc = nnn;
      }
      
      // 2NNN entering subrouting  at 0xNNN address, adding pc to current stack
      (2 , _ , _ , _) => {
        let nnn = op & 0xFFF;
        self.push(self.pc);
        self.pc = nnn;
      }
      
      //conditions and jumping to next instruction
      (3, _ , _ ,_) => {
        let x = digit2 as usize;
        let nn = (op & 0xFF) as u8;
        if self.v_reg[x] == nn {
          self.pc += 2;
        }
      }
      
      (4 , _ , _ , _) => {
        let x = digit2 as usize;
        let nn = (op & 0xFF) as u8;
        if self.v_reg[x] != nn {
          self.pc += 2;
        }
      }
    
      
      (5 , _ , _ , 0) => {
        let x = digit2 as usize;
        let y = digit3 as usize;
        if self.v_reg[x] == self.v_reg[y]{
          self.pc += 2;
        }
      }
      
      (6 , _ , _ , _) => {
        let x = digit2 as usize;
        let nn = (op & 0xFF) as u8;
        self.v_reg[x] = nn;
      }
      
      (7 , _ , _ , _) => {
        let x = digit2 as usize;
        let nn = (op & 0xFF) as u8;
        self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
      }
      
      (8 , _ , _ , 0) =>{
        let x = digit2 as usize;
        let y = digit3 as usize;
        self.v_reg[x] = self.v_reg[y];
      }
      
      (8 , _ , _ , 1) => {
        let x = digit2 as usize;
        let y = digit3 as usize;
        self.v_reg[x] |= self.v_reg[y];
      }
      
      (8 , _ , _ , 2) => {
        let x = digit2 as usize;
        let y = digit3 as usize;
        self.v_reg[x] &= self.v_reg[y];
      }
      
      (8 , _ , _ , 3) => {
        let x = digit2 as usize;
        let y = digit3 as usize;
        self.v_reg[x] ^= self.v_reg[y];
      }
      
      (8 , _ , _ , 4) => {
        let x = digit2 as usize;
        let y = digit3 as usize;
        let (new_vx , carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
        let new_vf = if carry{1}else{0};
        self.v_reg[x] = new_vx;
        self.v_reg[0xF] = new_vf;
      }
      
      (8 , _ , _ , 5) => {
        let x = digit2 as usize;
        let y = digit3 as usize;
        let (new_vx , borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
        let new_vf = if borrow{0}else{1};
        self.v_reg[x] = new_vx;
        self.v_reg[0xF] = new_vf;
      }
      
      (8 , _ , _ , 6) => {
        let x = digit2 as usize;
        let lsb = self.v_reg[x] & 1;
        self.v_reg[x] >>= 1;
        self.v_reg[0xF] = lsb;
        
      }
      
      (8 , _ , _ , 7) => {
        let x = digit2 as usize;
        let y = digit3 as usize;
        let (new_vx , borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
        let new_vf = if borrow{0}else{1};
        self.v_reg[x] = new_vx;
        self.v_reg[0xF] = new_vf;
      }
      
      (8 , _ , _ , 0xE) => {
        let x = digit2 as usize;
        let msb = (self.v_reg[x] >> 7 ) & 1;
        self.v_reg[x] <<= 1;
        self.v_reg[0xF] = msb;
      }
      
      (9 , _ , _ , 0) => {
        let x = digit2 as usize;
        let y = digit3 as usize;
        if self.v_reg[x] != self.v_reg[y]{
          self.pc += 2;
        }
      }
      
      (0xA , _ , _ , _) => {
        let nnn = op & 0xFFF;
        self.i_reg = nnn;
      }
      
      (0xB , _ , _ , _) => {
        let nnn = op & 0xFFF;
        self.pc = (self.v_reg[0] as u16) + nnn;
      }
      
      (0xC , _ , _, _) => {
        let x = digit2 as usize;
        let nn = (op & 0xFF) as u8;
        let rng: u8 = random();
        self.v_reg[x] = rng & nn;
      }
      
      (0xD, _, _, _) => {
          let x_coord = self.v_reg[digit2 as usize] as u16;
          let y_coord = self.v_reg[digit3 as usize] as u16;
          let num_rows = digit4;
          let mut flipped = false;
      
          for y_line in 0..num_rows {
              let addr = self.i_reg + y_line as u16;
              let pixels = self.ram[addr as usize];
      
              // Iterate over each column in the row
              for x_line in 0..8 {
                  if (pixels & (0b1000_0000 >> x_line)) != 0 {
                      let x = (x_coord + x_line) as usize % W;
                      let y = (y_coord + y_line) as usize % H;
                      let idx = x + W * y;
                      flipped |= self.screen[idx];
                      self.screen[idx] ^= true;
                  }
              }
          }
      
          // Set VF if any pixels were flipped
          self.v_reg[0xF] = if flipped { 1 } else { 0 };
      }
      
      
      (0xE , _ , 9 , 0xE) => {
        let x = digit2 as usize;
        let vx = self.v_reg[x];
        let key = self.keys[vx as usize];
        
        if key {
          self.pc += 2;
        }
      }
      
      (0xE , _ , 0xA , 1) => {
        let x = digit2 as usize;
        let vx = self.v_reg[x];
        let key = self.keys[vx as usize];
        if !key{
          self.pc += 2;
        }
      }
      
      (0xF , _ , 0 , 7) => {
        let x = digit2 as usize;
        self.v_reg[x] = self.dt;
      }
      
      (0xF , _ , 0 , 0xA) => {
        let x = digit2 as usize;
        let mut pressed = false;
        
        for i in 0..self.keys.len(){
          if self.keys[i]{
            self.v_reg[x] = i as u8;
            pressed = true;
            break;
          }   
        }
        
        if !pressed {
          self.pc += 2;
        }
      }
      
      (0xF , _ , 1 , 5) => {
        let x = digit2 as usize;
        self.dt = self.v_reg[x];
      }
      
      (0xF , _ , 1 , 8) => {
        let x = digit2 as usize;
        self.st = self.v_reg[x];
      }
      
      (0xF , _  , 1 , 0xE) => {
        let x = digit2 as usize;
        let vx = self.v_reg[x] as u16;
        self.i_reg = self.i_reg.wrapping_add(vx);
      }
      
      (0xF , _ , 2 , 9) => {
        let x = digit2 as usize;
        let c = self.v_reg[x] as u16;
        self.i_reg = c * 5;
      }
      
      (0xF , _ , 3 , 3) => {
        let x = digit2 as usize;
        let vx = self.v_reg[x] as f32;
        let hundreds = (vx / 100.0).floor() as u8;
        let tens = ((vx / 10.0) % 10.0).floor() as u8;
        let ones = (vx % 10.0) as u8;
        self.ram[self.i_reg as usize] = hundreds;
        self.ram[(self.i_reg + 1) as usize] = tens;
        self.ram[(self.i_reg + 2) as usize] = ones;
      }

      (0xF, _ , 5 , 5) => {
        let x = digit2 as usize;
        let i = self.i_reg as usize;
        for idx in 0..=x{
            self.ram[i + idx] = self.v_reg[idx];
        }
      }

      (0xF , _ , 6 , 5) => {
        let x = digit2 as usize;
        let i = self.i_reg as usize;
        for idx in 0..=x{
            self.v_reg[idx] = self.ram[i + idx]; 
        }
      }
      
      (_,_,_,_) => unimplemented!("opcode {}" , op),

      
    }
  }


  pub fn getscreen(&self) -> &[bool] {
    &self.screen
  }

  pub fn keypress(&mut self , idx : usize , pressed : bool){
    self.keys[idx] = pressed;
  }

  pub fn load(&mut self , data : &[u8]){
    let start = START_ADDR as usize;
    let end = (START_ADDR as usize) + data.len();
    self.ram[start..end].copy_from_slice(data);
  }
}

