fn prim(d: Vec<Vec<i32>>, mut v: Vec<usize>) {
  let mut c = vec![0; v.len()];
  let mut n: Vec<usize> = vec![0; v.len()];
  for i in &v {
    c[*i] = d[0][*i];
    n[*i] = 0;
  }

  v.retain(|&x| x != 0);

  println!("{:?}", c); // 経過観察用

  let mut t: Vec<(usize, usize)> = Vec::new();

  while !v.is_empty() {
    let mut min_c = 100000;
    let mut w = 0;
    for i in &v {
      if c[*i] < min_c {
        min_c = c[*i];
        w = *i;
      }
    }

    v.retain(|&x| x != w);
    t.push((w, n[w]));

    for i in &v {
      if d[w][*i] < c[*i] {
        c[*i] = d[w][*i];
        n[*i] = w;
      } 
    }

    println!("{:?}", c); // 経過観察用
  }

  println!("{:?}", t);
}

fn main() {
  let mut graph = vec![vec![100000; 6]; 6];
  for i in 0..6 { graph[i][i] = 0; }
  graph[0][1] = 8; graph[1][0] = 8;
  graph[0][2] = 3; graph[2][0] = 3;
  graph[0][3] = 1; graph[3][0] = 0;
  graph[1][2] = 9; graph[2][1] = 9;
  graph[2][3] = 4; graph[3][2] = 4;
  graph[1][4] = 2; graph[4][1] = 2;
  graph[2][4] = 7; graph[4][2] = 7;
  graph[2][5] = 6; graph[5][2] = 6;
  graph[3][5] = 5; graph[5][3] = 5;
  graph[4][5] = 10; graph[5][4] = 10;
  let v: Vec<usize> = vec![0, 1, 2, 3, 4, 5];

  prim(graph, v);

  let mut graph2 = vec![vec![100000; 6]; 6];
  for i in 0..6 { graph2[i][i] = 0; }
  graph2[0][1] = 5; graph2[1][0] = 5;
  graph2[1][2] = 7; graph2[2][1] = 7;
  graph2[0][3] = 1; graph2[3][0] = 1;
  graph2[0][4] = 4; graph2[4][0] = 4;
  graph2[1][4] = 6; graph2[4][1] = 6;
  graph2[2][4] = 8; graph2[4][2] = 8;
  graph2[2][5] = 3; graph2[5][2] = 3;
  graph2[3][4] = 2; graph2[4][3] = 2;
  graph2[4][5] = 9; graph2[5][4] = 9;
  let v2: Vec<usize> = vec![0, 1, 2, 3, 4, 5];

  prim(graph2, v2);
}