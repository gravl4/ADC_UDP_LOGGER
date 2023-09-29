// mod Data

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_parens)]

// use std::sync::Arc;
use std::ptr;
use std::sync::Mutex;
use std::sync::Once;
use std::mem;


use crate::DataSweep:: { NUM_OF_MIN_MAX_VALS, DATA_SWEEP_TYPE, DATA_SWEEP_DRAW_INFO, arr_DataSweepInfo };

pub const MAX_FREQ         : usize = 1000;

pub static mut PacketsFreq : i32   = MAX_FREQ as i32;

pub const NUM_OF_A_CHS: usize = 8;
pub const NUM_OF_D_CHS: usize = 16;

#[derive(Copy, Clone)]
pub struct DATA_PACKET
{
  pub Id        : u16,
  pub A_Chs     : [u16; NUM_OF_A_CHS],
  pub D_Chs     : [u8; NUM_OF_D_CHS],
}

impl DATA_PACKET 
{
  pub const fn init() -> Self 
  {
    Self
    { 
      Id        : 0,
      A_Chs     : [0;      NUM_OF_A_CHS],
      D_Chs     : [0;      NUM_OF_D_CHS],
    }
  }
}

const DATA_PACKET_LEN:   usize= mem::size_of::<DATA_PACKET>(); 

pub const NUM_OF_DATA_PACKETS: usize= DATA_PACKET_LEN* MAX_FREQ* 2; // 2 sec

#[derive(Copy, Clone)]
pub struct DATA_ARR
{
  pub Packets         : [DATA_PACKET; NUM_OF_DATA_PACKETS], 
  pub D_Chs_avg       : [[u8; NUM_OF_D_CHS]; NUM_OF_MIN_MAX_VALS], 
  pub A_Chs_min       : [[u16; NUM_OF_A_CHS]; NUM_OF_MIN_MAX_VALS], 
  pub A_Chs_max       : [[u16; NUM_OF_A_CHS]; NUM_OF_MIN_MAX_VALS], 
  pub cnt_Packets     : usize,
  pub cnt_Steps       : i32,
  pub cnt_PointsToAcc : i32,
  pub iAvgVal         : usize,
  pub pDrawData       : &'static DATA_SWEEP_DRAW_INFO,
}

impl DATA_ARR 
{
  pub const fn init() -> Self 
  {
    Self
    {
      Packets         : [DATA_PACKET::init(); NUM_OF_DATA_PACKETS],
      D_Chs_avg       : [[0;      NUM_OF_D_CHS]; NUM_OF_MIN_MAX_VALS], 
      A_Chs_min       : [[0xFFFF; NUM_OF_A_CHS]; NUM_OF_MIN_MAX_VALS],
      A_Chs_max       : [[0;      NUM_OF_A_CHS]; NUM_OF_MIN_MAX_VALS],
      cnt_Packets     : 0,
      cnt_Steps       : 0,
      cnt_PointsToAcc : 0,
      iAvgVal         : 0,
      pDrawData       : &arr_DataSweepInfo[DATA_SWEEP_TYPE::_1_sec as usize].Draw,
    }
  }
}

//#[derive(Copy, Clone)]
pub struct MTX_DATA
{
  pub PacketArr: *mut DATA_ARR, // &'static mut DATA_ARR,
  pub iArr: i32,
}

impl MTX_DATA 
{
  pub const fn init() -> Self 
  {
    Self
    {
      PacketArr: ptr::null_mut(),  // &const  PacketArr_0, // DATA_ARR::init(),
      iArr: 0,
    }
  }
}

pub static mut PacketArr_0: DATA_ARR = DATA_ARR::init();
pub static mut PacketArr_1: DATA_ARR = DATA_ARR::init();

pub fn GetMtxData() -> &'static Mutex<MTX_DATA>  
{
  static mut mData: mem::MaybeUninit<Mutex<MTX_DATA>> = mem::MaybeUninit::uninit();
  static ONCE: Once = Once::new();

  unsafe 
  {
    ONCE.call_once(|| 
    {
      let singleton = Mutex::new(MTX_DATA::init());
      mData.write(singleton);
     let mut guard= mData.assume_init_ref().lock().unwrap();
     guard.iArr= 0;
     PacketArr_0.pDrawData= &arr_DataSweepInfo[DATA_SWEEP_TYPE::_1_sec as usize].Draw;
     PacketArr_1.pDrawData= &arr_DataSweepInfo[DATA_SWEEP_TYPE::_1_sec as usize].Draw;
     guard.PacketArr= &mut PacketArr_0;
    }
    );
    mData.assume_init_ref()
  }
}
