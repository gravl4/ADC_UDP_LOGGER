#[allow(unused_macros)]

#[cfg(feature = "USE_DEBUG")]
#[macro_export]
macro_rules! DBG_MSG {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}


#[cfg(not(feature = "USE_DEBUG"))]
#[macro_export]
macro_rules! DBG_MSG {
    ($( $args:expr ),*) => {}
}

pub(crate) use DBG_MSG;


#[cfg(feature = "DO_TIMING")]
#[macro_export]
macro_rules! SET_START 
{
  ( $Start: ident ) => 
  {
    let $Start= SystemTime::now();
  }
}

#[cfg(feature = "DO_TIMING")]
#[macro_export]
macro_rules! PRINT_DURATION 
{
  ( $Start: ident, $d_tag: literal ) => { let D= $Start.elapsed().unwrap(); println!(" {}:    {}   {}   {:?} ", $d_tag, D.as_millis(), D.as_micros(), D); };
  ( $Start: ident, $d_tag: literal, $param: expr ) => { let D= $Start.elapsed().unwrap(); println!(" {}: {}   {}   {}   {:?} ", $d_tag, $param, D.as_millis(), D.as_micros(), D); }
}

#[cfg(not(feature = "DO_TIMING"))]
#[macro_export]
macro_rules! SET_START 
{
  ( $Start: ident ) => { }
}

#[cfg(not(feature = "DO_TIMING"))]
#[macro_export]
macro_rules! PRINT_DURATION 
{
  ( $Start: ident, $d_tag: literal ) => { };
  ( $Start: ident, $d_tag: literal, $param: expr ) => { }
}

pub(crate) use SET_START;
pub(crate) use PRINT_DURATION;


#[cfg(feature = "TRACE_GUI")]
#[macro_export]
macro_rules! PRINT_TRACE 
{
  ($( $args:expr ),*) => 
  { 
    println!( $( $args ),* );
    println!("line: {}", line!());
  };
  () =>
  { 
    println!("Line: {}", line!());
  }
}

#[cfg(not(feature = "TRACE_GUI"))]
#[macro_export]
macro_rules! PRINT_TRACE 
{
  ($( $args:expr ),*) => { };
  () => { }
}

pub(crate) use PRINT_TRACE;