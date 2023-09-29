// mos GUI

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(unused_macros)]

#![cfg_attr(not(feature = "xlib"), allow(dead_code))]
#![cfg_attr(not(feature = "xlib"), allow(unused_imports))]

// #[path = "deps/libx11-1f2d22a51889d0d7.rlib"]
extern crate x11;
use x11::xlib;

use std::sync::mpsc::{self};
use std::sync::Mutex; 
use std::sync::RwLock;
use std::sync::Arc;
use std::sync::Once;
use std::sync::atomic::{AtomicI32, Ordering};
use std::time;
use std::time::{ SystemTime, Duration };
use std::mem;
use std::ptr;
use std::os::raw::*;
use std::ffi::CString;

use chrono::{Datelike, Timelike, Utc};

// #[path = "CnfgData.rs"]
use crate::CnfgReader:: { Pause };
use crate::DrawParams:: { arr_A_Ch_Params, arr_D_Ch_Params, ADC_max };

// #[path = "Data.rs"]
use crate::Data:: { NUM_OF_A_CHS, NUM_OF_D_CHS, DATA_PACKET };

use crate::guiVals:: { iScr, pDspl, Wnd_root, Wnd_main, pGC, pGC_xor, pGC_or, gcVals, pFontInfo, FontAscent, FontDescent, DX_TXT, DY_TXT, FontFamily, FontLen, FontWeight };
use crate::guiCntrls:: { CNTRL, SWEEP_TYPE_LIST, SWEEP_COMBOBOX, RADIO_BTTN };
use crate::DataSweep:: { DATA_SWEEP_TYPE, NUM_OF_MIN_MAX_VALS };
use crate::Dbg:: { DBG_MSG, SET_START, PRINT_DURATION, PRINT_TRACE };

pub static mut pGUIEvntSender  : *mut mpsc::Sender<i32> = ptr::null_mut(); 

const WND_PANEL_WIDTH          :      i32=                150;

static mut w_min            : i32               =  0;
static mut h_min            : i32               =  0; 
static mut Width            : c_int             =  0;
static mut Height           : c_int             =  0;
static mut pImg_Grid        : *mut xlib::XImage =  ptr::null_mut(); 
static mut pImg_Graph       : *mut xlib::XImage =  ptr::null_mut(); 
static mut PixMap_Grid      : xlib::Pixmap      =  0;
static mut PixMap_D_Ch0     : xlib::Pixmap      =  0;
static mut PixMap_D_Ch1     : xlib::Pixmap      =  0;
static mut PixMap_A_Ch0     : xlib::Pixmap      =  0;
static mut arr_PixMap_A_Chs0    : [xlib::Pixmap; NUM_OF_A_CHS]      =  [0; NUM_OF_A_CHS];
static mut arr_PixMap_A_Chs1    : [xlib::Pixmap; NUM_OF_A_CHS]      =  [0; NUM_OF_A_CHS];
static mut PixMap_Wnd       : xlib::Pixmap      =  0;
static mut WndTitleHeight   : c_int             = 32;
static mut TopBar_yy        : c_int             = 0;
static mut BottomBar_yy     : c_int             = 0;
static mut DrawArea_xLeft   : c_int             =  WND_PANEL_WIDTH;
static mut DrawArea_yTop    : c_int             =  0;
static mut DrawArea_w       : c_int             =  0; 
static mut DrawArea_h       : c_int             =  0;
static mut Graph_xx         : c_int             =  0;
static mut Graph_A_Ch_yTop  : c_int             =  0;
static mut Graph_D_Ch_yTop  : c_int             =  0;
static mut Graph_A_Ch_yy    : c_int             =  0;
static mut Graph_D_Ch_yy    : c_int             =  0;
static mut Graph_x0         : c_int             =  0;
static mut Graph_A_Ch_y0    : c_int             =  0;
static mut GraphShift_max   : c_int             =  0;
static mut iGraph_prev      : u32               = 3;
static mut NumOfDspl_A_Chs  : usize             = NUM_OF_A_CHS; 

static mut bttn_Pause       : RADIO_BTTN                 = RADIO_BTTN::init();
static mut SweepComboBox    : SWEEP_COMBOBOX             = SWEEP_COMBOBOX::init();
static mut bttn_Show_D_Chs  : RADIO_BTTN                 = RADIO_BTTN::init();
pub static mut GRAPH_BCK_COLOR        : c_ulong        = 0x00FFFFFF;
pub static mut GRID_LINE_COLOR        : c_ulong        = 0x00AFAFAF;
pub static mut TIME_VALS_COLOR        : c_ulong        = 0x00AFAFAF;

static mut Ch_name_xx        : i32            = 0;
static mut Ch_D_txt_xx       : i32            = 0;
static mut Ch_D_yy           : i32            = 11;
static mut ChScale_xLeft     : c_int          = 32;
static mut ChScale_xx        : c_int          = 32;
static mut Graph_dyTop       : c_int          = 32;
static mut TimeScale_yy      : c_int          = 32;
static mut TimeStamp_dx      : i32            = 0;
static mut TimeStampVal_xx   : i32            = 0;
static mut TimeStampExVal_xx : i32            = 0;

static mut mIsPaused         : Mutex<i32>     = Mutex::new(0);

fn Free()
{
unsafe
{
  xlib::XFreeFontInfo(ptr::null_mut(), pFontInfo, 0);
}
}

pub fn InitGUI()
{
unsafe
{
  xlib::XInitThreads();

  pDspl = xlib::XOpenDisplay(ptr::null()); // _1: *const c_char *mut Display

  if pDspl.is_null() { return; }

  iScr = xlib::XDefaultScreen(pDspl);
  let Scr= xlib::XDefaultScreenOfDisplay(pDspl);
  Wnd_root = xlib::XRootWindow(pDspl, iScr);

  let ScrWidth:  c_int= xlib::XWidthOfScreen(Scr);
  let ScrHeight: c_int= xlib::XHeightOfScreen(Scr); 
  Width= ScrWidth / 5 * 4; // 1000
  Height= ScrHeight/ 5* 4; //- WndTitleHeight- WndTitleHeight;

  DBG_MSG!("ScrWidth: {}   ScrHeight: {}", ScrWidth, ScrHeight);

  for i in 0.. NUM_OF_D_CHS
  {
    arr_D_Ch_Params[i].Ch.DsplOpt.color= (arr_D_Ch_Params[i].Ch.DsplOpt.color & 0x00FFFFFF)  as c_ulong;
  }

  for i in 0.. NUM_OF_A_CHS
  {
    arr_A_Ch_Params[i].Ch.DsplOpt.color= (arr_A_Ch_Params[i].Ch.DsplOpt.color & 0x00FFFFFF) as c_ulong;
  }

  xlib::XSync(pDspl, xlib::True);
  xlib::XLockDisplay(pDspl);
  InitGC(ScrWidth, ScrHeight);
  InitCntrls();
  InitDraw();

  if Graph_xx < 1000  
  {
    Width= Width+ (1000- Graph_xx);
    DrawArea_w= Width- DrawArea_xLeft;
    InitDraw();
  }
  
  w_min= Width;
  h_min= Height; 

  CreateWindow(ScrWidth, ScrHeight);
  CreateCntrls();
  ResetDrawData();
  DrawGrid(DATA_SWEEP_TYPE::_1_sec);
  xlib::XUnlockDisplay(pDspl);
}
}

pub fn CreateWindow(w_max: i32, h_max: i32)
{
unsafe 
{
  let mut attributes: xlib::XSetWindowAttributes = mem::MaybeUninit::zeroed().assume_init();
  attributes.background_pixmap= xlib::XDefaultColormap(pDspl, 0);
  attributes.background_pixel =  GRAPH_BCK_COLOR; 
  attributes.event_mask=  xlib::ExposureMask
                        | xlib::SubstructureNotifyMask 
                        | xlib::StructureNotifyMask    
                        | xlib::ButtonPressMask
                        | xlib::ButtonReleaseMask
                        | xlib::PointerMotionMask
                        | xlib::EnterWindowMask
                        | xlib::KeyPressMask
                        | xlib::KeyReleaseMask
                        | xlib::LeaveWindowMask
//                        | xlib::PropertyChangeMask
                        ;

  Wnd_main = xlib::XCreateSimpleWindow(pDspl, Wnd_root, w_max- Width, h_max- Height, Width  as c_uint, Height as c_uint, 0, GRAPH_BCK_COLOR, GRAPH_BCK_COLOR);
  DBG_MSG!("Wnd_main: {}", Wnd_main); 
  xlib::XSetWindowBackground(pDspl, Wnd_main, GRAPH_BCK_COLOR as c_ulong);
  xlib::XClearWindow(pDspl, Wnd_main);

let mut pHints = xlib::XAllocSizeHints();
(*pHints).flags = xlib::PMinSize | xlib::PMaxSize;
(*pHints).min_width = w_min;
(*pHints).min_height = h_min;
(*pHints).max_width = w_max;
(*pHints).max_height = h_max;
//XSetWMNormalHints(d, w, sh);
xlib::XSetWMSizeHints(pDspl, Wnd_main, pHints, xlib::XA_WM_NORMAL_HINTS);
xlib::XFree(pHints as *mut c_void);

  xlib::XSelectInput ( pDspl, Wnd_main, attributes.event_mask );
  let WND_TITLE: String= String::from("Data Visualizer");
  xlib::XStoreName(pDspl, Wnd_main, WND_TITLE.as_ptr() as *mut c_char);
  xlib::XMapWindow(pDspl, Wnd_main);
  xlib::XRaiseWindow(pDspl, Wnd_main);
  xlib::XMoveWindow(pDspl, Wnd_main, 0, 0);
}
}

