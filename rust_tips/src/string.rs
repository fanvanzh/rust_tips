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
