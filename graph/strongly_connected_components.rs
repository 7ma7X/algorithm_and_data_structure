struct Edge {
  from: usize,
  to: usize
}

fn strongly_connected_components(v: Vec<Edge>, n: usize) {
  let mut visited = vec![false; n];
  let mut sorted = Vec::new();

  for i in 0..n {
    dfs(i, &v, &mut visited, &mut sorted);
  }

  println!("{:?}", sorted); // 後ろの方が優先順位高い

  visited = vec![false; n];

  while !sorted.is_empty() {
    let i = sorted.pop().unwrap();
    let mut res = Vec::new();
    dfs2(i, &v, &mut visited, &mut sorted, &mut res);
    println!("強連結成分: {:?}", res);
  }
}

fn dfs(i: usize, v: &Vec<Edge>, visited: &mut Vec<bool>, sorted: &mut Vec<usize>) {
  if visited[i] { return; }
  visited[i] = true;

  for e in v {
    if e.from == i {
      dfs(e.to, v, visited, sorted);
    }
  }

  sorted.push(i);
}

fn dfs2(i: usize, v: &Vec<Edge>, visited: &mut Vec<bool>, sorted: &mut Vec<usize>, res: &mut Vec<usize>) {
  if visited[i] { return; }
  visited[i] = true;
  res.push(i);
  sorted.retain(|&x| x != i);

  for e in v {
    if e.to == i && sorted.contains(&e.from) {
      dfs2(e.from, v, visited, sorted, res);
    }
  }
}

fn main() {
  let mut graph = Vec::new();
  graph.push(Edge{ from: 0, to: 1 });
  graph.push(Edge{ from: 0, to: 3 });
  graph.push(Edge{ from: 1, to: 2 });
  graph.push(Edge{ from: 1, to: 3 });
  graph.push(Edge{ from: 2, to: 4 });
  graph.push(Edge{ from: 3, to: 0 });
  graph.push(Edge{ from: 3, to: 4 });
  graph.push(Edge{ from: 4, to: 2 });
  graph.push(Edge{ from: 5, to: 2 });
  graph.push(Edge{ from: 5, to: 4 });
  strongly_connected_components(graph, 6);

  let mut graph2 = Vec::new();
  graph2.push(Edge{ from: 0, to: 1 });
  graph2.push(Edge{ from: 1, to: 2 });
  graph2.push(Edge{ from: 1, to: 4 });
  graph2.push(Edge{ from: 3, to: 0 });
  graph2.push(Edge{ from: 3, to: 4 });
  graph2.push(Edge{ from: 4, to: 0 });
  graph2.push(Edge{ from: 4, to: 2 });
  strongly_connected_components(graph2, 5);

  let mut graph3 = Vec::new();
  graph3.push(Edge{ from: 0, to: 2 });
  graph3.push(Edge{ from: 1, to: 4 });
  graph3.push(Edge{ from: 3, to: 1 });
  graph3.push(Edge{ from: 3, to: 2 });
  graph3.push(Edge{ from: 4, to: 3 });
  strongly_connected_components(graph3, 5); 
}