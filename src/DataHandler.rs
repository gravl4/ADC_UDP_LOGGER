// mod DataHandler;

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use std::time::{ SystemTime, Duration };
use std::sync::mpsc::{self, RecvTimeoutError};
use std::mem;

use crate::DataSweep:: { DATA_SWEEP_TYPE, DATA_SWEEP_FLUSH_INFO, arr_DataSweepInfo };

// #[path = "Data.rs"]
use crate::Data:: { DATA_ARR, PacketArr_0, PacketArr_1, GetMtxData};

// #[path = "gui.rs"]
use crate::gui:: { ResetDrawDataEx, FlushPoints, pGUIEvntSender };

use crate::Dbg:: { DBG_MSG, SET_START, PRINT_DURATION };

pub fn ToggleMtxArr(PointsToFlush: i32, SweepType: DATA_SWEEP_TYPE) -> &'static mut DATA_ARR
{
  let mData= GetMtxData();
  let mut guard  = mData.lock().unwrap();

  if PointsToFlush != 0 { FlushPoints(PointsToFlush, SweepType); }
  else                  
  if PointsToFlush == 0 {  ResetDrawDataEx(SweepType);  } 

  guard.iArr= guard.iArr ^ 1;
unsafe
{
  if guard.iArr == 1
  {
    guard.PacketArr= &mut PacketArr_1;
    PacketArr_1.cnt_Packets= 0;
    PacketArr_1.cnt_Steps= 0;
    PacketArr_1.cnt_PointsToAcc= 0;
    PacketArr_1.iAvgVal= 0;
    if PointsToFlush == -1 && SweepType >= DATA_SWEEP_TYPE::_1_min
    {
      PacketArr_1.cnt_Steps       = PacketArr_0.cnt_Steps;
      PacketArr_1.cnt_PointsToAcc = PacketArr_0.cnt_PointsToAcc;
      PacketArr_1.iAvgVal         = PacketArr_0.iAvgVal;
    }
    PacketArr_1.pDrawData= &arr_DataSweepInfo[SweepType as usize].Draw;
    return &mut PacketArr_0;
  }
  else
  {
    guard.PacketArr= &mut PacketArr_0;
    PacketArr_0.cnt_Packets= 0;
    PacketArr_0.cnt_Steps= 0;
    PacketArr_0.cnt_PointsToAcc= 0;
    PacketArr_0.iAvgVal= 0;
    if PointsToFlush == -1 && SweepType >= DATA_SWEEP_TYPE::_1_min
    {
      PacketArr_0.cnt_Steps       = PacketArr_1.cnt_Steps;
      PacketArr_0.cnt_PointsToAcc = PacketArr_1.cnt_PointsToAcc;
      PacketArr_0.iAvgVal         = PacketArr_1.iAvgVal;
    }
   PacketArr_0.pDrawData= &arr_DataSweepInfo[SweepType as usize].Draw;
   return &mut PacketArr_1;
  }
}
}

pub fn DataHandler_loop()
{
  let mut SweepType  : DATA_SWEEP_TYPE              = DATA_SWEEP_TYPE::_1_sec; 
  let mut pFlushData : *const DATA_SWEEP_FLUSH_INFO = &arr_DataSweepInfo[DATA_SWEEP_TYPE::_1_sec as usize].Flush;
  let mut cnt_FlushSteps  : i32= 0;
  let (mut sender, receiver) = mpsc::channel();
  unsafe { pGUIEvntSender= &mut sender; }

  let SleepDuration= Duration::from_millis(200);
  let mut now= SystemTime::now();
  let mut duration= Duration::from_millis(0);
  let mut SweepTypeChanged: bool = false;
  loop
  {
    if duration < SleepDuration 
    {
      match receiver.recv_timeout(SleepDuration- duration) 
      {
        Err(RecvTimeoutError::Timeout) => 
        {
          now= SystemTime::now();
        }, // RecvTimeoutError::Timeout
        Err(RecvTimeoutError::Disconnected) => 
        {  },
        Ok(iSweepType) => 
        { // Sweep val
          duration= now.elapsed().unwrap();
          now= SystemTime::now();
          if iSweepType >= DATA_SWEEP_TYPE::_1_sec as i32 && iSweepType < DATA_SWEEP_TYPE::NUM_OF_DATA_SWEEP_TYPES as i32 && SweepType as i32 != iSweepType
          {
            SweepType= unsafe { mem::transmute(iSweepType) };
            pFlushData= &arr_DataSweepInfo[SweepType as usize].Flush;
            cnt_FlushSteps= 0;
            ToggleMtxArr(0, SweepType);
            duration= now.elapsed().unwrap();
          }
          else
          {
            println!("duration: {:?}", duration);
          }
          SweepTypeChanged= true;
        },
      } // match receiver.recv_timeout(duration) 
    } // if duration < SleepDuration 
    else { now= SystemTime::now(); println!("duration >= SleepDuration"); }
    if SweepTypeChanged 
    { 
      SweepTypeChanged= false; 
    }
    else
    {
      cnt_FlushSteps+= 1;
    unsafe
    { 
      if cnt_FlushSteps == (*pFlushData).StepsToAcc
      {
        cnt_FlushSteps= 0;
        let PacketArr= ToggleMtxArr((*pFlushData).PointsToFlush, SweepType);
        if PacketArr.cnt_Packets > 0
        {
          DBG_MSG!("Data.Cnt: {}    ", PacketArr.cnt_Packets);
        }
      } // if cnt_Steps == (*pFlushData).StepsToAcc
      else
      {
        if SweepType >= DATA_SWEEP_TYPE::_10_min
        {
          ToggleMtxArr(-1, SweepType);
        }
      }
    }
      duration= now.elapsed().unwrap();
    }
#[cfg(feature = "DO_TIMING")]
{
    println!(" hd: :    {}   {}   {:?}", duration.as_millis(), duration.as_micros(), duration); 
}
  }
}

