pub const NUM_OF_MIN_MAX_VALS: usize = 15;

#[derive(Copy, Clone)]
#[derive(PartialEq, PartialOrd)]
// #[derive(FromPrimitive)]
#[derive(Debug)] 
#[repr(i32)]
pub enum DATA_SWEEP_TYPE
{
//  _10_msec, 
//  _100_msec, 
  _1_sec      = 0, 
  _10_sec        , 
  _1_min         , 
  _10_min        , 
  _1_h           , 
//  _3_h         , 
//  _12_h        ,
  NUM_OF_DATA_SWEEP_TYPES
}


#[derive(Copy, Clone)]
pub struct DATA_SWEEP_FLUSH_INFO
{
  pub DataSweep      : DATA_SWEEP_TYPE,
  pub StepsToAcc     : i32,
  pub PointsToFlush  : i32
}

impl DATA_SWEEP_FLUSH_INFO 
{
  pub const fn init(tag: DATA_SWEEP_TYPE, FlushStepsToAcc: i32, _PointsToFlush: i32) -> Self 
  {
    Self
    { 
      DataSweep       : tag, // DATA_SWEEP_TYPE::_1_sec,
      StepsToAcc      : FlushStepsToAcc,
      PointsToFlush   : _PointsToFlush
    }
  }
}

#[derive(Copy, Clone)]
pub struct DATA_SWEEP_DRAW_INFO
{
  pub DataSweep      : DATA_SWEEP_TYPE,
  pub StepsToAcc     : i32,
  pub PointsToAcc    : i32,
  pub PointsPerStep  : i32
}

impl DATA_SWEEP_DRAW_INFO 
{
  pub const fn init(tag: DATA_SWEEP_TYPE, DrawStepsToAcc: i32, DrawPointsToAcc: i32, PointsPerDrawStep: i32) -> Self 
  {
    Self
    { 
      DataSweep      : tag, // DATA_SWEEP_TYPE::_1_sec,
      StepsToAcc     : DrawStepsToAcc,
      PointsToAcc    : DrawPointsToAcc,
      PointsPerStep  : PointsPerDrawStep
    }
  }
}

#[derive(Copy, Clone)]
pub struct DATA_SWEEP_INFO
{
  pub tag    : DATA_SWEEP_TYPE,
  pub Draw   : DATA_SWEEP_DRAW_INFO,
  pub Flush  : DATA_SWEEP_FLUSH_INFO,
}

impl DATA_SWEEP_INFO 
{
  pub const fn init(tag: DATA_SWEEP_TYPE, FlushStepsToAcc: i32, PointsToFlush: i32, DrawStepsToAcc: i32, PointsToAcc: i32, PointsPerStep: i32) -> Self 
  {
    Self
    { 
      tag     : tag,
      Flush   : DATA_SWEEP_FLUSH_INFO::init(tag, FlushStepsToAcc, PointsToFlush),
      Draw    : DATA_SWEEP_DRAW_INFO::init(tag, DrawStepsToAcc, PointsToAcc, PointsPerStep),
    }
  }
}

pub const arr_DataSweepInfo : 
[DATA_SWEEP_INFO; DATA_SWEEP_TYPE::NUM_OF_DATA_SWEEP_TYPES as usize]= 
[
  DATA_SWEEP_INFO::init(DATA_SWEEP_TYPE::_1_sec,   1,   200,      1,               1,             50),
  DATA_SWEEP_INFO::init(DATA_SWEEP_TYPE::_10_sec,  1,    20,      1,              10,              5), 
  DATA_SWEEP_INFO::init(DATA_SWEEP_TYPE::_1_min,   1,     2,      2,             100,              1), 
  DATA_SWEEP_INFO::init(DATA_SWEEP_TYPE::_10_min,  5,     1,     20,            1000,              1),
  DATA_SWEEP_INFO::init(DATA_SWEEP_TYPE::_1_h,    30,     1,    120,            6000,              1),
/*
  DATA_SWEEP_INFO::init(DATA_SWEEP_TYPE::_3_h,     0,     0,   0,      0,    0),
  DATA_SWEEP_INFO::init(DATA_SWEEP_TYPE::_12_h,    0,     0,   0,      0,    0),
*/
];

