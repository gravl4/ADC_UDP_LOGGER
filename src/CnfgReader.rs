// mod CnfgData;

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_parens)]

use std::io::{stdin, stdout, Read, Write};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::ptr;
use std::os::raw::c_ulong;

// #[path = "DrawParams.rs"]
use crate::DrawParams:: { /* A_CH_OPT, D_CH_OPT, A_CH_DRAW_DATA, D_CH_DRAW_DATA, */ arr_A_Ch_Params, arr_D_Ch_Params }; 
use crate::Data:: { NUM_OF_A_CHS, NUM_OF_D_CHS, PacketsFreq }; 

// #[path = "udp.rs"]
use crate::udp:: { IP, Port }; 

use crate::guiVals:: { FontFamily, FontLen, FontWeight };
use crate::gui:: { GRAPH_BCK_COLOR, GRID_LINE_COLOR, TIME_VALS_COLOR };
use crate::guiCntrls:: { CNTRL_BCK_COLOR, CNTRL_HIGHLIGHT_COLOR, CNTRL_TXT_COLOR, CNTRL_LINE_COLOR };

const FN: &str = "./cnfg.txt";


fn DsplCnfgErr(pCnfgLine: &str, pDescr: &str,  pDetail: *const String)
{
  println!("{}", pCnfgLine); 
  println!("{}", pDescr);
unsafe
{  
  if !pDetail.is_null() { println!("{}", *pDetail); } 
}
  Pause();
}

enum PARAM_TYPE
{
  color=     1, 
  name=      2,
  GUI=       4,
  min=       8,
  max=      16,
  k=        32,  
  b=        64,
  c=       128,
  invert=  256
}

fn GetStr(ValStr:String, pInited:&mut i32, InitedVal:i32)-> &'static str
{

  *pInited= *pInited | InitedVal;
  Box::leak(ValStr.into_boxed_str())
}

fn GetInt(ValStr:&str, pVal:&mut i32, pInited:&mut i32, InitedVal:i32, CnfgLine:&String, ErrStr:&str)->i32
{
  let Str1: String= ValStr.chars().filter(|c| !c.is_whitespace()).collect();

  match Str1.parse::<i32>()
  {
    Ok(val) => 
    { 
      *pVal= val; 
      *pInited= *pInited | InitedVal;
      0
    }
    Err(e) => 
    { 
      DsplCnfgErr(CnfgLine, ErrStr, &e.to_string());
      1
    }
  }
}

fn GetHex(ValStr:&str, pVal:&mut c_ulong, pInited:&mut i32, InitedVal:i32, CnfgLine:&String, ErrStr:&str)->i32
{
  let Str1: String= ValStr.chars().filter(|c| !c.is_whitespace()).collect();
  let without_prefix = Str1.trim_start_matches("0x");

  match u32::from_str_radix(&without_prefix, 16)
  {
    Err(e) => 
    { 
      DsplCnfgErr(CnfgLine, ErrStr, &e.to_string());
      1
    }
    Ok(val) => 
    { 
      *pVal= val as c_ulong; 
      *pInited= *pInited | InitedVal;
      0
    }
  }
}

fn GetFloat(ValStr:&str, pVal:&mut f32, pInited:&mut i32, InitedVal:i32, CnfgLine:&String, ErrStr:&str)->i32
{
  let Str1: String= ValStr.chars().filter(|c| !c.is_whitespace()).collect();

  match Str1.parse::<f32>()
  {
    Ok(val) => 
    { 
      *pVal= val; 
      *pInited= *pInited | InitedVal;
      0
    }
    Err(e) => 
    { 
      DsplCnfgErr(CnfgLine, ErrStr, &e.to_string());
      1
    }
  }
}

