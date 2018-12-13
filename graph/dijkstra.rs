// v: 頂点集合, d: 辺のコスト, s: 出発点
fn dijkstra(mut v: Vec<usize>, d: Vec<Vec<i32>>, s: usize) {
  let mut c = vec![1000000; v.len()]; // コスト1000000で初期化

  for i in &v {
    c[*i] = d[s][*i];
  }

  println!("{:?}", c); // 経過観察用

  v.retain(|&x| x != s);

  while !v.is_empty() {
    let mut min = 1000000;
    let mut x = 0;

    for i in &v {
      if c[*i] < min {
        min = c[*i];
        x = *i;
      }
    }

    v.retain(|&i| i != x);

    for i in &v {
      if (c[x] + d[x][*i]) < c[*i] {
        c[*i] = c[x] + d[x][*i] // コストの更新
      }
    }

    println!("{:?}", c); // 経過観察用
  }
  println!("Ans: {:?}", c);
}

fn main() {
  let v1 = vec![0, 1, 2, 3];
  let mut d1 = vec![vec![100000; 4]; 4];
  let s1 = 0;

  for i in &v1 { d1[*i][*i] = 0; }
  d1[0][1] = 2;
  d1[1][2] = 3;
  d1[0][2] = 6;
  d1[2][3] = 2;

  dijkstra(v1, d1, s1);

  let v2 = vec![0, 1, 2, 3];
  let mut d2 = vec![vec![100000; 4]; 4];
  let s2 = 0;

  for i in &v2 { d2[*i][*i] = 0; }
  d2[0][1] = 6;
  d2[0][2] = 1;
  d2[2][1] = 2;
  d2[1][3] = 1;
  d2[2][3] = 4;

  dijkstra(v2, d2, s2);
}