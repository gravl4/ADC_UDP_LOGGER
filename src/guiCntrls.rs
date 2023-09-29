// mos GUI

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_parens)]

#![cfg_attr(not(feature = "xlib"), allow(dead_code))]
#![cfg_attr(not(feature = "xlib"), allow(unused_imports))]

// #[path = "deps/libx11-1f2d22a51889d0d7.rlib"]
extern crate x11;
use x11::xlib;

use std::sync::atomic::{AtomicI32, Ordering};
use std::mem;
use std::ptr;
use std::os::raw::*;
use std::ffi::CString;

use crate::DataSweep:: { DATA_SWEEP_TYPE };
use crate::guiVals:: { iScr, pDspl, Wnd_root, Wnd_main, pGC, pGC_xor, gcVals, pFontInfo, FontAscent, FontDescent, DX_TXT, DY_TXT };
use crate::Dbg:: { DBG_MSG };

const CNTRL_MIN_LEN                  : i32     = 14;
const D_BORDER                       : i32     = 2;
pub static mut CNTRL_BCK_COLOR       : c_ulong = 0x00FFFFFF;
pub static mut CNTRL_HIGHLIGHT_COLOR : c_ulong = 0x00FFFF0F;
pub static mut CNTRL_TXT_COLOR       : c_ulong = 0x00000000;
pub static mut CNTRL_LINE_COLOR      : c_ulong = 0x00AFAFAF;

pub struct CNTRL
{ 
  pub Wnd     : c_ulong,
  pub Txt     : &'static str, 
  pub TxtLen  : i32,
  pub w       : i32, 
  pub h       : i32,
  xTxt        : i32,
  yTxt        : i32,
  xxTxt       : i32,
  yyTxt       : i32,
  PixMap      : xlib::Pixmap,
  BttnPressed : i32,
  IsMouseOver : bool,
  BckColor    : c_ulong,
  TxtColor    : c_ulong,
}

impl CNTRL 
{
  pub const fn init() -> Self 
  {
    Self
    { 
      Wnd         : 0,
      Txt         : "",
      TxtLen      : 0,
      w           : 0,
      h           : 0,
      xTxt        : 0,
      yTxt        : 0,
      xxTxt       : 0,
      yyTxt       : 0,
      PixMap      : 0,
      BttnPressed : 0,
      IsMouseOver : false,
      BckColor    : 0x00AFAFAF,
      TxtColor    : 9
   }
  }

  fn drop(&mut self)
  {
  unsafe 
  {
    xlib::XFreePixmap(pDspl, self.PixMap);
  }
  }

  pub fn SetDefltColors(&mut self)
  {
  unsafe
  {
    self.BckColor= CNTRL_BCK_COLOR;
    self.TxtColor= CNTRL_TXT_COLOR;
  }
  }

  pub fn SetColors(&mut self, BckColor: c_ulong, TxtColor: c_ulong)
  {
    self.BckColor= BckColor;
    self.TxtColor= TxtColor;
  }

  pub fn SetSize(&mut self, w: i32, h: i32)
  {
    self.w= w;
    self.h= h;
    if self.w < self.xxTxt + DX_TXT+ DX_TXT { self.w= self.xxTxt + DX_TXT+ DX_TXT; }
    if self.h < self.yyTxt + DY_TXT+ DY_TXT { self.h= self.yyTxt + DY_TXT+ DY_TXT; }
  unsafe
  {
    gcVals.function = xlib::GXcopy;   
    xlib::XChangeGC (pDspl, pGC, xlib::GCFunction as c_ulong, &mut gcVals);
    self.PixMap =  xlib::XCreatePixmap(pDspl, Wnd_root, self.w as c_uint, self.h as c_uint, xlib::XDefaultDepth(pDspl, iScr) as c_uint); 
    xlib::XSetForeground(pDspl, pGC, self.BckColor);
    xlib::XFillRectangle(pDspl, self.PixMap, pGC, 0, 0, self.w as u32, self.h as u32);
  }
  }

