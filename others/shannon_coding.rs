fn shannon(p: &mut Vec<f64>) -> Vec<String> {
  // まず確率の大きさが降順になるように整列する
  p.sort_by(|a, b| b.partial_cmp(a).unwrap());

  let n = p.len();
  let mut encoded_p: Vec<String> = Vec::new();
  let mut c: f64 = 0.0;

  for i in 0..n {
    let l = (-p[i].log2()).ceil() as i32;
    let mut code = "".to_string();
    let mut check_c = c;

    for _j in 0..l {
      check_c *= 2.0;

      if check_c >= 1.0 {
        code += "1";
        check_c -= 1.0;
      } else {
        code += "0";
      }
    }

    c += p[i];
    encoded_p.push(code);
  }

  encoded_p
}

fn main() {
  let mut v: Vec<f64> = vec![0.4, 0.25, 0.2, 0.1, 0.05];
  println!("{:?}", shannon(&mut v));

  v = vec![0.36, 0.18, 0.18, 0.12, 0.09, 0.07];
  println!("{:?}", shannon(&mut v));

  v = vec![0.08, 0.36, 0.12, 0.24, 0.2];
  println!("{:?}", shannon(&mut v));
}