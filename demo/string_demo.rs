// So we don't get "function complexity" lints and such since it's a demo.
#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(incomplete_features)]

use core::iter::FromIterator;
use staticvec::{staticstring, staticvec, StaticString};

// There'll eventually be more stuff here probably, but for now it just tries
// to show the more "interesting" features.

const CONST_UNICODE: StaticString<255> = staticstring!("👍🍉🙃👍🍉🙃👍🍉🙃👍🍉🙃", 255);

static STATIC_UNICODE: StaticString<255> = staticstring!("🙉🙉💣💣🙉🙉💣💣🙉🙉💣💣", 255);

fn main() {
  let mut s = StaticString::<4>::new();
  s.push_str("🤔");
  println!("Value: {}", s);
  println!("Debug info: {:?}", s);
  println!("Length: {}", s.len());
  println!("Remaining capacity: {}", s.remaining_capacity());
  let mut s2 = StaticString::<4>::new();
  s2.push('🤔');
  println!("Value: {}", s2);
  println!("Debug info: {:?}", s2);
  println!("Length: {}", s2.len());
  println!("Remaining capacity: {}", s2.remaining_capacity());
  let s3 = StaticString::<7>::from_chars(
    staticvec!['A', 'B', 'C', 'D', 'E', 'F', 'G']
      .reversed()
      .into_iter(),
  );
  println!("Value: {}", s3);
  println!("Debug info: {:?}", s3);
  println!("Length: {}", s3.len());
  println!("Remaining capacity: {}", s3.remaining_capacity());
  let v = staticvec!["ABCDEFG", "HIJKLMNOP", "QRSTUV", "WXYZ"];
  let s4 = StaticString::<26>::from_iter(v.into_iter());
  println!("Value: {}", s4);
  println!("Debug info: {:?}", s4);
  println!("Length: {}", s4.len());
  println!("Remaining capacity: {}", s4.remaining_capacity());
  let mut s5 = StaticString::<6>::from("ABEF");
  s5.insert_str(2, "CD");
  println!("Value: {}", s5);
  println!("Debug info: {:?}", s5);
  println!("Length: {}", s5.len());
  println!("Remaining capacity: {}", s5.remaining_capacity());
  s5.replace_range(2..4, "XY");
  println!("Value: {}", s5);
  println!("Debug info: {:?}", s5);
  println!("Length: {}", s5.len());
  println!("Remaining capacity: {}", s5.remaining_capacity());
  let mut s6 = StaticString::<5>::from(" ABC ");
  println!("Value: {}", s6);
  println!("Debug info: {:?}", s6);
  println!("Length: {}", s6.len());
  println!("Remaining capacity: {}", s6.remaining_capacity());
  s6.trim();
  println!("Value: {}", s6);
  println!("Debug info: {:?}", s6);
  println!("Length: {}", s6.len());
  println!("Remaining capacity: {}", s6.remaining_capacity());
  let mut a = StaticString::<6>::from("ABCDEF");
  let b = a.split_off(3);
  println!("Value: {}", a);
  println!("Debug info: {:?}", a);
  println!("Length: {}", a.len());
  println!("Remaining capacity: {}", a.remaining_capacity());
  println!("Value: {}", b);
  println!("Debug info: {:?}", b);
  println!("Length: {}", b.len());
  println!("Remaining capacity: {}", b.remaining_capacity());
  let mut s7 = StaticString::<12>::from("🤔ABCD🤔");
  s7.retain(|c| c != '🤔');
  println!("Value: {}", s7);
  println!("Debug info: {:?}", s7);
  println!("Length: {}", s7.len());
  println!("Remaining capacity: {}", s7.remaining_capacity());
  let mut s8 = StaticString::<8>::new();
  for _ in 0..12 {
    s8.push_str_truncating("A");
  }
  println!("Value: {}", s8);
  println!("Debug info: {:?}", s8);
  println!("Length: {}", s8.len());
  println!("Remaining capacity: {}", s8.remaining_capacity());
  println!("Value: {}", CONST_UNICODE);
  println!("Debug info: {:?}", CONST_UNICODE);
  println!("Length: {}", CONST_UNICODE.len());
  println!("Remaining capacity: {}", CONST_UNICODE.remaining_capacity());
  println!("Value: {}", STATIC_UNICODE);
  println!("Debug info: {:?}", STATIC_UNICODE);
  println!("Length: {}", STATIC_UNICODE.len());
  println!(
    "Remaining capacity: {}",
    STATIC_UNICODE.remaining_capacity()
  );
  let runtime_unicode = staticstring!("🙉🙉💣💣🙉🙉💣💣🙉🙉💣💣", 255);
  println!("Value: {}", runtime_unicode);
  println!("Debug info: {:?}", runtime_unicode);
  println!("Length: {}", runtime_unicode.len());
  println!(
    "Remaining capacity: {}",
    runtime_unicode.remaining_capacity()
  );
}
