fn index_str() {
  let msg = "hello world";
  println!("{}", &msg[0..2]);
  let msg = "汉字";
  println!("{}", &msg[0..3]);
}

fn to_vec8() {
  let msg = String::from("hello");
  let v = msg.into_bytes();
  println!("{:?}", v);
}

fn from_vec8() {
  let msg = vec![240, 159, 146, 150];
  println!("{:?}", String::from_utf8_uncheck(msg.clone()));
  println!("{:?}", String::from_utf8(msg.clone()).unwrap());
}

fn from_c() {
  use std::ffi::CStr;
  let ptr = 0 as *const i8;
  let msg = CStr::from_ptr(ptr);
  println!("msg is {}", msg.to_str().unwrap());
}

fn to_c() {
  use std::ffi::CString;
  let c_str = CString::new("hello").unwrap();;
  let ptr = c_str.as_ptr();
  // ptr is with 0 terminated.
}
