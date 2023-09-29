#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// #![feature(static_mutex)]

use std::{thread};
use std::sync::Arc;

use std::net::{UdpSocket};
use std::net::{IpAddr, SocketAddr};

mod CnfgReader;
mod DataHandler;
mod Data;
mod DataSweep;
mod DrawParams;
mod gui;
mod guiCntrls;
mod guiVals;
mod udp;
mod Dbg;


// #[path = "Data.rs"]
// use Data:: { NUM_OF_A_CHS, NUM_OF_D_CHS, DATA_PACKET, PacketsFreq }; 

use udp:: { IP, Port }; 
use CnfgReader:: { ReadCnfg, Pause };
use DataHandler:: { DataHandler_loop };
use gui:: { InitGUI, wnd_evnt_loop };

fn main() 
{
  println!("Data visualizator started");
  println!("Reading configuration...");  
  if !ReadCnfg() 
  { 
    println!("Correct configuration options and try again");
    Pause();
//    return; 
  }

  println!("Initializing GUI...");  

  InitGUI();

  thread::spawn(move ||
  {
    wnd_evnt_loop();
  });
  
  thread::spawn(move ||
  {
    DataHandler_loop();
  });
  
  let RemAddr= SocketAddr::new(unsafe{IP.parse::<IpAddr>()}.unwrap(), unsafe{Port as u16});

  let udpSock= UdpSocket::bind(unsafe{format!("0.0.0.0:{}", Port)}).expect("couldn't bind to local address");
  let RecvSock = Arc::new(udpSock);
  let SendSock = RecvSock.clone();
  match SendSock.send_to(&[0x35; 1], &RemAddr)
  {
    Err(e) =>   { println!("Err: {}", e); }
    Ok(_len) =>  { /* println!("Sent: {:?} bytes", _len); */ }
  }

/*
  thread::spawn( move|| 
  {
     udp::Send_loop(&SendSock, &RemAddr);
  });
*/
  udp::ReadData_loop(&RecvSock, &RemAddr);
}