fn InitCntrls()
{
unsafe
{
  let yy= FontAscent+ FontDescent+ DY_TXT+ DY_TXT;

  bttn_Show_D_Chs.Cntrl.SetDefltColors();
  bttn_Show_D_Chs.Cntrl.SetTitle(String::from("Show D channels"));
  bttn_Show_D_Chs.SetSize(WND_PANEL_WIDTH- 3- DX_TXT, yy);
  bttn_Show_D_Chs.Draw();

  DrawArea_xLeft= bttn_Show_D_Chs.Cntrl.w+ 3+ DX_TXT;

  bttn_Pause.Cntrl.SetDefltColors();
  bttn_Pause.Cntrl.SetTitle(String::from("Pause"));
  bttn_Pause.SetSize(DrawArea_xLeft- 3- DX_TXT, yy);
  bttn_Pause.Draw();

  SweepComboBox.Cntrl.SetDefltColors();
  SweepComboBox.List.Cntrl.SetDefltColors();
  SweepComboBox.SetWidth(DrawArea_xLeft- 3- DX_TXT);
}
}

fn CreateCntrls()
{
unsafe
{
  let mut yy= FontAscent+ FontDescent+ DY_TXT+ DY_TXT;
  let mut y= Height- yy- DY_TXT- 3;

  SweepComboBox.CreateWnd(1, y);
  SweepComboBox.Cntrl.Show();
  
  yy= bttn_Pause.Cntrl.h+ 3; 
  y= y- yy;
  bttn_Pause.Cntrl.CreateWnd(1, y, 2);
  bttn_Pause.Cntrl.Show();

  y= y- yy;
  bttn_Show_D_Chs.Cntrl.CreateWnd(1, y, 2);
  bttn_Show_D_Chs.Cntrl.Show();

  y= 0;
  for i in 0.. NUM_OF_A_CHS
  {
    arr_A_Ch_Params[i].Draw.txt_y= y;
    y+= yy+ 3;
  }
}
}

fn SetFont()
{
unsafe
{
  let mut FontNameStr= CString::new(format!("-*-{}-{}-*-*-*-{}-*-*-*-*-*-*-*", FontFamily, FontWeight, FontLen)).expect("CString::new failed");
  pFontInfo=  xlib::XLoadQueryFont(pDspl, FontNameStr.into_raw());
  if pFontInfo == ptr::null_mut()
  {
    println!("Error of loading {} font", FontFamily);
    println!("Consider using default font");
    FontNameStr= CString::new(format!("-*-*-*-*-*-*-{}-*-*-*-*-*-koi8-r", FontLen)).expect("CString::new failed");
    pFontInfo= xlib::XLoadQueryFont(pDspl, FontNameStr.into_raw());
    if pFontInfo == ptr::null_mut()
    {
      let GC    : xlib::GContext /* XID */ = xlib::XGContextFromGC(pGC);
      pFontInfo = xlib::XQueryFont(pDspl, GC);
      Pause();
      return;
    }
  }
  xlib::XSetFont(pDspl, pGC, ((*pFontInfo) as xlib::XFontStruct).fid);
}
}

pub fn InitGC(ScrWidth: c_int, ScrHeight: c_int)
{
unsafe
{
  let valuemask: c_ulong = 0; // GCBackground | GCForeground | GCFont;		/* which values in 'values' to  */
					/* check when creating the pGC.  */

  gcVals.plane_mask=  /* c_ulong: */ xlib::XAllPlanes();

  pGC = xlib::XCreateGC(pDspl, Wnd_root, valuemask, &mut gcVals);
  if pGC as c_long == 0 
  {
    println!("pGC == 0");
	  return;
  }

  pGC_xor = xlib::XCreateGC(pDspl, Wnd_root, valuemask, &mut gcVals);
  if pGC_xor as c_long == 0 
  {
    println!("pGC_xor == 0");
	  return;
  }

  pGC_or = xlib::XCreateGC(pDspl, Wnd_root, valuemask, &mut gcVals);
  if pGC_or as c_long == 0 
  {
    println!("pGC_or == 0");
	  return;
  }

  gcVals.function = xlib::GXcopy;   
  xlib::XChangeGC (pDspl, pGC, xlib::GCFunction as c_ulong, &mut gcVals);

  gcVals.function = xlib::GXxor;   
  xlib::XChangeGC (pDspl, pGC_xor, xlib::GCFunction as c_ulong, &mut gcVals);

  gcVals.function = xlib::GXor;   
  xlib::XChangeGC (pDspl, pGC_or, xlib::GCFunction as c_ulong, &mut gcVals);

  PixMap_Wnd =    xlib::XCreatePixmap(pDspl, Wnd_root, ScrWidth as c_uint, ScrHeight as c_uint, xlib::XDefaultDepth(pDspl, iScr) as c_uint); 
  PixMap_Grid =   xlib::XCreatePixmap(pDspl, Wnd_root, ScrWidth as c_uint, ScrHeight as c_uint, xlib::XDefaultDepth(pDspl, iScr) as c_uint); 
  PixMap_D_Ch0 = xlib::XCreatePixmap(pDspl, Wnd_root, ScrWidth as c_uint, ScrHeight as c_uint, xlib::XDefaultDepth(pDspl, iScr) as c_uint); 
  PixMap_D_Ch1 = xlib::XCreatePixmap(pDspl, Wnd_root, ScrWidth as c_uint, ScrHeight as c_uint, xlib::XDefaultDepth(pDspl, iScr) as c_uint); 
  PixMap_A_Ch0 = xlib::XCreatePixmap(pDspl, Wnd_root, ScrWidth as c_uint, ScrHeight as c_uint, xlib::XDefaultDepth(pDspl, iScr) as c_uint); 
  for i in 0.. NUM_OF_A_CHS
  {
    arr_PixMap_A_Chs0[i] = xlib::XCreatePixmap(pDspl, Wnd_root, ScrWidth as c_uint, ScrHeight as c_uint, xlib::XDefaultDepth(pDspl, iScr) as c_uint); 
    arr_PixMap_A_Chs1[i] = xlib::XCreatePixmap(pDspl, Wnd_root, ScrWidth as c_uint, ScrHeight as c_uint, xlib::XDefaultDepth(pDspl, iScr) as c_uint); 
  }

  SetFont();
  
  let mut FontOverall     : xlib::XCharStruct      = mem::MaybeUninit::zeroed().assume_init();
  let mut direction  : c_int = 0;
  let mut yTxt: i32= 0;
  for i in 0.. NUM_OF_D_CHS
  {
    xlib::XTextExtents (pFontInfo, arr_D_Ch_Params[i].Ch.DsplOpt.name.as_ptr() as *mut c_char, arr_D_Ch_Params[i].Ch.DsplOpt.name_len, &mut direction , &mut FontAscent, &mut FontDescent, &mut FontOverall);
    if Ch_D_txt_xx < FontOverall.width as i32 { Ch_D_txt_xx= FontOverall.width as i32; }
    arr_D_Ch_Params[i].Draw.name_xx= FontOverall.width as i32;
    yTxt= yTxt+ DY_TXT+ FontAscent; 
    arr_D_Ch_Params[i].Draw.txt_y= yTxt;
    yTxt= yTxt+ DY_TXT+ FontDescent;
  }
  if Ch_D_yy < FontAscent+ FontDescent { Ch_D_yy= FontAscent+ FontDescent; }

  Ch_name_xx= Ch_D_txt_xx;

  for i in 0.. NUM_OF_A_CHS
  {
    let mut ValStr: String= format!("{}", arr_A_Ch_Params[i].Ch.min);
    xlib::XTextExtents (pFontInfo, ValStr.as_ptr() as *mut c_char, ValStr.chars().count() as i32, &mut direction , &mut FontAscent, &mut FontDescent, &mut FontOverall);
    if ChScale_xx < FontOverall.width as i32 { ChScale_xx= FontOverall.width as i32; }
    ValStr= format!("{}", arr_A_Ch_Params[i].Ch.max);
    xlib::XTextExtents (pFontInfo, ValStr.as_ptr() as *mut c_char, ValStr.chars().count() as i32, &mut direction , &mut FontAscent, &mut FontDescent, &mut FontOverall);
    if ChScale_xx < FontOverall.width as i32 { ChScale_xx= FontOverall.width as i32; }
  }
  ChScale_xx= ChScale_xx+ DX_TXT+ DX_TXT;
  TimeScale_yy= FontAscent+ FontDescent+ DY_TXT+ DY_TXT;

  let ValStr: CString= CString::new(format!("{}", "00:00:00")).expect("CString::new failed");
  xlib::XTextExtents (pFontInfo, ValStr.into_raw(), 8, &mut direction , &mut FontAscent, &mut FontDescent, &mut FontOverall);
  TimeStampVal_xx= FontOverall.width as i32;
  let ValStr: CString= CString::new(format!("{}", "00:00:00:000")).expect("CString::new failed");
  xlib::XTextExtents (pFontInfo, ValStr.into_raw(), 12, &mut direction , &mut FontAscent, &mut FontDescent, &mut FontOverall);
  TimeStampExVal_xx= FontOverall.width as i32;
}
}

