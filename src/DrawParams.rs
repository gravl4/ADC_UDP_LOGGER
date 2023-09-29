// mod DrawParams;

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_parens)]

use std::os::raw::c_ulong;

// #[path = "Data.rs"]
use crate::Data:: {  NUM_OF_A_CHS, NUM_OF_D_CHS };

pub const ADC_max: u32= 0x0FFF;

pub struct DSPL_OPT
{
  pub name     : &'static str,  
  pub name_len : i32,  
  pub color    : c_ulong, 
}

impl DSPL_OPT 
{
  pub const fn init() -> Self 
  {
    Self
    { 
      name: "",
      name_len: 0,
      color: 0,
    }
  }
}

pub struct A_CH_OPT
{
 pub DsplOpt    : DSPL_OPT,
 pub k          : f32,
 pub b          : i32,
 pub c          : i32,
 pub min        : i32,
 pub max        : i32,
 pub visible    : i32,
 pub show_scale : i32,
}

impl A_CH_OPT 
{
  pub const fn init() -> Self 
  {
    Self
    {
      DsplOpt     : DSPL_OPT::init(),
      k           : 1.,
      b           : -2047,
      c           : 0,
      min         : -2047, 
      max         : 2047, 
      visible     : 1,
      show_scale  : 0,
   }
  }
}

pub struct D_CH_OPT
{
  pub DsplOpt: DSPL_OPT,
  pub invert: i32
}

impl D_CH_OPT 
{
  pub const fn init() -> Self 
  {
    Self
    {
      DsplOpt: DSPL_OPT::init(),
      invert: 0
    }
  }
}

pub struct DRAW_PARAMS
{
  pub name_xx    : i32,  
  pub txt_y      : i32,
  pub y_prev     : i32,
  pub y0         : i32,
  pub Scale      : f32,
}

pub struct A_CH_DRAW_PARAMS
{
  pub Ch: A_CH_OPT,
  pub Draw: DRAW_PARAMS
}

pub struct D_CH_DRAW_PARAMS
{
  pub Ch:  D_CH_OPT,
  pub Draw: DRAW_PARAMS
}

impl DRAW_PARAMS
{
  pub const fn init() -> Self 
  {
    Self
    { 
      name_xx    : 0,
      txt_y      : 0,
      y_prev     : 0,
      y0         : 0,
      Scale      : 0.,
    }
  }
}

impl A_CH_DRAW_PARAMS
{
  pub const fn init() -> Self 
  {
    Self
    { 
      Ch: A_CH_OPT::init(),
      Draw: DRAW_PARAMS::init(),
    }
  }
}

impl D_CH_DRAW_PARAMS
{
  pub const fn init() -> Self 
  {
    Self
    { 
      Ch: D_CH_OPT::init(),
      Draw: DRAW_PARAMS::init()
    }
  }
}

pub static mut arr_A_Ch_Params: [A_CH_DRAW_PARAMS; NUM_OF_A_CHS] = 
[
  A_CH_DRAW_PARAMS::init(), A_CH_DRAW_PARAMS::init(), 
  A_CH_DRAW_PARAMS::init(), A_CH_DRAW_PARAMS::init(), 
  A_CH_DRAW_PARAMS::init(), A_CH_DRAW_PARAMS::init(), 
  A_CH_DRAW_PARAMS::init(), A_CH_DRAW_PARAMS::init()
];

pub static mut arr_D_Ch_Params: [D_CH_DRAW_PARAMS; NUM_OF_D_CHS] = 
[
  D_CH_DRAW_PARAMS::init(),  D_CH_DRAW_PARAMS::init(), 
  D_CH_DRAW_PARAMS::init(),  D_CH_DRAW_PARAMS::init(), 
  D_CH_DRAW_PARAMS::init(),  D_CH_DRAW_PARAMS::init(), 
  D_CH_DRAW_PARAMS::init(),  D_CH_DRAW_PARAMS::init(),
  D_CH_DRAW_PARAMS::init(),  D_CH_DRAW_PARAMS::init(), 
  D_CH_DRAW_PARAMS::init(),  D_CH_DRAW_PARAMS::init(), 
  D_CH_DRAW_PARAMS::init(),  D_CH_DRAW_PARAMS::init(), 
  D_CH_DRAW_PARAMS::init(),  D_CH_DRAW_PARAMS::init()
];

