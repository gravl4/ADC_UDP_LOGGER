#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#![cfg_attr(not(feature = "xlib"), allow(dead_code))]
#![cfg_attr(not(feature = "xlib"), allow(unused_imports))]

extern crate x11;
use x11::xlib;

use std::mem;
use std::ptr;
use std::os::raw::*;

pub static mut FontFamily : &'static str = "courier";
pub static mut FontLen    : i32          = 18;
pub static mut FontWeight : &'static str = "*";

pub static mut iScr     : c_int              = 0;
pub static mut pDspl    : *mut xlib::Display = ptr::null_mut();
pub static mut Wnd_root : c_ulong            = 0;
pub static mut Wnd_main : c_ulong            = 0;
pub static mut pGC      : xlib::GC           = ptr::null_mut(); 
pub static mut pGC_xor : xlib::GC           = ptr::null_mut(); 
pub static mut pGC_or  : xlib::GC           = ptr::null_mut(); 
pub static mut gcVals   : xlib::XGCValues    = xlib::XGCValues 
{ 
//  function:           /* c_int:   */ xlib::GXand, 
  function:           /* c_int:   */ xlib::GXcopy,
  plane_mask:         /* c_ulong: */ 0,
  foreground:         /* c_ulong: */ 0,
  background:         /* c_ulong: */ 0,
  line_width:         /* c_int:   */ 0,
  line_style:         /* c_int:   */ 0,
  cap_style:          /* c_int:   */ 0,
  join_style:         /* c_int:   */ 0,
  fill_style:         /* c_int:   */ 0,
  fill_rule:          /* c_int:   */ 0,
  arc_mode:           /* c_int:   */ 0,
  tile:               /* Pixmap:  */ 0,
  stipple:            /* Pixmap:  */ 0,
  ts_x_origin:        /* c_int:   */ 0,
  ts_y_origin:        /* c_int:   */ 0,
  font:               /* Font:    */ 0,
  subwindow_mode:     /* c_int:   */ 0,
  graphics_exposures: /* Bool:    */ 0,
  clip_x_origin:      /* c_int:   */ 0,
  clip_y_origin:      /* c_int:   */ 0,
  clip_mask:          /* Pixmap:  */ 0,
  dash_offset:        /* c_int:   */ 0,
  dashes:             /* c_char:  */ 0,
  };

pub const DX_TXT                 : i32     = 4;
pub const DY_TXT                 : i32     = 3;

pub static mut pFontInfo   : *mut xlib::XFontStruct     = ptr::null_mut(); // xlib::XQueryFont(pDspl, GC);
pub static mut FontAscent  : i32                        = 0;
pub static mut FontDescent : i32                        = 0;