  pub fn CreateWnd(&mut self, x0: i32, y0: i32, BorderLen: u32)
  {
  unsafe
  {
    self.Wnd = xlib::XCreateSimpleWindow ( pDspl, Wnd_main, x0, y0, self.w as u32, self.h as u32, BorderLen, CNTRL_LINE_COLOR, CNTRL_BCK_COLOR);
    xlib::XSelectInput ( pDspl, self.Wnd,  xlib::ExposureMask 
			                                  | xlib::ButtonPressMask 
			                                  | xlib::ButtonReleaseMask 
			                                  | xlib::EnterWindowMask
			                                  | xlib::LeaveWindowMask 
			                                  | xlib::PointerMotionMask 
			                                  | xlib::FocusChangeMask 
			                                  | xlib::KeyPressMask 
			                                  | xlib::KeyReleaseMask 
			                                  | xlib::SubstructureNotifyMask
                                        | xlib::StructureNotifyMask
			                                  | xlib::LeaveWindowMask
                                        | xlib::EnterWindowMask);
  
  }
  }

  pub fn Show(&mut self)
  {
  unsafe
  {
    xlib::XMapWindow ( pDspl, self.Wnd );
  }
  }

  pub fn Hide(&mut self)
  {
  unsafe
  {
    xlib::XUnmapWindow ( pDspl, self.Wnd );
  }
  }

  pub fn Refresh(&mut self)
  {
  unsafe
  {
    if self.Wnd == 0 { return; }
    if self.PixMap == 0{ return; }
    xlib::XCopyArea(pDspl, self.PixMap, self.Wnd, pGC, 0, 0, self.w as c_uint, self.h as c_uint, 0, 0); 
  }
  }

  pub fn SetTitle(&mut self, Txt: String)
  {
  unsafe
  {
    self.Txt= Box::leak(Txt.into_boxed_str());
    self.TxtLen= self.Txt.len() as i32;

    let mut direction  : c_int = 0;
    let mut ascent     : c_int = 0;
    let mut descent    : c_int = 0;
    let mut overall    : xlib::XCharStruct= mem::MaybeUninit::zeroed().assume_init();

    xlib::XTextExtents (pFontInfo, self.Txt.as_ptr() as *mut c_char, self.TxtLen, &mut direction , &mut ascent, &mut descent, &mut overall);
    self.xxTxt= overall.width as i32;
    self.yyTxt= (FontAscent+ FontDescent) as i32; 
  }
  }

  pub fn Draw(&mut self)
  {
  unsafe
  {
    xlib::XSetForeground(pDspl, pGC, self.TxtColor);
    xlib::XDrawString(pDspl, self.PixMap, pGC, DX_TXT+ (self.w- self.xxTxt) / 2, DY_TXT+ FontAscent, self.Txt.as_ptr() as *mut c_char, self.TxtLen); 
  }
  }
}

pub struct RADIO_BTTN
{
  pub Cntrl         : CNTRL,
  pub Checked       : AtomicI32,
  pub LinkedVal : *mut i32, 
}

impl RADIO_BTTN 
{
  pub const fn init() -> Self 
  {
    Self
    { 
      Cntrl     : CNTRL::init(),
      Checked   : AtomicI32::new(0),
      LinkedVal : ptr::null_mut(),
    }
  }

  pub fn SetSize(&mut self, w: i32, h: i32)
  {
    let mut xx= DX_TXT+ DX_TXT+ DX_TXT+ DX_TXT+ DX_TXT+ CNTRL_MIN_LEN+ self.Cntrl.xxTxt;
    if xx < w { xx= w; }
    let mut yy= unsafe{ FontAscent+ FontDescent }+ DY_TXT+ DY_TXT;
    if yy < CNTRL_MIN_LEN+ 4 { yy= CNTRL_MIN_LEN+ 4; }
    if yy < h { yy= h; }
    self.Cntrl.SetSize(xx, yy);
  }