pub fn InitDraw()
{
unsafe
{
  DrawArea_w= Width- DrawArea_xLeft;
  DrawArea_yTop= TopBar_yy+ DY_TXT; 
  DrawArea_h= Height- DrawArea_yTop- BottomBar_yy;
  InitDrawParams();
}
}

pub fn InitDrawParams()
{
unsafe
{
  ChScale_xLeft= Width- ChScale_xx* 4- DX_TXT- DX_TXT;
  Graph_x0= ChScale_xLeft- DX_TXT;
  Graph_xx= Graph_x0- DrawArea_xLeft;
  if Graph_xx > 1000 { GraphShift_max= Graph_xx- 1000; }
  else { GraphShift_max= 0; }
  DBG_MSG!("Width: {} {} {}  Height: {} {}   Graph_x0: {}", Width, DrawArea_w, Graph_xx,  Height, DrawArea_h, Graph_x0);

  Graph_D_Ch_yTop= DrawArea_yTop+ DY_TXT;
  Graph_D_Ch_yy= (DY_TXT+ DY_TXT+ Ch_D_yy)* NUM_OF_D_CHS as i32; 
  if bttn_Show_D_Chs.Checked.load(Ordering::Relaxed) == 1
  {
    Graph_A_Ch_yTop= Graph_D_Ch_yTop+ Graph_D_Ch_yy+ DY_TXT+ DY_TXT;
  }
  else
  {
    Graph_A_Ch_yTop= DrawArea_yTop+ FontAscent+ FontDescent;
  }
  Graph_A_Ch_y0= DrawArea_h- TimeScale_yy;
  Graph_A_Ch_yy= Graph_A_Ch_y0- Graph_A_Ch_yTop;

  DBG_MSG!("Graph_A_Ch_y0: {}   Graph_A_Ch_yTop: {}   Graph_A_Ch_yy: {}   ({})", Graph_A_Ch_y0, Graph_A_Ch_yTop, Graph_A_Ch_yy, Graph_A_Ch_yTop+ Graph_A_Ch_yy);

  for i in 0.. NUM_OF_A_CHS
  {
    arr_A_Ch_Params[i].Draw.Scale= Graph_A_Ch_yy as f32 / (arr_A_Ch_Params[i].Ch.max- arr_A_Ch_Params[i].Ch.min) as f32;
  }
}
}

pub fn DrawGrid(DataSweep:DATA_SWEEP_TYPE)
{
unsafe
{
  xlib::XSetForeground(pDspl, pGC, GRAPH_BCK_COLOR); 
  xlib::XFillRectangle(pDspl, PixMap_Grid, pGC,  0, 0, Width as c_uint, Height as c_uint);
  xlib::XSetForeground(pDspl, pGC, 0); 
  xlib::XSetForeground(pDspl, pGC, GRID_LINE_COLOR); 
  xlib::XSetLineAttributes(pDspl, pGC, 1, xlib::LineOnOffDash, xlib::CapButt, xlib::JoinBevel);

  match DataSweep
  {
    DATA_SWEEP_TYPE::_1_sec   => { TimeStamp_dx= 100; },
    DATA_SWEEP_TYPE::_10_sec  => { TimeStamp_dx= 100; },
    DATA_SWEEP_TYPE::_1_min   => { TimeStamp_dx= 60; },
    DATA_SWEEP_TYPE::_10_min  => { TimeStamp_dx= 100; },
    DATA_SWEEP_TYPE::_1_h     => { TimeStamp_dx= 100; },
  _ =>  { }
  }
  let NumOfLines= Graph_xx/ TimeStamp_dx;
  let mut x= Graph_x0;
  for _i in 0.. NumOfLines+ 1
  {
    xlib::XDrawLine(pDspl, PixMap_Grid, pGC, x, DrawArea_yTop+ 1, x, Graph_A_Ch_y0); 
    x= x- TimeStamp_dx;
  }

  if bttn_Show_D_Chs.Checked.load(Ordering::Relaxed) == 1
  {
    xlib::XSetLineAttributes(pDspl, pGC, 1, xlib::LineOnOffDash, xlib::CapButt, xlib::JoinBevel);
    xlib::XSetForeground(pDspl, pGC, GRID_LINE_COLOR); 
    let mut y= Graph_D_Ch_yTop; // + DY_TXT+ 1+ 4* 2);
    let dy= DY_TXT+ DY_TXT+ Ch_D_yy;
    xlib::XDrawLine(pDspl, PixMap_Grid, pGC, DX_TXT, y, Graph_x0, y); 
    for i in 0.. NUM_OF_D_CHS
    {
      xlib::XSetForeground(pDspl, pGC, arr_D_Ch_Params[i].Ch.DsplOpt.color); 
      xlib::XDrawString(pDspl, PixMap_Grid, pGC, DrawArea_xLeft- DX_TXT- arr_D_Ch_Params[i].Draw.name_xx, Graph_D_Ch_yTop+ arr_D_Ch_Params[i].Draw.txt_y, arr_D_Ch_Params[i].Ch.DsplOpt.name.as_ptr() as *mut c_char, arr_D_Ch_Params[i].Ch.DsplOpt.name_len); 
      y= y+ dy;
      xlib::XSetForeground(pDspl, pGC, GRID_LINE_COLOR); 
      xlib::XDrawLine(pDspl, PixMap_Grid, pGC, DrawArea_xLeft- DX_TXT- DX_TXT- Ch_D_txt_xx, y, Graph_x0, y); 
      arr_D_Ch_Params[i].Draw.y0= y;
     }
  }

  x= ChScale_xLeft; // ChScale_xx;
  let dy= Graph_A_Ch_yy/ 4;
  for i in 0.. 4
  {
    x= x+ ChScale_xx;
    xlib::XSetLineAttributes(pDspl, pGC, 3, xlib::LineSolid, xlib::CapButt, xlib::JoinBevel);
    xlib::XSetForeground(pDspl, pGC, arr_A_Ch_Params[i].Ch.DsplOpt.color); 
    xlib::XDrawLine(pDspl, PixMap_Grid, pGC, x, Graph_A_Ch_yTop, x, Graph_A_Ch_y0); 
    let mut y= Graph_A_Ch_y0;
    let mut FontOverall : xlib::XCharStruct      = mem::MaybeUninit::zeroed().assume_init();
    let mut direction   : c_int = 0;
    let mut ValStr      : String= String::new();
    for j in 0.. 5
    {
      match j
      {
        0 =>
        {
          ValStr= format!("{}", arr_A_Ch_Params[i].Ch.min);
        },
        4 =>
        {
          ValStr= format!("{}", arr_A_Ch_Params[i].Ch.max);
        },
        2 =>
        {
          ValStr= format!("{}", (arr_A_Ch_Params[i].Ch.max+ arr_A_Ch_Params[i].Ch.min) / 2);
        },
        1 =>
        {
          ValStr= format!("{}", arr_A_Ch_Params[i].Ch.min+ (arr_A_Ch_Params[i].Ch.max- arr_A_Ch_Params[i].Ch.min) / 4);
        }
        3 =>
        {
          ValStr= format!("{}", arr_A_Ch_Params[i].Ch.max- (arr_A_Ch_Params[i].Ch.max- arr_A_Ch_Params[i].Ch.min) / 4);
        }
        _ => {}
      }
      xlib::XTextExtents (pFontInfo, ValStr.as_ptr() as *mut c_char, ValStr.chars().count() as i32, &mut direction , &mut FontAscent, &mut FontDescent, &mut FontOverall);
      xlib::XDrawString(pDspl, PixMap_Grid, pGC, x- FontOverall.width as i32- DX_TXT, y- DY_TXT, ValStr.as_ptr() as *mut c_char, ValStr.chars().count() as i32); 
      xlib::XDrawLine(pDspl, PixMap_Grid, pGC, x, y, x- ChScale_xx/ 4, y); 
      y= y- dy;
    }
  }
  xlib::XSetLineAttributes(pDspl, pGC, 1, xlib::LineOnOffDash, xlib::CapButt, xlib::JoinBevel);
  xlib::XSetForeground(pDspl, pGC, GRID_LINE_COLOR); 
  let mut y= Graph_A_Ch_y0;
  let dy= Graph_A_Ch_yy/ 4;
  for _i in 0.. 5
  {
    xlib::XDrawLine(pDspl, PixMap_Grid, pGC, DrawArea_xLeft, y, Graph_x0, y); 
    y= y- dy;
  }

  for i in 0.. NUM_OF_A_CHS
  {
    xlib::XSetForeground(pDspl, pGC, arr_A_Ch_Params[i].Ch.DsplOpt.color); 
    xlib::XDrawString(pDspl, PixMap_Grid, pGC, DX_TXT, Graph_A_Ch_yTop+ arr_A_Ch_Params[i].Draw.txt_y, arr_A_Ch_Params[i].Ch.DsplOpt.name.as_ptr() as *mut c_char, arr_A_Ch_Params[i].Ch.DsplOpt.name_len); 
  }
  xlib::XCopyArea(pDspl, PixMap_Grid, PixMap_Wnd, pGC, 0, 0, Width as c_uint, Height as c_uint, 0, 0); 
}
}

