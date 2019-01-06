use std::collections::HashSet;

struct Edge {
  from: usize,
  to: usize
}

impl PartialEq for Edge {
  fn eq(&self, other: &Edge) -> bool {
    self.from == other.from && self.to == other.to
  }
}

fn lowlink(v: Vec<Edge>, n: usize) {
  let mut visited = vec![false; n]; // 訪問済みか否か
  let mut used: Vec<Edge> = Vec::new(); // DFSで使われる親子辺（そうでないのは後退辺）
  let mut num = vec![0; n];
  let mut low = vec![0; n];

  let mut visited_iter = 1;

  // 探索

  for i in 0..n {
    dfs(i, &mut visited_iter, &v, &mut visited, &mut used, &mut num, &mut low);
  }

  println!("num: {:?}", num);
  println!("low: {:?}", low);

  // 関節点を求める

  let mut root_child_num = 0;
  let mut res: HashSet<usize> = HashSet::new();

  for e in used {
    if e.from == 0 {
      root_child_num = root_child_num + 1;
    } else {
      if low[e.to] >= num[e.from] {
        res.insert(e.from);
      }
    }
  }

  if root_child_num >= 2 { res.insert(0); }

  println!("関節点: {:?}", res);
}

fn dfs(i: usize, n: &mut usize, v: &Vec<Edge>, visited: &mut Vec<bool>, used: &mut Vec<Edge>, num: &mut Vec<usize>, low: &mut Vec<usize>) {
  if visited[i] { return; } // すでに訪問済み
  visited[i] = true; // 訪問済みの印を付ける

  num[i] = *n; // 行きがけに番号を振る
  *n += 1;

  println!("{} を訪問中", i); // 経過観察用
  println!("{} に num として {} を割り当てた", i, num[i]); // 経過観察用

  for e in v {
    // 無向グラフなので双方向必要
    if e.from == i {
      if !visited[e.to] {
        used.push(Edge{ from: e.from, to: e.to });
      }
      dfs(e.to, n, v, visited, used, num, low);
    }
    if e.to == i {
      if !visited[e.from] {
        used.push(Edge{ from: e.to, to: e.from });
      }
      dfs(e.from, n, v, visited, used, num, low);
    }
  }

  // low の計算
  low[i] = num[i]; // まず num にそろえる
  // 後退辺のnumの確認
  for e in v {
    if e.from == i && num[e.to] != 0 
        && !used.contains(&Edge{ from: e.from, to: e.to }) // 使用済み経路を除外したものが後退辺
        && !used.contains(&Edge{ from: e.to, to: e.from }) {
      low[i] = if num[e.to] < low[i] { num[e.to] } else { low[i] }; // 後退辺の先がより小さければ更新
    }
    if e.to == i && num[e.from] != 0 
        && !used.contains(&Edge{ from: e.from, to: e.to })
        && !used.contains(&Edge{ from: e.to, to: e.from }) {
      low[i] = if num[e.from] < low[i] { num[e.from] } else { low[i] }; 
    }
  }
  // 子のlowの確認
  for e in used {
    if i == e.from  {
      low[i] = if low[e.to] < low[i] { low[e.to] } else { low[i] }; // 子の low がより小さければ更新
    }
  }

  println!("{} に low として {} を割り当てた", i, low[i]); // 経過観察用
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

  let mut graph3: Vec<Edge> = Vec::new();
  graph3.push(Edge{ from: 0, to: 1 });
  graph3.push(Edge{ from: 0, to: 3 });
  graph3.push(Edge{ from: 1, to: 4 });
  graph3.push(Edge{ from: 2, to: 5 });
  graph3.push(Edge{ from: 3, to: 4 });
  graph3.push(Edge{ from: 4, to: 5 });
  lowlink(graph3, 6);
}