  pub fn Draw(&mut self)
  {
  unsafe
  {
    xlib::XSetForeground(pDspl, pGC, self.Cntrl.BckColor);
    xlib::XFillRectangle(pDspl, self.Cntrl.PixMap, pGC, 0, 0, self.Cntrl.w as u32, self.Cntrl.h as u32);
    let mut d= CNTRL_MIN_LEN;
    let y= (self.Cntrl.h- CNTRL_MIN_LEN)/ 2;
    xlib::XSetForeground(pDspl, pGC, self.Cntrl.TxtColor); 
    xlib::XDrawString(pDspl, self.Cntrl.PixMap, pGC, DX_TXT+ d+ DX_TXT+ DX_TXT, (self.Cntrl.h- FontAscent- FontDescent)/ 2+ FontAscent, self.Cntrl.Txt.as_ptr() as *mut c_char, self.Cntrl.TxtLen); 
    xlib::XSetLineAttributes(pDspl, pGC, 1, xlib::LineSolid, xlib::CapButt, xlib::JoinBevel);
    xlib::XDrawArc(pDspl, self.Cntrl.PixMap, pGC, DX_TXT, y, d as u32, d as u32, 0, 360* 64);
    if self.Checked.load(Ordering::Relaxed) == 1
    { 
      d= d- 6;
      xlib::XFillArc(pDspl, self.Cntrl.PixMap, pGC, DX_TXT+ 3, y+ 3, d as u32, d as u32, 0, 360* 64); 
    }
  }
  }

  pub fn Toggle(&mut self)
  {
  unsafe
  {
    if self.Checked.fetch_xor(1, Ordering::SeqCst) == 0
    {
      xlib::XSetForeground(pDspl, pGC, self.Cntrl.TxtColor); 
    }
    else
    {
      xlib::XSetForeground(pDspl, pGC, self.Cntrl.BckColor); 
    }
    xlib::XFillArc(pDspl, self.Cntrl.PixMap, pGC, DX_TXT+ 3, (self.Cntrl.h- CNTRL_MIN_LEN)/ 2+ 3, (CNTRL_MIN_LEN- 6) as u32, (CNTRL_MIN_LEN- 6) as u32, 0, 360* 64); 
    xlib::XCopyArea(pDspl, self.Cntrl.PixMap, self.Cntrl.Wnd, pGC, 0, 0, self.Cntrl.w as c_uint, self.Cntrl.h as c_uint, 0, 0); 
  }
  if self.LinkedVal != ptr::null_mut() { unsafe { *self.LinkedVal= *self.LinkedVal ^ 1; } }
  }

  pub fn OnMouseLeave(&mut self)
  {
    self.Cntrl.BttnPressed= 0;
  }

  pub fn OnBttnPress(&mut self)
  {
    self.Cntrl.BttnPressed= 1;
  }

  pub fn OnBttnRelease(&mut self)->bool
  {
    if self.Cntrl.BttnPressed != 0
    {
      self.Cntrl.BttnPressed= 0;
      self.Toggle();
      return true;
    }
    return false;
  }
}

pub struct SWEEP_TYPE_LIST
{
  pub Cntrl         : CNTRL,
  pub arr_TxtLine   : [&'static str; DATA_SWEEP_TYPE::NUM_OF_DATA_SWEEP_TYPES as usize],
  pub arr_TxtLen    : [i32; DATA_SWEEP_TYPE::NUM_OF_DATA_SWEEP_TYPES as usize],
  pub arr_yLine     : [i32; DATA_SWEEP_TYPE::NUM_OF_DATA_SWEEP_TYPES as usize],
  pub yy_Line       : i32,
  PixMap_Line_Mask  : xlib::Pixmap,
  iLine             : i32,
  pub DataSweepType : AtomicI32
}

impl SWEEP_TYPE_LIST 
{
  pub const fn init() -> Self 
  {
    Self
    { 
      Cntrl            : CNTRL::init(),
      arr_TxtLine      : 
                         [ 
//                           "10 msec", 
//                           "100 msec", 
                           "1 sec", 
                           "10 sec",
                           "1 min", 
                           "10 min", 
                           "1 h", 
//                           "3 h", 
//                           "12 h",
                         ],
      arr_TxtLen       : [0; DATA_SWEEP_TYPE::NUM_OF_DATA_SWEEP_TYPES as usize],
      arr_yLine        : [0; DATA_SWEEP_TYPE::NUM_OF_DATA_SWEEP_TYPES as usize],
      yy_Line          : 0,
      PixMap_Line_Mask : 0,
      iLine            : 0,
      DataSweepType    : AtomicI32::new(DATA_SWEEP_TYPE::_1_sec as i32)
  }
  }