// #[macro_export]
macro_rules! REFRESH 
{
  () =>
  {
//  unsafe
  {
    xlib::XCopyArea(pDspl, PixMap_Wnd, Wnd_main,    pGC, 0             , 0, Width as c_uint, Height as c_uint,              0, 0); 
  }
  };
}

pub(crate) use REFRESH;

// #[macro_export]
macro_rules! FLUSH_VALS
{
  () =>
  {
    SET_START!(Start);
    xlib::XCopyArea(pDspl, PixMap_Grid, PixMap_Wnd, pGC, 0, 0, Width as c_uint, Height as c_uint, 0, 0); 
    if bttn_Show_D_Chs.Checked.load(Ordering::Relaxed) == 1
    { xlib::XCopyArea(pDspl, PixMap_D_Ch0, PixMap_Wnd, pGC_or, DrawArea_xLeft, Graph_D_Ch_yTop, Graph_xx as c_uint, Graph_D_Ch_yy as c_uint, DrawArea_xLeft, Graph_D_Ch_yTop); }
    for i in 0.. NUM_OF_A_CHS
    {
      if arr_A_Ch_Params[i].Ch.visible == 1 
      { xlib::XCopyArea(pDspl, arr_PixMap_A_Chs0[i], PixMap_Wnd, pGC_or, DrawArea_xLeft, Graph_A_Ch_yTop, Graph_xx as c_uint, Graph_A_Ch_yy as c_uint, DrawArea_xLeft, Graph_A_Ch_yTop); }
    }
    PRINT_DURATION!(Start, "fd", cnt_Vals);
  };
}

// #[macro_export]
macro_rules! COPY_0_1
{
  () =>
  {
    xlib::XCopyArea(pDspl, PixMap_D_Ch0, PixMap_D_Ch1, pGC, DrawArea_xLeft, Graph_D_Ch_yTop, Graph_xx as c_uint, Graph_D_Ch_yy as c_uint, DrawArea_xLeft, Graph_D_Ch_yTop); 
    for i in 0.. NUM_OF_A_CHS
    {
      xlib::XCopyArea(pDspl, arr_PixMap_A_Chs0[i], arr_PixMap_A_Chs1[i], pGC, DrawArea_xLeft, Graph_A_Ch_yTop, Graph_xx as c_uint, Graph_A_Ch_yy as c_uint, DrawArea_xLeft, Graph_A_Ch_yTop); 
    }
  };
}

// #[macro_export]
macro_rules! COPY_1_0
{
  ($PointsPerStep: ident) =>
  {
    xlib::XCopyArea(pDspl, PixMap_D_Ch1, PixMap_D_Ch0, pGC, DrawArea_xLeft+ $PointsPerStep, Graph_D_Ch_yTop, Graph_xx as c_uint, Graph_D_Ch_yy as c_uint,  DrawArea_xLeft, Graph_D_Ch_yTop);
    for i in 0.. NUM_OF_A_CHS
    {
      xlib::XCopyArea(pDspl, arr_PixMap_A_Chs1[i], arr_PixMap_A_Chs0[i], pGC, DrawArea_xLeft+ $PointsPerStep, Graph_A_Ch_yTop, Graph_xx as c_uint, Graph_A_Ch_yy as c_uint,  DrawArea_xLeft, Graph_A_Ch_yTop);
    }
  };
}

// #[macro_export]
macro_rules! DRAW_D_CH_VALS
{
  ($arr_Packets: ident, $PointsPerStep: ident, $NumOfPoints: ident) =>
  {
    for i in 0.. NUM_OF_D_CHS
    {
      let mut Point_x= Graph_x0;
      if ($PointsPerStep > 0) { Point_x-= ($PointsPerStep- 1); }
      xlib::XSetForeground(pDspl, pGC, arr_D_Ch_Params[i].Ch.DsplOpt.color); 
      for j in 0.. $NumOfPoints
      {
        let mut y: i32= ($arr_Packets[j].D_Chs[i] as i32)^ arr_D_Ch_Params[i].Ch.invert;
        if y != 0 { y= Ch_D_yy; }
        y= arr_D_Ch_Params[i].Draw.y0- y;
        xlib::XDrawPoint(pDspl, PixMap_D_Ch0, pGC, Point_x, y); 
        xlib::XDrawPoint(pDspl, PixMap_D_Ch0, pGC, Point_x, y- 1); 
        if (arr_D_Ch_Params[i].Draw.y_prev != 0) && (arr_D_Ch_Params[i].Draw.y_prev != y)
        {
          xlib::XDrawLine(pDspl, PixMap_D_Ch0, pGC, Point_x, arr_D_Ch_Params[i].Draw.y_prev, Point_x, y); 
        }
        arr_D_Ch_Params[i].Draw.y_prev= y;
        if ($PointsPerStep > 0)
          Point_x= Point_x+ 1;
      }
    }
  };
}

// #[macro_export]
macro_rules! DRAW_A_CH_VALS
{
  () =>
  {
    SET_START!(Start);    
    xlib::XSetLineAttributes(pDspl, pGC, 1, xlib::LineSolid, xlib::CapButt, xlib::JoinBevel);
    for i in 0.. NUM_OF_A_CHS
    {

      xlib::XSetForeground(pDspl, pGC, 0); 
      xlib::XFillRectangle(pDspl, arr_PixMap_A_Chs0[i], pGC, 0, 0, Width as c_uint, Height as c_uint);
      xlib::XFillRectangle(pDspl, arr_PixMap_A_Chs1[i], pGC, 0, 0, Width as c_uint, Height as c_uint);
      xlib::XSetForeground(pDspl, pGC, arr_A_Ch_Params[i].Ch.DsplOpt.color); 

      let mut Point_x= Graph_x0;
      let mut y_prev: i32= -1;
      let mut iVal: i32= iVal0 as i32;
      for _j in 0.. cnt_Vals
      { 
        if arr_Vals_ADC[iVal as usize][i] != -1
        {
          if SweepComboBox.List.DataSweepType.load(Ordering::Relaxed) == DATA_SWEEP_TYPE::_1_sec as i32
          {
            let tmp= arr_Vals[iVal as usize][i]* arr_A_Ch_Params[i].Draw.Scale; 
            let y: i32= Graph_A_Ch_y0- Graph_A_Ch_yy / 2 - tmp as i32;

            xlib::XDrawPoint(pDspl, arr_PixMap_A_Chs0[i], pGC, Point_x, y); 
            if y_prev != -1
            { xlib::XDrawLine(pDspl, arr_PixMap_A_Chs0[i], pGC, Point_x, y, Point_x- 1, y_prev); }
            y_prev= y;
          }
          else
          {
            let tmp= arr_Vals[iVal as usize][i]* arr_A_Ch_Params[i].Draw.Scale; 
            let tmp_1= arr_Vals_1[iVal as usize][i]* arr_A_Ch_Params[i].Draw.Scale; 
            let y: i32= Graph_A_Ch_y0- Graph_A_Ch_yy / 2 - tmp as i32;
            let y1: i32= Graph_A_Ch_y0- Graph_A_Ch_yy / 2 - tmp_1 as i32;
      
            if y == y1 
            { xlib::XDrawPoint(pDspl, arr_PixMap_A_Chs0[i], pGC, Point_x, y); }
            else 
            { xlib::XDrawLine(pDspl, arr_PixMap_A_Chs0[i], pGC, Point_x, y, Point_x, y1); }
          }
        }
        else { y_prev= -1; }
        iVal-= 1;
        if iVal < 0 { iVal= (NUM_OF_DRAW_VALS- 1) as i32; }
        Point_x= Point_x- 1;
        if Point_x < DrawArea_xLeft { break; }
      } // while j < NumOfPoints
      xlib::XCopyArea(pDspl, arr_PixMap_A_Chs0[i], arr_PixMap_A_Chs1[1], pGC, DrawArea_xLeft, Graph_A_Ch_yTop, Graph_xx as c_uint, Graph_A_Ch_yy as c_uint, DrawArea_xLeft, Graph_A_Ch_yTop); 
    } // for i in 0.. NUM_OF_A_CHS
    COPY_0_1!();

    PRINT_DURATION!(Start, "dvd", cnt_Vals);
  };
}

