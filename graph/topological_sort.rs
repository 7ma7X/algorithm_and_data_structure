struct Edge {
  from: usize,
  to: usize
}

fn topological_sort(v: Vec<Edge>, n: usize) {
  let mut visited = vec![false; n];
  let mut sorted: Vec<usize> = Vec::new();

  for i in 0..n {
    dfs(i, &v, &mut visited, &mut sorted);
  }

  sorted.reverse();
  println!("{:?}", sorted);
}

fn dfs(i: usize, v: &Vec<Edge>, visited: &mut Vec<bool>, sorted: &mut Vec<usize>) {
  if visited[i] { return; } // すでに訪問済み
  visited[i] = true; // 訪問済みの印を付ける

  println!("{} を訪問中", i); // 経過観察用

  for e in &(*v) {
    if e.from == i {
      dfs(e.to, v, visited, sorted);
    }
  }

  sorted.push(i); // 帰りがけにリストに追加

  println!("{} を訪問した", i); // 経過観察用
}

fn main() {
  let mut graph: Vec<Edge> = Vec::new();
  graph.push(Edge{ from: 0, to: 1 });
  graph.push(Edge{ from: 0, to: 3 });
  graph.push(Edge{ from: 1, to: 4 });
  graph.push(Edge{ from: 2, to: 1 });
  graph.push(Edge{ from: 2, to: 3 });
  graph.push(Edge{ from: 3, to: 4 });
  graph.push(Edge{ from: 3, to: 5 });
  graph.push(Edge{ from: 4, to: 5 });
  topological_sort(graph, 6);

  let mut graph2: Vec<Edge> = Vec::new();
  graph2.push(Edge{ from: 0, to: 1 });
  graph2.push(Edge{ from: 0, to: 3 });
  graph2.push(Edge{ from: 1, to: 2 });
  graph2.push(Edge{ from: 1, to: 5 });
  graph2.push(Edge{ from: 2, to: 3 });
  graph2.push(Edge{ from: 4, to: 0 });
  graph2.push(Edge{ from: 4, to: 2 });
  graph2.push(Edge{ from: 5, to: 2 });
  topological_sort(graph2, 6);
}