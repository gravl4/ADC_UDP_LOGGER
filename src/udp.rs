#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use std::io::{/*prelude::*, BufReader, */ErrorKind};
use std::mem;

use std::net::{ UdpSocket };
use std::net::{ SocketAddr };

use crate::Data:: { NUM_OF_A_CHS, NUM_OF_D_CHS, GetMtxData, DATA_ARR };
use crate::DataSweep:: { DATA_SWEEP_TYPE, arr_DataSweepInfo };
use crate::DrawParams:: { arr_A_Ch_Params, arr_D_Ch_Params };
use crate::gui;

pub static mut IP:   &'static str = "255.255.255.255";
pub static mut Port: i32= 0;

pub struct UDP_PACKET
{
 pub Tag: u16,
 pub Id:  u16,
 pub A: [u16; NUM_OF_A_CHS],
 pub D:   u16,
}

pub const UDP_PACKET_LEN    : usize = mem::size_of::<UDP_PACKET>(); 
pub const UDP_PACKET_EX_LEN : usize = UDP_PACKET_LEN* 50; 

fn AddData(Buff: &mut [u8; UDP_PACKET_EX_LEN], NumOfPackets: usize)
{
  let mut iData= 0;
  let mData= GetMtxData();
  match mData.lock()
  {
    Err(e) => { println!("Err: {}", e.to_string()); },
    Ok(mut Data) =>
    {
      let mut pData= unsafe { &mut *Data.PacketArr  as &mut DATA_ARR };
      let pDrawData= pData.pDrawData;
      for _j in 0.. NumOfPackets
      {
        let ptr: &mut [u8] = &mut Buff[iData..iData+ UDP_PACKET_LEN];
        let pPacket: &mut UDP_PACKET= unsafe { &mut *(ptr as *mut [u8] as *mut UDP_PACKET) }; 
        iData= iData+ UDP_PACKET_LEN;
        pData.Packets[pData.cnt_Packets].Id=  pPacket.Id;

        for i in 0.. NUM_OF_A_CHS
        {
          let Val= pPacket.A[i].swap_bytes();
          let fVal: f32= unsafe { ((Val as i32+ arr_A_Ch_Params[i].Ch.b) as f32)* arr_A_Ch_Params[i].Ch.k+ arr_A_Ch_Params[i].Ch.c as f32 } ;
          pData.Packets[pData.cnt_Packets].A_Chs[i]= Val;
          if (*pDrawData).DataSweep == DATA_SWEEP_TYPE::_1_sec {continue; }
          if pData.cnt_PointsToAcc == 0 
          {
            pData.A_Chs_min[pData.iAvgVal][i]= Val; 
            pData.A_Chs_max[pData.iAvgVal][i]= Val;
          }
          else
          {
            if pData.A_Chs_min[pData.iAvgVal][i] > Val { pData.A_Chs_min[pData.iAvgVal][i]= Val; }  
            if pData.A_Chs_max[pData.iAvgVal][i] < Val { pData.A_Chs_max[pData.iAvgVal][i]= Val; }
          }
        }
        let mut BitMask: u16 = 1;
        for i in 0.. NUM_OF_D_CHS
        {
          let Val: u16= (pPacket.D & BitMask) >> i ; // ^ (arr_D_Ch_Params[i].Ch.invert as u16);
          BitMask= BitMask << 1;
          pData.Packets[pData.cnt_Packets].D_Chs[i]= Val as u8;
          if (*pDrawData).DataSweep == DATA_SWEEP_TYPE::_1_sec {continue; }
          if pData.cnt_PointsToAcc == 0 { pData.D_Chs_avg[pData.iAvgVal][i]= Val as u8; }
          else 
          if pData.Packets[pData.iAvgVal].D_Chs[i] == Val as u8 { pData.D_Chs_avg[pData.iAvgVal][i]= Val as u8; }
          else { pData.D_Chs_avg[pData.iAvgVal][i]= 2; }
        }
        pData.cnt_Packets+= 1;
        pData.cnt_PointsToAcc+= 1;
        if pData.cnt_PointsToAcc == (*pDrawData).PointsToAcc 
        { 
          pData.iAvgVal+= 1; 
          pData.cnt_PointsToAcc= 0; 
        }
      } // for j in 0.. NumOfPackets
      pData.cnt_Steps+= 1;
   
      if (*pDrawData).DataSweep == DATA_SWEEP_TYPE::_1_sec
      { gui::AddPoints(&pData.Packets[pData.cnt_Packets- NumOfPackets .. pData.cnt_Packets], NumOfPackets, (*pDrawData).PointsPerStep); }
      else
      {
        if (*pDrawData).DataSweep == DATA_SWEEP_TYPE::_10_sec
        {
          if pData.cnt_Steps >= pDrawData.StepsToAcc
          {
            gui::AddMinMaxPoints(&pData.D_Chs_avg, 
                                 &pData.A_Chs_min, 
                                 &pData.A_Chs_max, 
                                 pData.iAvgVal, (*pDrawData).PointsPerStep);
            pData.cnt_Steps= 0;
            pData.iAvgVal= 0;
          }
        }
        else
        {

          if pData.cnt_Steps == 1
          {
            gui::AddMinMaxPoints(&pData.D_Chs_avg, 
                                 &pData.A_Chs_min, 
                                 &pData.A_Chs_max, 
                                 1, 1);
          }
          else
          {
            gui::AddMinMaxPoints(&pData.D_Chs_avg, 
                                 &pData.A_Chs_min, 
                                 &pData.A_Chs_max,
                                 1, 0); 
            if pData.cnt_Steps >= pDrawData.StepsToAcc
            {
              pData.cnt_Steps= 0;
            }    
            println!("1 0");
          }
          pData.iAvgVal= 0;
        }
      }

    } // Ok(mut Data)
  } // match mData.lock()
}

