use std::collections::HashSet;

struct Edge {
  from: usize,
  to: usize
}

fn lowlink(v: Vec<Edge>, n: usize) {
  let mut visited = vec![false; n];
  let mut used = vec![false; v.len()];
  let mut num = vec![0; n];
  let mut low = vec![0; n];

  let mut visited_iter = 1;

  for i in 0..n {
    dfs(i, &mut visited_iter, &v, &mut visited, &mut used, &mut num, &mut low);
  }

  println!("親子関係: {:?}", used);
  println!("num: {:?}", num);
  println!("low: {:?}", low);

  // 関節点を求める

  let mut root_child_num = 0;
  let mut res: HashSet<usize> = HashSet::new();

  for (iter, b) in used.into_iter().enumerate() {
    if b {
      let tmp_from = v[iter].from;
      let tmp_to = v[iter].to;
      if tmp_from == 0 {
        root_child_num = root_child_num + 1;
      } else {
        if low[tmp_to] >= num[tmp_from] {
          res.insert(tmp_from);
        }
      }
    }
  }

  if root_child_num >= 2 { res.insert(0); }

  println!("関節点: {:?}", res);
}

fn dfs(i: usize, n: &mut usize, v: &Vec<Edge>, visited: &mut Vec<bool>, used: &mut Vec<bool>, num: &mut Vec<usize>, low: &mut Vec<usize>) {
  if visited[i] { return; } // すでに訪問済み
  visited[i] = true; // 訪問済みの印を付ける

  num[i] = *n; // 行きがけに番号を振る
  *n += 1;

  println!("{} を訪問中", i); // 経過観察用

  for (iter, e) in v.into_iter().enumerate() {
    if e.from == i {
      if !visited[e.to] { used[iter] = true; }
      dfs(e.to, n, v, visited, used, num, low);
    }
  }

  low[i] = num[i]; // まず num にそろえる
  for e in v {
    if e.from == i && num[e.to] != 0 && (num[e.to] < low[i]) {
      low[i] = num[e.to]; // 後退辺の確認
    }
  }

  for (iter, b) in used.into_iter().enumerate() {
    if *b {
      let tmp_from = v[iter].from;
      let tmp_to = v[iter].to;
      if i == tmp_from && low[tmp_to] < low[i] {
        low[i] = low[tmp_to];
      }
    }
  }

  println!("{} に {} を割り当てた", i, low[i]); // 経過観察用
  println!("{} を訪問した", i); // 経過観察用
}

fn main() {
  let mut graph: Vec<Edge> = Vec::new();
  graph.push(Edge{ from: 0, to: 1 });
  graph.push(Edge{ from: 0, to: 3 });
  graph.push(Edge{ from: 1, to: 0 });
  graph.push(Edge{ from: 1, to: 2 });
  graph.push(Edge{ from: 1, to: 3 });
  graph.push(Edge{ from: 1, to: 4 });
  graph.push(Edge{ from: 1, to: 5 });
  graph.push(Edge{ from: 2, to: 1 });
  graph.push(Edge{ from: 2, to: 5 });
  graph.push(Edge{ from: 3, to: 0 });
  graph.push(Edge{ from: 3, to: 1 });
  graph.push(Edge{ from: 3, to: 4 });
  graph.push(Edge{ from: 4, to: 1 });
  graph.push(Edge{ from: 4, to: 3 });
  graph.push(Edge{ from: 5, to: 1 });
  graph.push(Edge{ from: 5, to: 2 });
  lowlink(graph, 6);

  let mut graph2: Vec<Edge> = Vec::new();
  graph2.push(Edge{ from: 0, to: 1 });
  graph2.push(Edge{ from: 0, to: 3 });
  graph2.push(Edge{ from: 1, to: 0 });
  graph2.push(Edge{ from: 1, to: 2 });
  graph2.push(Edge{ from: 1, to: 3 });
  graph2.push(Edge{ from: 2, to: 1 });
  graph2.push(Edge{ from: 2, to: 4 });
  graph2.push(Edge{ from: 2, to: 5 });
  graph2.push(Edge{ from: 3, to: 0 });
  graph2.push(Edge{ from: 3, to: 1 });
  graph2.push(Edge{ from: 4, to: 2 });
  graph2.push(Edge{ from: 4, to: 5 });
  graph2.push(Edge{ from: 5, to: 2 });
  graph2.push(Edge{ from: 5, to: 4 });
  lowlink(graph2, 6);
}