  fn drop(&mut self)
  {
  unsafe 
  {
    xlib::XFreePixmap(pDspl, self.PixMap_Line_Mask);
  }
  }

  pub fn InitDraw(&mut self, /*  w: i32, h: i32*/ )
  {
  unsafe
  {
    let mut xx          : i32                      = 0; 
    let mut yy          : i32                      = 0;

    let mut direction   : c_int                    = 0;
    let mut ascent      : c_int                    = 0;
    let mut descent     : c_int                    = 0;
    let mut overall     : xlib::XCharStruct        = mem::MaybeUninit::zeroed().assume_init();

    for i in 0.. DATA_SWEEP_TYPE::NUM_OF_DATA_SWEEP_TYPES as usize
    {
      self.arr_TxtLen[i]= self.arr_TxtLine[i].len() as i32;
      xlib::XTextExtents (pFontInfo, self.arr_TxtLine[i].as_ptr() as *mut c_char, self.arr_TxtLen[i], &mut direction , &mut ascent, &mut descent, &mut overall);
      if xx < overall.width as i32 { xx= overall.width as i32; }
    }
    self.yy_Line= FontAscent+ FontDescent+ DY_TXT+ DY_TXT;
    xx= xx+ DX_TXT+ DX_TXT;
    yy= self.yy_Line* DATA_SWEEP_TYPE::NUM_OF_DATA_SWEEP_TYPES as i32;
    self.Cntrl.w= xx;
    self.Cntrl.h= yy;
  }
  }

  pub fn Draw(&mut self)
  {
  unsafe
  {
//    self.Cntrl.PixMap      =  xlib::XCreatePixmap(pDspl, Wnd_root, self.Cntrl.w as c_uint, self.Cntrl.h as c_uint, xlib::XDefaultDepth(pDspl, iScr) as c_uint); 
    self.PixMap_Line_Mask  =  xlib::XCreatePixmap(pDspl, Wnd_root, self.Cntrl.w as c_uint, self.yy_Line as c_uint, xlib::XDefaultDepth(pDspl, iScr) as c_uint); 

    xlib::XSetForeground(pDspl, pGC, CNTRL_BCK_COLOR); 
    xlib::XFillRectangle(pDspl, self.Cntrl.PixMap, pGC,  0, 0, self.Cntrl.w as c_uint, self.Cntrl.h as c_uint);
    xlib::XSetForeground(pDspl, pGC, CNTRL_LINE_COLOR); 
    xlib::XSetLineAttributes(pDspl, pGC, 1, xlib::LineOnOffDash, xlib::CapButt, xlib::JoinBevel);
    let mut y= 0; // self.yy_Line;
    for i in 0.. DATA_SWEEP_TYPE::NUM_OF_DATA_SWEEP_TYPES as usize
    {
      self.arr_yLine[i]= y;
      xlib::XSetForeground(pDspl, pGC, CNTRL_TXT_COLOR); 
      xlib::XDrawString(pDspl, self.Cntrl.PixMap, pGC, DX_TXT, y+ FontAscent+ DY_TXT, self.arr_TxtLine[i].as_ptr() as *mut c_char, self.arr_TxtLen[i]); 
      xlib::XSetForeground(pDspl, pGC, CNTRL_LINE_COLOR); 
      y+= self.yy_Line;
      xlib::XDrawLine(pDspl, self.Cntrl.PixMap, pGC, DX_TXT, y, self.Cntrl.w- DX_TXT, y); 
    }
    xlib::XSetForeground(pDspl, pGC, CNTRL_BCK_COLOR ^ CNTRL_HIGHLIGHT_COLOR); 
    xlib::XFillRectangle(pDspl, self.PixMap_Line_Mask, pGC,  0, 0, self.Cntrl.w as c_uint, self.yy_Line as c_uint);
  }
  }

