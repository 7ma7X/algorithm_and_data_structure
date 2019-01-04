extern crate rand;

fn bubble(v: &mut Vec<i8>) {
  let n = v.len();

  for i in 0..n-1 {
    for j in (i+1..n).rev() {
      if v[j-1] > v[j] {
        let tmp = v[j-1];
        v[j-1] = v[j];
        v[j] = tmp;
      } 
    }
  }
}


fn quick(v: &mut Vec<i8>) {
  let n = v.len();
  if n <= 1 { return; }

  let pivot = if v[0] > v[1] { v[0] } else { v[1] };

  let mut left_array: Vec<i8> = Vec::new();
  let mut middle_array: Vec<i8> = Vec::new();
  let mut right_array: Vec<i8> = Vec::new();

  for i in 0..n {
    if v[i] < pivot {
      left_array.push(v[i]);
    } else if v[i] > pivot {
      right_array.push(v[i]);
    } else {
      middle_array.push(v[i]);
    }
  }

  let p = left_array.len();
  let q = middle_array.len();
  let r = right_array.len();

  quick(&mut left_array);
  quick(&mut right_array);

  for i in 0..p {
    v[i] = left_array[i];
  }
  for i in 0..q {
    v[p+i] = middle_array[i]; 
  }
  for i in 0..r {
    v[p+q+i] = right_array[i];
  }
}


fn merge(v: &mut Vec<i8>) {
  let n = v.len();
  if n <= 1 { return; }

  let mut left_array = &mut v[0..n/2].to_vec();
  let mut right_array = &mut v[n/2..n].to_vec();

  merge(&mut left_array);
  merge(&mut right_array);

  let p = left_array.len();
  let q = right_array.len();

  let mut i = 0;
  let mut j = 0;

  loop {
    if i == p && j == q { break; }

    if j == q {
      v[i+q] = left_array[i];
      i = i + 1;
    } else if i == p {
      v[p+j] = right_array[j];
      j = j + 1;
    } else {
      if left_array[i] < right_array[j] {
        v[i+j] = left_array[i];
        i = i + 1;
      } else {
        v[i+j] = right_array[j];
        j = j + 1;
      }
    }
  }
}


fn main() {
  let mut v: Vec<i8> = vec![0; 100];

  for x in v.iter_mut() {
    *x = rand::random();
  }

  println!("もとの配列:");
  println!("{:?}", v);
  bubble(&mut v);
  println!("バブルソート完了後:");
  println!("{:?}", v);

  for x in v.iter_mut() {
    *x = rand::random();
  }

  println!("もとの配列:");
  println!("{:?}", v);
  quick(&mut v);
  println!("クイックソート完了後:");
  println!("{:?}", v);

  for x in v.iter_mut() {
    *x = rand::random();
  }

  println!("もとの配列:");
  println!("{:?}", v);
  merge(&mut v);
  println!("マージソート完了後:");
  println!("{:?}", v);
}