// #[macro_export]
macro_rules! REDRAW
{
  () =>
  {
    xlib::XCopyArea(pDspl, PixMap_Grid, PixMap_Wnd, pGC, 0, 0, Width as c_uint, Height as c_uint, 0, 0); 
    if bttn_Show_D_Chs.Checked.load(Ordering::Relaxed) == 1
    { xlib::XCopyArea(pDspl, PixMap_D_Ch0, PixMap_Wnd, pGC_or, DrawArea_xLeft, Graph_D_Ch_yTop, Graph_xx as c_uint, Graph_D_Ch_yy as c_uint, DrawArea_xLeft, 0); }
    DRAW_A_CH_VALS!();
  };
}

// #[macro_export]
macro_rules! DEC_SEC 
{
  ( $h:expr, $m: expr, $s: expr ) =>
  {
    $s= $s- 1;
    if $s < 0 { $m= $m -1; $s= 59; }
    if $m < 0 { $m= 59; $h= $h- 1; }
    if $h < 0 { $h= 23; $m= 59; $s= 59; }
  };
}

// #[macro_export]
macro_rules! DRAW_TIMESCALE 
{
  ( ) =>
  {
    let x= Graph_x0- TimeStampVal_xx/ 2;
    let y= Graph_A_Ch_y0+ DY_TXT+ FontAscent;
    xlib::XSetForeground(pDspl, pGC, GRID_LINE_COLOR); 
    xlib::XDrawString(pDspl, PixMap_Wnd, pGC, x, y, TimeStr0.as_ptr() as *mut c_char, TimeStr0.chars().count() as i32); 
    if Graph_xx > TimeStampVal_dx
    {
      xlib::XDrawString(pDspl, PixMap_Wnd, pGC, x- TimeStampVal_dx, y, TimeStr1.as_ptr() as *mut c_char, TimeStr1.chars().count() as i32); 
    }
  };
  ($h:expr, $m: expr, $s: expr, $SweepType: expr) =>
  {
    Hour0 = $h;
    Min0  = $m;
    Sec0  = $s;
    Hour1 = $h;
    Min1  = $m;
    Sec1  = $s;

    match $SweepType
    {
      DATA_SWEEP_TYPE::_1_sec =>
      {
        DEC_SEC!(Hour1, Min1, Sec1); 
        TimeStampVal_dx= 1000;
      }
      DATA_SWEEP_TYPE::_10_sec  => 
      { 
        Sec1= Sec0- 10;
        if Sec1 < 0 { Min1= Min1 -1; Sec1= 60- Sec1; }
        if Min1 < 0 { Min1= 59; Hour1= Hour1- 1; }
        if Hour1 < 0 { Hour1= 23; Min1= 59; Sec1= 59; }
        TimeStampVal_dx= 1000;
      },
      DATA_SWEEP_TYPE::_1_min   => 
      { 
        Min1= Min0- 1;
//      if Sec1 < 0 { Min1= Min1 -1; Sec1= 60- Sec1; }
        if Min1 < 0 { Min1= 59; Hour1= Hour1- 1; }
        if Hour1 < 0 { Hour1= 23;  /* Sec1= 59; */ }
        TimeStampVal_dx= 600;
      },
      DATA_SWEEP_TYPE::_10_min  => 
      { 
        Min1= Min0- 10;
        if Min1 < 0 { Min1= 60+ Min1; Hour1= Hour1- 1; }
        if Hour1 < 0 { Hour1= 23; }
        TimeStampVal_dx= 600;
      },
      DATA_SWEEP_TYPE::_1_h     => 
      { 
        Hour1= Hour0- 1;
        if Hour1 < 0 { Hour1= 23; }
        TimeStampVal_dx= 600;
      },
/*
    DATA_SWEEP_TYPE::_3_h     => { },
    DATA_SWEEP_TYPE::_12_h    => { },
*/
      _ =>  { }
    }
    Hour2 = Hour1;
    Min2  = Min1;
    Sec2  = Sec1;
    DEC_SEC!(Hour2, Min2, Sec2);

    TimeStr0= format!("{:0>2}:{:0>2}:{:0>2}", Hour0, Min0, Sec0);
    TimeStr1= format!("{:0>2}:{:0>2}:{:0>2}", Hour1, Min1, Sec1);
    DRAW_TIMESCALE!();
  };
}

// #[macro_export]
macro_rules! SHOW_VALS 
{
  ( $x:expr ) =>
  {
    if $x <= Graph_x0 && $x >= DrawArea_xLeft && bttn_Pause.Checked.load(Ordering::Relaxed) == 1
    { 
      REFRESH!();
      xlib::XSetLineAttributes(pDspl, pGC, 1, xlib::LineSolid, xlib::CapButt, xlib::JoinBevel);
      xlib::XSetForeground(pDspl, pGC, GRID_LINE_COLOR); 
      xlib::XDrawLine(pDspl, Wnd_main, pGC, $x, Graph_D_Ch_yTop, $x, Graph_A_Ch_y0);

      let mut iVal= Graph_x0- $x;
        let TimeStr;
        if iVal == 0
        { TimeStr= format!("{:0>2}:{:0>2}:{:0>2}:000", Hour0, Min0, Sec0); }
        else
        if iVal == TimeStampVal_dx
        { TimeStr= format!("{:0>2}:{:0>2}:{:0>2}:000", Hour1, Min1, Sec1); }
        else
        if iVal < TimeStampVal_dx
        { TimeStr= format!("{:0>2}:{:0>2}:{:0>2}:{:0>3}", Hour1, Min1, Sec1, 1000- iVal); }
        else  //        if iVal < TimeStampVal_dx
        { 
          TimeStr= format!("{:0>2}:{:0>2}:{:0>2}:{:0>3}", Hour2, Min2, Sec2, 1000- (iVal- 1000)); 
        }
      if iVal < cnt_Vals as i32
      {   
        iVal= iVal0 as i32 - iVal;
        if iVal < 0 { iVal= cnt_Vals as i32 + iVal; }
        DBG_MSG!("iVal0: {}   iVval: {}   cnt_Vals: {}", iVal0, iVal, cnt_Vals);
      
        for i in 0.. NUM_OF_A_CHS
        {
          if arr_A_Ch_Params[i].Ch.visible == 0 { continue; }
          let ValStr; //= String::new();
          if SweepComboBox.List.DataSweepType.load(Ordering::Relaxed) == 0
          { 
            ValStr= format!("{:3.3} / {}", arr_Vals[iVal as usize][i], arr_Vals_ADC[iVal as usize][i]); 
            xlib::XSetForeground(pDspl, pGC, arr_A_Ch_Params[i].Ch.DsplOpt.color); 
            xlib::XDrawString(pDspl, Wnd_main, pGC, DX_TXT+ DX_TXT+ Ch_name_xx, Graph_A_Ch_yTop+ arr_A_Ch_Params[i].Draw.txt_y, ValStr.as_ptr() as *mut c_char, ValStr.chars().count() as i32); 
          }
        }
      } // if iVal < cnt_Vals as i32
      let x0= $x- TimeStampExVal_xx/ 2;
      let y= Graph_A_Ch_y0+ DY_TXT+ FontAscent;
      xlib::XSetForeground(pDspl, pGC, 0x00FFFFF0); // GRID_LINE_COLOR); 
      xlib::XFillRectangle(pDspl, Wnd_main, pGC,  x0- DX_TXT, Graph_A_Ch_y0, (TimeStampExVal_xx+ DX_TXT+ DX_TXT) as c_uint, (DY_TXT+ DY_TXT+ FontAscent+ FontDescent) as c_uint);
      xlib::XSetForeground(pDspl, pGC, 0x000000B0); 
      xlib::XDrawString(pDspl, Wnd_main, pGC, x0, y, TimeStr.as_ptr() as *mut c_char, TimeStr.chars().count() as i32); 
    }
  };
}