pub fn ReadCnfg()-> bool
{
  const CH_A:i32= 'a' as i32;
  const CH_D:i32= 'd' as i32;

  let mut cnt_Err: i32= 0;
  let mut arr_A_Chs_Inited: [i32; NUM_OF_A_CHS]= [-1; NUM_OF_A_CHS];
  let mut arr_D_Chs_Inited: [i32; NUM_OF_D_CHS]= [-1; NUM_OF_D_CHS];
  let mut IP_inited: i32= -1;
  let mut Port_inited: i32= -1;
  let mut PacketsFreq_inited: i32= -1;
  let mut GUI_inited: i32= -1;
  let path = Path::new(FN);
  match File::open(path) 
  {
    Err(why) => 
    {
      println!("couldn't open {}: {}", FN, why.to_string()); 
      Pause();
      return false;
    }
    Ok(file) => 
    {
      enum SECT_TYPE
      {
        undefined,
        Network,
        GUI,
        A_Ch,
        D_Ch
      }
      let reader = BufReader::new(file);
      let mut ParserState  : i32  = -1;
      let mut SectionFound : bool; // = false;
      let mut SectionType  : i32  = SECT_TYPE::undefined as i32;
      let mut ChType       : i32  = 0;

      for line in reader.lines() 
      {
        match line 
        {
          Err(_why) => { }
          Ok(line) => 
          {
          unsafe 
          {
            let strings: Vec<&str> = line.split("=").collect(); 
            if strings[0].eq("[Network]") 
            {
              ParserState= 0;
              SectionType= SECT_TYPE::Network as i32;
              continue;
            }
            else
            if IP_inited == -1 && strings[0].eq("IP") 
            {
              if SectionType != SECT_TYPE::Network as i32
              {
                cnt_Err= cnt_Err+ 1;
                DsplCnfgErr(&line, "Err. ^^^^^ IP needs to be in [Network] section", ptr::null()); 
              }
              else
              {
                IP_inited= 0;
                IP= GetStr(strings[1].to_string(), &mut IP_inited, 1);
              }
              continue;
            } // IP
            else
            if Port_inited == -1 && strings[0].eq("Port") 
            {
              if SectionType != SECT_TYPE::Network as i32
              {
                cnt_Err= cnt_Err+ 1;
                DsplCnfgErr(&line, "Err. ^^^^^ Port needs to be in  [Network] section", ptr::null()); 
              }
              else
              {
                Port_inited= 0;
                cnt_Err= cnt_Err+ GetInt(strings[1], &mut Port, &mut Port_inited, 1, &line, "Err. ^^^^^ Wrong value for Port number");     
              }
              continue;
            } // Port     
            else
            if PacketsFreq_inited == -1 && strings[0].eq("PacketsFreq") 
            {
              if SectionType != SECT_TYPE::Network as i32
              {
                cnt_Err= cnt_Err+ 1;
                DsplCnfgErr(&line, "Err. ^^^^^ PacketsFreq needs to be in  [Network] section", ptr::null()); 
              }
              else
              {
                PacketsFreq_inited= 0;
                cnt_Err= cnt_Err+ GetInt(strings[1], &mut PacketsFreq, &mut PacketsFreq_inited, 1, &line, "Err. ^^^^^ Wrong value for PacketsFreq number");     
              }
              continue;
            } // PacketsFreq     
            else
            if strings[0].eq("[GUI]") 
            {
              ParserState= 0;
              SectionType= SECT_TYPE::GUI as i32;
              continue;
            }
            else
            if strings[0].eq("GRAPH_BCK_COLOR") 
            {
              if SectionType != SECT_TYPE::GUI as i32
              {
                cnt_Err= cnt_Err+ 1;
                DsplCnfgErr(&line, "Err. ^^^^^ GRAPH_BCK_COLOR needs to be in [GUI] section", ptr::null()); 
              }
              else
              { 
                GUI_inited= 0;
                cnt_Err= cnt_Err+ GetHex(strings[1], &mut GRAPH_BCK_COLOR, 
                                        &mut GUI_inited, PARAM_TYPE::color as i32, 
                                        &line, "Err. ^^^^^ Wrong value for color");  
              }
              continue;
            } // GRAPH_BCK_COLOR
            else
            if strings[0].eq("GRID_LINE_COLOR") 
            {
              if SectionType != SECT_TYPE::GUI as i32
              {
                cnt_Err= cnt_Err+ 1;
                DsplCnfgErr(&line, "Err. ^^^^^ GRID_LINE_COLOR needs to be in [GUI] section", ptr::null()); 
              }
              else
              { 
                GUI_inited= 0;
                cnt_Err= cnt_Err+ GetHex(strings[1], &mut GRID_LINE_COLOR, 
                                        &mut GUI_inited, PARAM_TYPE::color as i32, 
                                        &line, "Err. ^^^^^ Wrong value for color");  
              }
              continue;
            } // GRID_LINE_COLOR
            else
            if strings[0].eq("TIME_VALS_COLOR") 
            {
              if SectionType != SECT_TYPE::GUI as i32
              {
                cnt_Err= cnt_Err+ 1;
                DsplCnfgErr(&line, "Err. ^^^^^ TIME_VALS_COLOR needs to be in [GUI] section", ptr::null()); 
              }
              else
              { 
                GUI_inited= 0;
                cnt_Err= cnt_Err+ GetHex(strings[1], &mut TIME_VALS_COLOR, 
                                        &mut GUI_inited, PARAM_TYPE::color as i32, 
                                        &line, "Err. ^^^^^ Wrong value for color");  
              }
              continue;
            } // TIME_VALS_COLOR
            else
            if strings[0].eq("CNTRL_BCK_COLOR") 
            {
              if SectionType != SECT_TYPE::GUI as i32
              {
                cnt_Err= cnt_Err+ 1;
                DsplCnfgErr(&line, "Err. ^^^^^ CNTRL_BCK_COLOR needs to be in [GUI] section", ptr::null()); 
              }
              else
              { 
                GUI_inited= 0;
                cnt_Err= cnt_Err+ GetHex(strings[1], &mut CNTRL_BCK_COLOR, 
                                        &mut GUI_inited, PARAM_TYPE::color as i32, 
                                        &line, "Err. ^^^^^ Wrong value for color");  
              }
              continue;
            } // CNTRL_BCK_COLOR
            else
            if strings[0].eq("CNTRL_HIGHLIGHT_COLOR") 
            {
              if SectionType != SECT_TYPE::GUI as i32
              {
                cnt_Err= cnt_Err+ 1;
                DsplCnfgErr(&line, "Err. ^^^^^ CNTRL_HIGHLIGHT_COLOR needs to be in [GUI] section", ptr::null()); 
              }
              else
              { 
                GUI_inited= 0;
                cnt_Err= cnt_Err+ GetHex(strings[1], &mut CNTRL_HIGHLIGHT_COLOR, 
                                        &mut GUI_inited, PARAM_TYPE::color as i32, 
                                        &line, "Err. ^^^^^ Wrong value for color");  
              }
              continue;
            } // CNTRL_HIGHLIGHT_COLOR
            else
            if strings[0].eq("CNTRL_TXT_COLOR") 
            {
              if SectionType != SECT_TYPE::GUI as i32
              {
                cnt_Err= cnt_Err+ 1;
                DsplCnfgErr(&line, "Err. ^^^^^ CNTRL_TXT_COLOR needs to be in [GUI] section", ptr::null()); 
              }
              else
              { 
                GUI_inited= 0;
                cnt_Err= cnt_Err+ GetHex(strings[1], &mut CNTRL_TXT_COLOR, 
                                        &mut GUI_inited, PARAM_TYPE::color as i32, 
                                        &line, "Err. ^^^^^ Wrong value for color");  
              }
              continue;
            } // CNTRL_TXT_COLOR
            else
            if strings[0].eq("CNTRL_LINE_COLOR") 
            {
              if SectionType != SECT_TYPE::GUI as i32
              {
                cnt_Err= cnt_Err+ 1;
                DsplCnfgErr(&line, "Err. ^^^^^ CNTRL_LINE_COLOR needs to be in [GUI] section", ptr::null()); 
              }
              else
              { 
                GUI_inited= 0;
                cnt_Err= cnt_Err+ GetHex(strings[1], &mut CNTRL_LINE_COLOR, 
                                        &mut GUI_inited, PARAM_TYPE::color as i32, 
                                        &line, "Err. ^^^^^ Wrong value for color");  
              }
              continue;
            } // CNTRL_LINE_COLOR
            else
            if strings[0].eq("FontFamily") 
            {
              if SectionType != SECT_TYPE::GUI as i32
              {
                cnt_Err= cnt_Err+ 1;
                DsplCnfgErr(&line, "Err. ^^^^^ FontFamily needs to be in [GUI] section", ptr::null()); 
              }
              else
              {
                GUI_inited= 0;
                FontFamily= GetStr(strings[1].to_string(), &mut GUI_inited, 1);
              }
              continue;
            } // FontFamily
            else
            if strings[0].eq("FontWeight") 
            {
              if SectionType != SECT_TYPE::GUI as i32
              {
                cnt_Err= cnt_Err+ 1;
                DsplCnfgErr(&line, "Err. ^^^^^ FontWeight needs to be in [GUI] section", ptr::null()); 
              }
              else
              {
                GUI_inited= 0;
                FontWeight= GetStr(strings[1].to_string(), &mut GUI_inited, 1);
              }
              continue;
            } // FontWeight
            else
            if strings[0].eq("FontLen") 
            {
              if SectionType != SECT_TYPE::GUI as i32
              {
                cnt_Err= cnt_Err+ 1;
                DsplCnfgErr(&line, "Err. ^^^^^ FontLen needs to be in  [GUI] section", ptr::null()); 
              }
              else
              {
                GUI_inited= 0;
                cnt_Err= cnt_Err+ GetInt(strings[1], &mut FontLen, &mut GUI_inited, 1, &line, "Err. ^^^^^ Wrong value for Port number");     
              }
              continue;
            } // FontLen     
            else
            {
              SectionFound= false;
              for i in 0.. NUM_OF_A_CHS
              {
                let Section: String = format!("[Ch_A{}]", (i+ 1).to_string());
                if strings[0].eq(&Section) 
                {
                  ParserState= i as i32;
                  ChType= CH_A;
                  SectionFound= true;
                  SectionType= SECT_TYPE::A_Ch as i32; 
                  arr_A_Chs_Inited[ParserState as usize]= 0;
                  break;
                }
              }
              if SectionFound { continue; }
             
              SectionFound= false;
              for i in 0.. NUM_OF_D_CHS
              {
                let Section: String = format!("[Ch_D{}]", (i+ 1).to_string());
                if strings[0].eq(&Section) 
                {
                  ParserState= i as i32;
                  ChType= CH_D;
                  SectionFound= true;
                  SectionType= SECT_TYPE::D_Ch as i32; 
                  arr_D_Chs_Inited[ParserState as usize]= 0;
                  break;
                }
              }
              if SectionFound { continue; }

              if ChType == CH_A
              {
                if strings[0].eq("k")
                {
                  cnt_Err= cnt_Err+ GetFloat(strings[1], &mut arr_A_Ch_Params[ParserState as usize].Ch.k, 
                                            &mut arr_A_Chs_Inited[ParserState as usize], PARAM_TYPE::k as i32, 
                                            &line, "Err. ^^^^^ Wrong value for k");     
                  continue;  
                }
                else
                if strings[0].eq("b")
                {
                  cnt_Err= cnt_Err+ GetInt(strings[1], &mut arr_A_Ch_Params[ParserState as usize].Ch.b, 
                                             &mut arr_A_Chs_Inited[ParserState as usize], PARAM_TYPE::b as i32, 
                                             &line, "Err. ^^^^^ Wrong value for b"); 
                  continue;  
                }
                else
                if strings[0].eq("c")
                {
                  cnt_Err= cnt_Err+ GetInt(strings[1], &mut arr_A_Ch_Params[ParserState as usize].Ch.c, 
                                             &mut arr_A_Chs_Inited[ParserState as usize], PARAM_TYPE::c as i32, 
                                             &line, "Err. ^^^^^ Wrong value for c");
                  continue;  
                }
                else
                if strings[0].eq("visible")
                {
                  cnt_Err= cnt_Err+ GetInt(strings[1], &mut arr_A_Ch_Params[ParserState as usize].Ch.visible, 
                                            &mut GUI_inited, PARAM_TYPE::GUI as i32, 
                                            &line, "Err. ^^^^^ Wrong value for Channel Visible"); 
                  continue;  
                }
                else
                if strings[0].eq("min")
                {
                  cnt_Err= cnt_Err+ GetInt(strings[1], &mut arr_A_Ch_Params[ParserState as usize].Ch.min, 
                                             &mut arr_A_Chs_Inited[ParserState as usize], PARAM_TYPE::min as i32, 
                                             &line, "Err. ^^^^^ Wrong value for min");
                  continue;  
                }
                else
                if strings[0].eq("max")
                {
                  cnt_Err= cnt_Err+ GetInt(strings[1], &mut arr_A_Ch_Params[ParserState as usize].Ch.max, 
                                             &mut arr_A_Chs_Inited[ParserState as usize], PARAM_TYPE::max as i32, 
                                             &line, "Err. ^^^^^ Wrong value for max");      
                  continue;  
                }
              }
              if ChType == CH_D
              {
                if strings[0].eq("invert")
                {
                  cnt_Err= cnt_Err+ GetInt(strings[1], &mut arr_D_Ch_Params[ParserState as usize].Ch.invert, 
                                             &mut arr_D_Chs_Inited[ParserState as usize], PARAM_TYPE::invert as i32, 
                                             &line, "Err. ^^^^^ Wrong value for invert");  
                  continue;                           
                }
              }
              
              if strings[0].eq("color")
              {
                 
                  if ChType == CH_A 
                  {
                    cnt_Err= cnt_Err+ GetHex(strings[1], &mut arr_A_Ch_Params[ParserState as usize].Ch.DsplOpt.color, 
                                             &mut arr_A_Chs_Inited[ParserState as usize], PARAM_TYPE::color as i32, 
                                             &line, "Err. ^^^^^ Wrong value for color"); 
                    continue;
                  }
                  if ChType == CH_D 
                  { 
                    cnt_Err= cnt_Err+ GetHex(strings[1], &mut arr_D_Ch_Params[ParserState as usize].Ch.DsplOpt.color, 
                                           &mut arr_D_Chs_Inited[ParserState as usize], PARAM_TYPE::color as i32, 
                                           &line, "Err. ^^^^^ Wrong value for color");  
                    continue;
                  }
              }
              else
              if strings[0].eq("name")
              {
                if ChType == CH_A 
                { 
//                  arr_A_Ch_Params[ParserState as usize].Ch.DsplOpt.name= strings[1].to_string();
//                  arr_A_Chs_Inited[ParserState as usize]= arr_A_Chs_Inited[ParserState as usize] | PARAM_TYPE::name as i32;
                  arr_A_Ch_Params[ParserState as usize].Ch.DsplOpt.name= GetStr(strings[1].to_string(), &mut arr_A_Chs_Inited[ParserState as usize], PARAM_TYPE::name as i32);
                  arr_A_Ch_Params[ParserState as usize].Ch.DsplOpt.name_len= arr_A_Ch_Params[ParserState as usize].Ch.DsplOpt.name.chars().count() as i32;
                  continue;
                }
                if ChType == CH_D 
                {
//                  arr_D_Ch_Params[ParserState as usize].Ch.DsplOpt.name= strings[1].to_string();
//                  arr_D_Chs_Inited[ParserState as usize]= arr_D_Chs_Inited[ParserState as usize] | PARAM_TYPE::name as i32;
                  arr_D_Ch_Params[ParserState as usize].Ch.DsplOpt.name= GetStr(strings[1].to_string(), &mut arr_D_Chs_Inited[ParserState as usize], PARAM_TYPE::name as i32);
                  arr_D_Ch_Params[ParserState as usize].Ch.DsplOpt.name_len= arr_D_Ch_Params[ParserState as usize].Ch.DsplOpt.name.chars().count() as i32;
                  continue;
                }
              }
            }
          } // unsafe 
          } // OK
        } // line
      } // for line in reader.lines() 
    } // file OK
  };    
  
  if cnt_Err != 0 
  { 
    Pause();
    return false;
  }
 
  if IP_inited   != 1     { cnt_Err= 1; println!("Err. IP missed"); }
  if Port_inited != 1     { cnt_Err= 1; println!("Err. Port missed"); }
  if PacketsFreq_inited != 1 { cnt_Err= 1; println!("Err. PacketsFreq missed"); }
  if cnt_Err != 0 
  { 
    Pause();
    return false;
  }
  unsafe 
  { 
  println!("FontFamily: {}, FontLen: {}, FontWeight: {}", FontFamily, FontLen, FontWeight);
  }
  for i in 0.. NUM_OF_A_CHS
  {
    if arr_A_Chs_Inited[i] == -1
    {  cnt_Err= 1; println!("Err. Missed options for Analog channel: {}", i+ 1); }
    else
    {
      if arr_A_Chs_Inited[i] & PARAM_TYPE::color as i32 ==  0
      {
          cnt_Err= 1; 
          println!("Err. Missed color for Analog channel: {}", i+ 1); 
      }
      if arr_A_Chs_Inited[i] & PARAM_TYPE::name as i32   ==  0
      {
          cnt_Err= 1; 
          println!("Err. Missed name for Analog channel: {}", i+ 1); 
      }
/*
      if arr_A_Chs_Inited[i] & PARAM_TYPE::min as i32    ==  0 
      {
          cnt_Err= 1; 
          println!("Err. Missed min for Analog channel: {}", i+ 1); 
      }
      if arr_A_Chs_Inited[i] & PARAM_TYPE::max  as i32   ==  0 
      {
          cnt_Err= 1; 
          println!("Err. Missed max for Analog channel: {}", i+ 1); 
      }
*/
      if arr_A_Chs_Inited[i] & PARAM_TYPE::k as i32      ==  0   
      {
          cnt_Err= 1; 
          println!("Err. Missed k for Analog channel: {}", i+ 1); 
      }
      if arr_A_Chs_Inited[i] & PARAM_TYPE::b as i32      ==  0   
      {
          cnt_Err= 1; 
          println!("Err. Missed b for Analog channel: {}", i+ 1); 
      }
      if arr_A_Chs_Inited[i] & PARAM_TYPE::c as i32      ==  0   
      {
          cnt_Err= 1; 
          println!("Err. Missed c for Analog channel: {}", i+ 1); 
      }
    }                                      
  }

  for i in 0.. NUM_OF_D_CHS
  {
    if arr_D_Chs_Inited[i] == -1
    {  cnt_Err= 1; println!("Err. Missed options for Digital channel: {}", i+ 1); }
    else
    {
      if arr_D_Chs_Inited[i] & PARAM_TYPE::color as i32 ==  0
      {
          cnt_Err= 1; 
          println!("Err. Missed color for Digital channel: {}", i+ 1); 
      }
      if arr_D_Chs_Inited[i] & PARAM_TYPE::name as i32   ==  0
      {
          cnt_Err= 1; 
          println!("Err. Missed name for Digital channel: {}", i+ 1); 
      }
      if arr_D_Chs_Inited[i] & PARAM_TYPE::invert as i32 ==  0 
      {
          cnt_Err= 1; 
          println!("Err. Missed invert for Digital channel: {}", i+ 1); 
      }
    }
  }

  if cnt_Err != 0 
  { 
    Pause();  
    return false; 
  }

  return true; 

}

pub fn Pause() 
{
  let mut stdout = stdout();
  stdout.write(b"\nPress Enter to continue...").unwrap();
  stdout.flush().unwrap();
  stdin().read(&mut [0]).unwrap();
  stdout.write(b"\n").unwrap();
}

