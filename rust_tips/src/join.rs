/// 

pub fn join_str() {
	let v = vec![1,2,3];
	let s = v.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(",");
	println!("{:?}", s);
}

pub fn test() {
	join_str();
}