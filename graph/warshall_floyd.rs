fn warshall_floyd (v: Vec<usize>, d: Vec<Vec<i32>>) {
  let mut c = vec![vec![100000; v.len()]; v.len()];

  for i in &v {
    for j in &v { c[*i][*j] = d[*i][*j]; }
  }

  for i in &v {
    for j in &v { print!("{} ", c[*i][*j]); }
    println!("");
  } 
  println!(""); // 経過観察用

  for i in &v {
    for j in &v {
      for k in &v {
        if (c[*j][*i] + c[*i][*k]) < c[*j][*k] {
          c[*j][*k] = c[*j][*i] + c[*i][*k];
        }
      }
    }

    for j in &v {
      for k in &v { print!("{} ", c[*j][*k]); }
      println!("");
    }
    println!(""); // 経過観察用
  }                                                                                       
}

fn main() {
  let v1 = vec![0, 1, 2, 3];
  let mut d1 = vec![vec![100000; 4]; 4];

  for i in 0..4 { d1[i][i] = 0; }
  d1[0][1] = 2;
  d1[0][2] = 6;
  d1[0][3] = 9;
  d1[1][2] = 3;
  d1[2][3] = 1;
  d1[3][0] = 2;

  warshall_floyd(v1, d1);

  let v2 = vec![0, 1, 2, 3];
  let mut d2 = vec![vec![100000; 4]; 4];

  for i in 0..4 { d2[i][i] = 0; }
  d2[0][1] = 4;
  d2[0][2] = 9;
  d2[1][3] = 3;
  d2[2][1] = 2;
  d2[3][0] = 1;
  d2[3][2] = 1;

  warshall_floyd(v2, d2);
}