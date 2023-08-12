fn main(){
    let v1 = 251_u16+ 9;
    let v2 = i16::checked_add(251,8).unwrap();
    println!("{},{}",v1,v2);
    range_def();
}

use std::ops::{Range, RangeInclusive};
fn range_def(){
    assert_eq!((1..5), Range{start:1, end:5});
    assert_eq!((1..=5), RangeInclusive::new(1,5));
    assert!(9.6 as f32 /3.2 as f32 == 3.0);
    println!("Success")
}