fn GetIsPauisedMtx() -> &'static Mutex<i32>  
{
  static mut mData: mem::MaybeUninit<Mutex<i32>> = mem::MaybeUninit::uninit();
  static ONCE: Once = Once::new();

  unsafe 
  {
    ONCE.call_once(|| 
    {
     mData.write(Mutex::new(0));
    }
    );
    mData.assume_init_ref()
  }
}

// #[macro_export]
macro_rules! CHECK_IF_PAUSED 
{
  ( $IsPaused: ident, $i: expr ) =>
  {
    let $IsPaused = GetIsPauisedMtx().lock().unwrap();
//    let $IsPaused = mIsPaused.lock().unwrap();
  };
}

fn ResetDrawData()
{
unsafe
{
  xlib::XSetLineAttributes(pDspl, pGC, 3, xlib::LineSolid, xlib::CapButt, xlib::JoinBevel);
  xlib::XSetForeground(pDspl, pGC, 0); 
  xlib::XFillRectangle(pDspl, PixMap_D_Ch0, pGC, 0, 0, Width as c_uint, Height as c_uint);
  xlib::XFillRectangle(pDspl, PixMap_D_Ch1, pGC, 0, 0, Width as c_uint, Height as c_uint);
  for i in 0.. NUM_OF_A_CHS
  {
    xlib::XFillRectangle(pDspl, arr_PixMap_A_Chs0[i], pGC, 0, 0, Width as c_uint, Height as c_uint);
    xlib::XFillRectangle(pDspl, arr_PixMap_A_Chs1[i], pGC, 0, 0, Width as c_uint, Height as c_uint);
  }
//  xlib::XCopyArea(pDspl, PixMap_Grid, PixMap_Wnd, pGC, 0, 0, Width as c_uint, Height as c_uint, 0, 0); 


  for i in 0.. NUM_OF_D_CHS
  { arr_D_Ch_Params[i].Draw.y_prev= -1; }
  for i in 0.. NUM_OF_A_CHS
  { arr_A_Ch_Params[i].Draw.y_prev= -1; }
  GraphShift= 0;
  cnt_Points= 0;
  cnt_Vals= 0;
  iVal0= 0;
}
}

pub fn ResetDrawDataEx(DataSweep:DATA_SWEEP_TYPE)
{
{ 
  let _IsPaused = GetIsPauisedMtx().lock().unwrap();
  unsafe
  {
  xlib::XLockDisplay(pDspl);
  DrawGrid(DataSweep);
  ResetDrawData();
  xlib::XUnlockDisplay(pDspl);
  }
}
}

static mut cnt_Points  : i32            = 0; 
static mut GraphShift  : i32            = 0;
static mut iValToDspl  : usize          = 0;
static mut TimeStampVal_dx : i32        = 1000;
static mut Hour0        : i32           = 0;
static mut Min0         : i32           = 0;
static mut Sec0         : i32           = 0;
static mut Hour1        : i32           = 0;
static mut Min1         : i32           = 0;
static mut Sec1         : i32           = 0;
static mut Hour2        : i32           = 0;
static mut Min2         : i32           = 0;
static mut Sec2         : i32           = 0;
static mut TimeStr0     : String        = String::new();
static mut TimeStr1     : String        = String::new();

pub const NUM_OF_DRAW_VALS : usize          = 2000; 
pub static mut arr_Vals_ADC : [[i32; NUM_OF_A_CHS]; NUM_OF_DRAW_VALS]   = [[0;  NUM_OF_A_CHS]; NUM_OF_DRAW_VALS];
static mut arr_Vals     : [[f32; NUM_OF_A_CHS]; NUM_OF_DRAW_VALS]   = [[0.; NUM_OF_A_CHS]; NUM_OF_DRAW_VALS];
static mut arr_Vals_1   : [[f32; NUM_OF_A_CHS]; NUM_OF_DRAW_VALS]   = [[0.; NUM_OF_A_CHS]; NUM_OF_DRAW_VALS];
static mut cnt_Vals     : usize                                     = 0;
static mut iVal0        : usize                                     = 0;

pub fn AddPoints(pPackets: *const [DATA_PACKET], NumOfPoints: usize, PointsPerStep: i32)
{
  SET_START!(Start);
unsafe
{
{
//  CHECK_IF_PAUSED!(IsPaused, 1);
  let mut IsPaused = GetIsPauisedMtx().lock().unwrap();
  if bttn_Pause.Checked.load(Ordering::Relaxed) == 1 
  { 
    if *IsPaused < 1 { *IsPaused= 1; }
    drop(IsPaused); 
    return; 
  }
  else
  {
    if *IsPaused != 0
    {
      ResetDrawData();
      *IsPaused= 0;
    }
  }


  let arr_Packets: &[DATA_PACKET]= & *(pPackets as *const [u8] as *const [DATA_PACKET]);  
    
  xlib::XLockDisplay(pDspl);
  
  COPY_1_0!(PointsPerStep);
  xlib::XSetLineAttributes(pDspl, pGC, 1, xlib::LineSolid, xlib::CapButt, xlib::JoinBevel);

  for i in 0.. NUM_OF_D_CHS
  {
    let mut Point_x= Graph_x0;
    if (PointsPerStep > 0) { Point_x-= (PointsPerStep- 1); }
    xlib::XSetForeground(pDspl, pGC, arr_D_Ch_Params[i].Ch.DsplOpt.color); 
    for j in 0.. NumOfPoints
    {
      let mut y: i32= (arr_Packets[j].D_Chs[i] as i32)^ arr_D_Ch_Params[i].Ch.invert;
      if y != 0 { y= Ch_D_yy; }
      y= arr_D_Ch_Params[i].Draw.y0- y;
      xlib::XDrawPoint(pDspl, PixMap_D_Ch0, pGC, Point_x, y); 
      xlib::XDrawPoint(pDspl, PixMap_D_Ch0, pGC, Point_x, y- 1); 
      if (arr_D_Ch_Params[i].Draw.y_prev != 0) && (arr_D_Ch_Params[i].Draw.y_prev != y)
      {
        xlib::XDrawLine(pDspl, PixMap_D_Ch0, pGC, Point_x, arr_D_Ch_Params[i].Draw.y_prev, Point_x, y); 
      }
      arr_D_Ch_Params[i].Draw.y_prev= y;
      Point_x= Point_x+ 1;
    }
  }

  let mut iValToWrite: usize= 0;
  for i in 0.. NUM_OF_A_CHS
  {
    let mut Point_x= Graph_x0;
    if (PointsPerStep > 0) { Point_x-= (PointsPerStep- 1); }
    iValToWrite= iVal0;
    xlib::XSetForeground(pDspl, pGC, arr_A_Ch_Params[i].Ch.DsplOpt.color); 
    for j in 0.. NumOfPoints
    { 
      iValToWrite+= 1;
      if iValToWrite == NUM_OF_DRAW_VALS { iValToWrite= 0; }

      arr_Vals[iValToWrite][i]= ((arr_Packets[j].A_Chs[i] as i32+ arr_A_Ch_Params[i].Ch.b) as f32)* arr_A_Ch_Params[i].Ch.k+ arr_A_Ch_Params[i].Ch.c as f32;
      arr_Vals_ADC[iValToWrite][i]= arr_Packets[j].A_Chs[i] as i32;

      let tmp= arr_Vals[iValToWrite][i]* arr_A_Ch_Params[i].Draw.Scale; 
      let y: i32= Graph_A_Ch_y0- Graph_A_Ch_yy / 2 - tmp as i32;

      xlib::XDrawPoint(pDspl, arr_PixMap_A_Chs0[i], pGC, Point_x, y); 
      if arr_A_Ch_Params[i].Draw.y_prev != -1
      { xlib::XDrawLine(pDspl, arr_PixMap_A_Chs0[i], pGC, Point_x, y, Point_x- 1, arr_A_Ch_Params[i].Draw.y_prev); }

      arr_A_Ch_Params[i].Draw.y_prev= y;
      Point_x= Point_x+ 1;
    } // while j < NumOfPoints
  } // for i in 0.. NUM_OF_A_CHS
  iVal0= iValToWrite;
  cnt_Vals= cnt_Vals+ NumOfPoints;
  if cnt_Vals > NUM_OF_DRAW_VALS { cnt_Vals= NUM_OF_DRAW_VALS; }
  cnt_Points= cnt_Points+ NumOfPoints as i32;
  GraphShift= GraphShift+ NumOfPoints as i32;

  COPY_0_1!();

  xlib::XUnlockDisplay(pDspl);

  drop(IsPaused);
  PRINT_DURATION!(Start, "dd");
}
}
}