  pub fn CreateWnd(&mut self, x0: i32, y0: i32)
  {
    self.Cntrl.CreateWnd(x0, y0, 2);
  }

  pub fn Refresh(&mut self)
  {
  unsafe
  {
    xlib::XCopyArea(pDspl, self.Cntrl.PixMap, self.Cntrl.Wnd, pGC, 0, 0, self.Cntrl.w as c_uint, self.Cntrl.h as c_uint, 0, 0); 
    if self.iLine < DATA_SWEEP_TYPE::NUM_OF_DATA_SWEEP_TYPES as i32
    {
      xlib::XCopyArea(pDspl, self.PixMap_Line_Mask, self.Cntrl.Wnd, pGC_xor, 0, 0, self.Cntrl.w as c_uint, self.yy_Line as c_uint, 0, self.arr_yLine[self.iLine as usize]); 
    }
   }
  }

  pub fn Show(&mut self)
  {
  unsafe
  {
    self.Cntrl.BttnPressed= 0;
    self.Cntrl.Show();
    xlib::XRaiseWindow(pDspl, self.Cntrl.Wnd);
  }
  }

  pub fn OnMouseOver(&mut self, y: i32)
  {
    let i: i32= y / self.yy_Line;

    if self.iLine == i { return; }
    if i >= DATA_SWEEP_TYPE::NUM_OF_DATA_SWEEP_TYPES as i32 { return; }
  unsafe
  {
    if self.iLine < DATA_SWEEP_TYPE::NUM_OF_DATA_SWEEP_TYPES  as i32
    {
      xlib::XCopyArea(pDspl, self.PixMap_Line_Mask, self.Cntrl.Wnd, pGC_xor, 0, 0, self.Cntrl.w as c_uint, self.yy_Line as c_uint, 0, self.arr_yLine[self.iLine as usize]); 
    }
    self.iLine= i;
    xlib::XCopyArea(pDspl, self.PixMap_Line_Mask, self.Cntrl.Wnd, pGC_xor, 0, 0, self.Cntrl.w as c_uint, self.yy_Line as c_uint, 0, self.arr_yLine[self.iLine as usize]); 
  }
  }

  pub fn OnMouseLeave(&mut self)
  {
  }

  pub fn OnBttnPress(&mut self)
  {
    self.Cntrl.BttnPressed= 1;
    DBG_MSG!("BttnPressed: {} iLine: {}", self.Cntrl.BttnPressed, self.iLine);
 }

  pub fn OnBttnRelease(&mut self)-> i32
  {
    if self.Cntrl.BttnPressed == 1
    {
      self.Cntrl.BttnPressed= 2;
      self.Cntrl.Hide();
      if *self.DataSweepType.get_mut() == self.iLine { return -1; }
      else
      {
        *self.DataSweepType.get_mut()= self.iLine;    
        return self.iLine;
      }
    }
    return -1;
  }
}

pub struct SWEEP_COMBOBOX
{
  pub Cntrl         : CNTRL,
  pub IsExpanded    : bool,
  pub List          : SWEEP_TYPE_LIST,
}

impl SWEEP_COMBOBOX 
{
  pub const fn init() -> Self 
  {
    Self
    { 
      Cntrl            : CNTRL::init(),
      IsExpanded       : false,
      List             : SWEEP_TYPE_LIST::init(),
    }
  }