pub fn ReadData_loop( udpSock: &UdpSocket, _RemAddt: &SocketAddr)  
{
  let  mut udpDataBuff       : [u8; UDP_PACKET_EX_LEN] = [0; UDP_PACKET_EX_LEN];
  let mut NumOfPackets: usize= 0;
  println!("reading by {} bytes", UDP_PACKET_LEN);

  loop
  {
    match udpSock.recv_from( &mut udpDataBuff)
    {
      Err(e) => 
      { 
        if e.raw_os_error() == Some(10040)         {
          println!("Too small buff: {}", e.to_string()); 
        }
        else
        if e.kind() == ErrorKind::OutOfMemory
        {
          println!("OutOfMemory: {}", e.to_string()); 
        }
        else
        if e.kind() == ErrorKind::UnexpectedEof
        { 
          println!("UnexpectedEof"); 
        }
        else
        { 
          println!("Err {}: {}", e.kind(), e.to_string()); 
          continue;
        }
      },
      Ok((BytesRead, _FromAddr)) => 
//      Ok(BytesRead) => 
      { 
        NumOfPackets= BytesRead / UDP_PACKET_LEN;
        if BytesRead < UDP_PACKET_LEN
        {
          println!("Err: {} bytes", BytesRead); 
          continue;
        }
      }
    }
    AddData(&mut udpDataBuff, NumOfPackets);
  }
}
/*
pub fn Send_loop(SendSock: &UdpSocket, pRemAddr: &SocketAddr) 
{
  let interval = Duration::from_millis(15500);
  loop 
  {
//      println!("Sending {}... ", ConnectStr); 
    match SendSock.send_to(&[0x35; 1], pRemAddr)
    {
      Err(e) =>   { println!("Err: {}", e); },
      Ok(_len) =>  { println!("Sent: {:?} bytes", len);  }
    }
    thread::sleep(interval);
  }
}
*/