pub fn AddMinMaxPoints(D_Ch_Vals: &[[u8; NUM_OF_D_CHS]; NUM_OF_MIN_MAX_VALS], pMinVals: &[[u16; NUM_OF_A_CHS]; NUM_OF_MIN_MAX_VALS], pMaxVals: &[[u16; NUM_OF_A_CHS]; NUM_OF_MIN_MAX_VALS], NumOfPoints: usize, PointsPerStep: i32)
{
  SET_START!(Start);
unsafe
{
{
//  CHECK_IF_PAUSED!(IsPaused, 1);
  let mut IsPaused = GetIsPauisedMtx().lock().unwrap();
  if bttn_Pause.Checked.load(Ordering::Relaxed) == 1 
  { 
    if *IsPaused < 1 { *IsPaused= 1; }
    drop(IsPaused); 
    return; 
  }
  else
  {
    if *IsPaused != 0
    {
      ResetDrawData();
      *IsPaused= 0;
    }
  }

  xlib::XLockDisplay(pDspl);

  if PointsPerStep > 0
  { COPY_1_0!(PointsPerStep); }

  xlib::XSetLineAttributes(pDspl, pGC, 1, xlib::LineSolid, xlib::CapButt, xlib::JoinBevel);
  
  for i in 0.. NUM_OF_D_CHS
  {
    let mut Point_x= Graph_x0;
    if (PointsPerStep > 0) { Point_x-= (PointsPerStep- 1); }
    xlib::XSetForeground(pDspl, pGC, arr_D_Ch_Params[i].Ch.DsplOpt.color); 
    for j in 0.. NumOfPoints
    {
      let mut y: i32= (D_Ch_Vals[j][i] as i32)^ arr_D_Ch_Params[i].Ch.invert;
      if y == 1 { y= Ch_D_yy; }
      y= arr_D_Ch_Params[i].Draw.y0- y;
      if D_Ch_Vals[j][i] != 2
      {
        xlib::XDrawPoint(pDspl, PixMap_D_Ch0, pGC, Point_x, y); 
        xlib::XDrawPoint(pDspl, PixMap_D_Ch0, pGC, Point_x, y- 1); 
        if (arr_D_Ch_Params[i].Draw.y_prev != -1) && (arr_D_Ch_Params[i].Draw.y_prev != y)
        {
          xlib::XDrawLine(pDspl, PixMap_D_Ch0, pGC, Point_x, arr_D_Ch_Params[i].Draw.y0, Point_x, arr_D_Ch_Params[i].Draw.y0- Ch_D_yy); 
        }
      }
      else
      {
        xlib::XDrawLine(pDspl, PixMap_D_Ch0, pGC, Point_x, arr_D_Ch_Params[i].Draw.y0, Point_x, arr_D_Ch_Params[i].Draw.y0- Ch_D_yy); 
      }
      arr_D_Ch_Params[i].Draw.y_prev= y;
      Point_x= Point_x+ 1;
    }
  }
  let mut iValToWrite: usize= 0;
  for i in 0.. NUM_OF_A_CHS
  {
    let mut Point_x= Graph_x0;
    if (PointsPerStep > 0) { Point_x-= (PointsPerStep- 1); }
    iValToWrite= iVal0;
    xlib::XSetForeground(pDspl, pGC, arr_A_Ch_Params[i].Ch.DsplOpt.color); 
    for j in 0.. NumOfPoints
    { 
      if PointsPerStep > 0 
      { 
        iValToWrite+= 1; 
        if iValToWrite == NUM_OF_DRAW_VALS { iValToWrite= 0; }
      }
      arr_Vals[iValToWrite][i]=   ((pMaxVals[j][i] as i32+ arr_A_Ch_Params[i].Ch.b) as f32)* arr_A_Ch_Params[i].Ch.k+ arr_A_Ch_Params[i].Ch.c as f32;
      arr_Vals_1[iValToWrite][i]= ((pMinVals[j][i] as i32+ arr_A_Ch_Params[i].Ch.b) as f32)* arr_A_Ch_Params[i].Ch.k+ arr_A_Ch_Params[i].Ch.c as f32;

      let tmp= arr_Vals[iValToWrite][i]* arr_A_Ch_Params[i].Draw.Scale; 
      let tmp_1= arr_Vals_1[iValToWrite][i]* arr_A_Ch_Params[i].Draw.Scale; 
      let y: i32= Graph_A_Ch_y0- Graph_A_Ch_yy / 2 - tmp as i32;
      let y1: i32= Graph_A_Ch_y0- Graph_A_Ch_yy / 2 - tmp_1 as i32;
      
      if y == y1 
      { xlib::XDrawPoint(pDspl, arr_PixMap_A_Chs0[i], pGC, Point_x, y); }
      else 
      { xlib::XDrawLine(pDspl, arr_PixMap_A_Chs0[i], pGC, Point_x, y, Point_x, y1); }

      arr_A_Ch_Params[i].Draw.y_prev= y;
      Point_x= Point_x+ 1;
    } // while j < NumOfPoints
  } // for i in 0.. NUM_OF_A_CHS
  iVal0= iValToWrite;
  if PointsPerStep > 0 
  {
    cnt_Vals= cnt_Vals+ NumOfPoints;
    if cnt_Vals > NUM_OF_DRAW_VALS { cnt_Vals= NUM_OF_DRAW_VALS; }
  }
  cnt_Points= cnt_Points+ NumOfPoints as i32;
  GraphShift= GraphShift+ NumOfPoints as i32;

  COPY_0_1!();
  xlib::XUnlockDisplay(pDspl);

  drop(IsPaused);
  PRINT_DURATION!(Start, "dd");
}
}
}

pub fn FlushPoints(NumOfPoints: i32, SweepType: DATA_SWEEP_TYPE)
{
  let now= SystemTime::now();

 unsafe
{
{
//  CHECK_IF_PAUSED!(IsPaused, 2);
  let mut IsPaused = GetIsPauisedMtx().lock().unwrap();
  if bttn_Pause.Checked.load(Ordering::Relaxed) == 1 
  { 
    if *IsPaused == 2 { drop(IsPaused); DBG_MSG!("2 unlock 2"); return; }
    *IsPaused= 2;   
  }
  else
  {
    if *IsPaused != 0
    {
      ResetDrawData();
      *IsPaused= 0;
      drop(IsPaused); 
      DBG_MSG!("2 unlock 2"); 
      return;
    }
  }


  DBG_MSG!("FlushPoints {}  ({})", cnt_Points, NumOfPoints);

  xlib::XLockDisplay(pDspl);

  if NumOfPoints > 0 && cnt_Points < NumOfPoints && *IsPaused != 2
  {   
    xlib::XSetForeground(pDspl, pGC, 0); 
    let xx= NumOfPoints- cnt_Points; // / 50)* 50;
    if xx > 50
    {
      let mut iValToWrite= iVal0;
      for _j in 0.. xx
      {
        if cnt_Vals < NUM_OF_DRAW_VALS { cnt_Vals+= 1; }
        iValToWrite+= 1;
        for i in 0.. NUM_OF_A_CHS
        { 
          if iValToWrite == NUM_OF_DRAW_VALS { iValToWrite= 0; }
          arr_Vals[iValToWrite][i]= 0.; // arr_A_Ch_Params[i].Ch.max as f32;
          arr_Vals_ADC[iValToWrite][i]= -1;
        } // while j < NumOfPoints
      } // for i in 0.. NUM_OF_A_CHS
      iVal0= iValToWrite;

      xlib::XFillRectangle(pDspl, PixMap_D_Ch0, pGC, 0, 0, Width as c_uint, Height as c_uint);
      xlib::XCopyArea(pDspl, PixMap_D_Ch1, PixMap_D_Ch0, pGC, DrawArea_xLeft+ xx, Graph_D_Ch_yTop, (Graph_xx- xx) as c_uint, Graph_D_Ch_yy as c_uint, DrawArea_xLeft, Graph_D_Ch_yTop);
      xlib::XCopyArea(pDspl, PixMap_D_Ch0, PixMap_D_Ch1, pGC, DrawArea_xLeft, Graph_D_Ch_yTop, Graph_xx as c_uint, Graph_D_Ch_yy as c_uint, DrawArea_xLeft, Graph_D_Ch_yTop); 
      for i in 0.. NUM_OF_A_CHS
      {
        xlib::XFillRectangle(pDspl, arr_PixMap_A_Chs0[i], pGC, 0, 0, Width as c_uint, Height as c_uint);
        xlib::XCopyArea(pDspl,      arr_PixMap_A_Chs1[i], arr_PixMap_A_Chs0[i], pGC, DrawArea_xLeft+ xx, Graph_A_Ch_yTop, (Graph_xx- xx) as c_uint, Graph_A_Ch_yy as c_uint, DrawArea_xLeft, Graph_A_Ch_yTop);
        xlib::XCopyArea(pDspl,      arr_PixMap_A_Chs0[i], arr_PixMap_A_Chs1[i], pGC, DrawArea_xLeft, Graph_A_Ch_yTop, Graph_xx as c_uint, Graph_A_Ch_yy as c_uint, DrawArea_xLeft, Graph_A_Ch_yTop); 
      }
      GraphShift= GraphShift+ xx; 
      for i in 0.. NUM_OF_D_CHS
      { arr_D_Ch_Params[i].Draw.y_prev= -1; }
      for i in 0.. NUM_OF_A_CHS
      { arr_A_Ch_Params[i].Draw.y_prev= -1; }
    }
  }
  
  cnt_Points= 0; 
   
  FLUSH_VALS!();

  let TimeStamp: chrono::DateTime<Utc> = now.into();
  DRAW_TIMESCALE!(TimeStamp.hour() as i32, TimeStamp.minute() as i32, TimeStamp.second() as i32, SweepType);

  REFRESH!();
  xlib::XFlush(pDspl);
  xlib::XUnlockDisplay (pDspl);

  if GraphShift >= Graph_xx { GraphShift= Graph_xx- GraphShift_max; }
/*
  if GraphShift >= Graph_xx
  {
    if GraphShift_max != 0
    {
      xlib::XSetForeground(pDspl, pGC, 0); 
      xlib::XFillRectangle(pDspl, PixMap_D_Ch1, pGC, DrawArea_xLeft, 0, GraphShift_max as c_uint, Height as c_uint);
      for i in 0.. NUM_OF_A_CHS
      {
        xlib::XFillRectangle(pDspl, arr_PixMap_A_Chs1[i], pGC, DrawArea_xLeft, 0, GraphShift_max as c_uint, Height as c_uint);
      }
    }
    GraphShift= Graph_xx- GraphShift_max;
  }
*/
  drop(IsPaused);
}
}
  PRINT_DURATION!(now, "fd", NumOfPoints);
}