  pub fn SetWidth(&mut self, w: i32)
  {
    self.List.InitDraw();
    let mut xx: i32= self.List.Cntrl.w+ 20; // self.List.yy_Line;
    if xx < w { xx= w; }
    self.List.Cntrl.SetSize(xx, self.List.Cntrl.h);
    let yy= unsafe { FontAscent+ FontDescent+ DY_TXT+ DY_TXT };
    self.Cntrl.SetSize(xx, yy);
    self.List.Draw();
    self.List.iLine= 0;
    self.Draw();
  }

  pub fn ChangeSel(&mut self, iLine: i32)
  {
  unsafe
  {
    self.IsExpanded= false;

    if iLine >= 0 && iLine < DATA_SWEEP_TYPE::NUM_OF_DATA_SWEEP_TYPES as i32
    {
      xlib::XSetForeground(pDspl, pGC, CNTRL_BCK_COLOR); 
      xlib::XFillRectangle(pDspl, self.Cntrl.PixMap, pGC,  0, 0, self.Cntrl.w as c_uint - 21, self.Cntrl.h as c_uint);
      let dy: i32= (self.List.yy_Line- FontAscent- FontDescent) / 2;  
      xlib::XSetForeground(pDspl, pGC, CNTRL_TXT_COLOR); 
      xlib::XDrawString(pDspl, self.Cntrl.PixMap, pGC, DX_TXT, self.Cntrl.h- dy, self.List.arr_TxtLine[self.List.iLine as usize].as_ptr() as *mut c_char, self.List.arr_TxtLen[self.List.iLine as usize]); 
      self.Cntrl.Refresh();
    }
  }
  }

  pub fn Draw(&mut self)
  {
  unsafe
  {
    gcVals.function = xlib::GXcopy;   
    xlib::XChangeGC (pDspl, pGC, xlib::GCFunction as c_ulong, &mut gcVals);

    xlib::XSetLineAttributes(pDspl, pGC, 1, xlib::LineSolid, xlib::CapButt, xlib::JoinBevel);
    xlib::XSetForeground(pDspl, pGC, CNTRL_BCK_COLOR); 
    xlib::XFillRectangle(pDspl, self.Cntrl.PixMap, pGC,  0, 0, self.Cntrl.w as c_uint, self.Cntrl.h as c_uint);

    self.ChangeSel(0);

    xlib::XSetForeground(pDspl, pGC, CNTRL_LINE_COLOR); 
    let mut x: c_int= self.Cntrl.w- 21;
    xlib::XDrawLine(pDspl, self.Cntrl.PixMap, pGC, x, 0, x, self.Cntrl.h); 
    x= x+ 1;
    xlib::XSetForeground(pDspl, pGC, CNTRL_TXT_COLOR); 
    let y: c_int= self.Cntrl.h- DY_TXT- 4; 
    for _i in 0.. 3
    {
      x= x+ 2;
      xlib::XFillRectangle(pDspl, self.Cntrl.PixMap, pGC, x, y, 4, 4);
      x= x+ 4;
    }
  }
  }
 
  pub fn CreateWnd(&mut self, x0: i32, y0: i32)
  {
    self.Cntrl.CreateWnd(x0, y0, 2);
    self.List.CreateWnd(x0, y0- self.List.Cntrl.h- 1);
  }

  pub fn Show(&mut self)
  {
    self.Cntrl.Show();
  }

  pub fn OnMouseLeave(&mut self)
  {
  }

  pub fn OnBttnPress(&mut self)
  {
    self.Cntrl.BttnPressed= 1;
  }

  pub fn OnBttnRelease(&mut self)
  {
    if !self.IsExpanded && (self.Cntrl.BttnPressed != 0)  
    {
      self.Cntrl.BttnPressed= 0;
      self.IsExpanded= true;
      self.List.Show();
    }
  }
}