pub fn wnd_evnt_loop()
{
unsafe 
{
  static mut ShowVals: i32= 0;
  let mut Evnt: xlib::XEvent = mem::MaybeUninit::zeroed().assume_init();
  let mut x= 0;
  loop 
  {
    xlib::XNextEvent(pDspl, &mut Evnt);
    xlib::XLockDisplay(pDspl);
    match Evnt.type_ //event.get_type()
    {
      xlib::DestroyNotify =>
      {
        println!("DestroyNotify");
      },

      xlib::Expose =>
      { 
        let eData: xlib::XExposeEvent= Evnt.expose;
        if eData.window == Wnd_main { REFRESH!(); } 
        else
        if eData.window == bttn_Pause.Cntrl.Wnd { bttn_Pause.Cntrl.Refresh(); }
        else
        if eData.window == SweepComboBox.Cntrl.Wnd { SweepComboBox.Cntrl.Refresh(); }  
        else
        if eData.window == SweepComboBox.List.Cntrl.Wnd { SweepComboBox.List.Refresh(); }  
        else
        if eData.window == bttn_Show_D_Chs.Cntrl.Wnd { bttn_Show_D_Chs.Cntrl.Refresh(); }
      },

      xlib::ConfigureNotify => 
      { 
        let eData: xlib::XConfigureEvent  = Evnt.configure;
//        if eData.send_event == xlib::True
        if   (Width != eData.width && eData.width >= w_min) 
          || (Height != eData.height && eData.height >= h_min)
        {
          if eData.window == Wnd_main
          {
            Width= eData.width;
            Height= eData.height;
            xlib::XSync(pDspl, xlib::True);
            InitDraw();
            DrawGrid(mem::transmute(SweepComboBox.List.DataSweepType.load(Ordering::Relaxed)));
            xlib::XFlushGC(pDspl, pGC);
            let yy= FontAscent+ FontDescent+ DY_TXT+ DY_TXT+ 3+ 3+ DY_TXT;
            let mut y= Height- yy;
            xlib::XMoveWindow(pDspl, SweepComboBox.Cntrl.Wnd, 1, y);
            xlib::XMoveWindow(pDspl, SweepComboBox.List.Cntrl.Wnd, 1, y- SweepComboBox.List.Cntrl.h- 1);
            y= y- yy;
            xlib::XMoveWindow(pDspl, bttn_Pause.Cntrl.Wnd, 1, y);
            y= y- yy;
            xlib::XMoveWindow(pDspl, bttn_Show_D_Chs.Cntrl.Wnd, 1, y);
            xlib::XFlush(pDspl);
            if bttn_Pause.Checked.load(Ordering::Relaxed) == 1 
            {
              FLUSH_VALS!();
              DRAW_TIMESCALE!();
              if ShowVals == 2 
              { SHOW_VALS!(x); }
            }
            else
            {
              DRAW_A_CH_VALS!();
              FLUSH_VALS!();
            }
            REFRESH!();  
          }
        }
      },

      xlib::MotionNotify =>
      { 
        let eData: xlib::XMotionEvent = Evnt.motion;
        if eData.window == Wnd_main 
        { 
          x= eData.x;  
          if ShowVals == 1 
          { SHOW_VALS!(x); }
        }
        else
        if eData.window == SweepComboBox.List.Cntrl.Wnd { SweepComboBox.List.OnMouseOver(eData.y); }
      },

      xlib::LeaveNotify =>
      {
        let eData: xlib::XCrossingEvent = Evnt.crossing; 
        if eData.window == Wnd_main { x= 0; }
        else
        if eData.window == SweepComboBox.Cntrl.Wnd { SweepComboBox.OnMouseLeave(); }
        else
        if eData.window == SweepComboBox.List.Cntrl.Wnd { SweepComboBox.List.OnMouseLeave(); }
        else
        if eData.window == bttn_Show_D_Chs.Cntrl.Wnd { bttn_Show_D_Chs.OnMouseLeave(); }
      },

      xlib::ButtonPress =>
      {
        let eData: xlib::XButtonEvent = Evnt.button; 
        if eData.window == Wnd_main 
        { 
          if bttn_Pause.Checked.load(Ordering::Relaxed) == 1 { ShowVals= 1; SHOW_VALS!(x); } 
        }
        else
        if eData.window == SweepComboBox.Cntrl.Wnd 
        { 
          if bttn_Pause.Checked.load(Ordering::Relaxed) == 0
          { SweepComboBox.OnBttnPress(); }
        }
        else
        if eData.window == SweepComboBox.List.Cntrl.Wnd { SweepComboBox.List.OnBttnPress(); }
        else
        if eData.window == bttn_Show_D_Chs.Cntrl.Wnd { bttn_Show_D_Chs.OnBttnPress(); }
        else
        if eData.window == bttn_Pause.Cntrl.Wnd { bttn_Pause.OnBttnPress(); }
      },

      xlib::ButtonRelease =>
      {
        let eData: xlib::XButtonEvent = Evnt.button; 
        if eData.window == Wnd_main { if ShowVals == 1 { ShowVals= 2; } else { ShowVals= 0; } }
        else
        if eData.window == SweepComboBox.Cntrl.Wnd { SweepComboBox.OnBttnRelease(); }
        else
        if eData.window == SweepComboBox.List.Cntrl.Wnd 
        { 
          let iDataSweep : i32= SweepComboBox.List.OnBttnRelease();
          if iDataSweep >= DATA_SWEEP_TYPE::_1_sec as i32 && iDataSweep <= DATA_SWEEP_TYPE::_1_h as i32 
          {
          {
            (*pGUIEvntSender).send(iDataSweep).unwrap(); 
          }
          }
          SweepComboBox.ChangeSel(iDataSweep);
        }
        else
        if eData.window == bttn_Show_D_Chs.Cntrl.Wnd 
        { 
          if bttn_Show_D_Chs.OnBttnRelease()
          {
          {
            let _IsPaused = GetIsPauisedMtx().lock().unwrap();
            xlib::XSync(pDspl, xlib::True);
            InitDraw();
            DrawGrid(mem::transmute(SweepComboBox.List.DataSweepType.load(Ordering::Relaxed)));
            DRAW_A_CH_VALS!();
            FLUSH_VALS!();
            xlib::XFlushGC(pDspl, pGC);
            xlib::XFlushGC(pDspl, pGC_or);
            xlib::XSync(pDspl, xlib::False);
            REFRESH!();
            if bttn_Pause.Checked.load(Ordering::Relaxed) == 1 
            {
              DRAW_TIMESCALE!();
              if ShowVals == 2 
              { SHOW_VALS!(x); }
              REFRESH!();
            }
            else
            {
//              ResetDrawData();
            }
          }              
          }
        }
        else
        if eData.window == bttn_Pause.Cntrl.Wnd 
        { 
          if bttn_Pause.OnBttnRelease()
          {
            if bttn_Pause.Checked.load(Ordering::Relaxed) == 0 
            { ShowVals= 0; }
            else {  }
          }
        }
      },



      _ => {  },

    }
    xlib::XUnlockDisplay(pDspl);
  }
}
}