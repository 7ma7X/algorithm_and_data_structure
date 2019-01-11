use std::cmp;

fn needleman_wunsch(s1: &str, s2: &str, match_score: i32, miss_score: i32, gap: i32) {
  let s1_bytes = s1.as_bytes();
  let s2_bytes = s2.as_bytes();

  let n = s1_bytes.len();
  let m = s2_bytes.len();

  let mut d: Vec<Vec<i32>> = vec![vec![0; m+1]; n+1];

  for i in 0..n+1 {
    for j in 0..m+1 {
      if i == 0 {
        d[i][j] = gap * (j as i32);
      } else if j == 0 {
        d[i][j] = gap * (i as i32);
      } else {
        let f = if s1_bytes[i-1] == s2_bytes[j-1] { match_score } else { miss_score };
        d[i][j] = cmp::max(cmp::max(d[i-1][j]+gap, d[i][j-1]+gap), d[i-1][j-1]+f);
      }
    }
  }

  for i in 0..n+1 { println!("{:?}", d[i])}
  println!("最大スコア: {}", d[n][m]);

  let max_alignment_1 = "";
  let max_alignment_2 = "";
  println!("最大スコアを与えるアラインメント:");
  nw_trace_back(max_alignment_1, max_alignment_2, n, m, s1_bytes, s2_bytes, &d, match_score, miss_score, gap);
}

fn nw_trace_back(al1: &str, al2: &str, i: usize, j: usize, s1: &[u8], s2: &[u8], d: &Vec<Vec<i32>>,
                mas: i32, mis: i32, gap: i32) {
  if i == 0 && j == 0 {
    println!("{}\n{}", al1, al2);
  } else if j == 0 {
    nw_trace_back(&((s1[i-1] as char).to_string() + al1), &("-".to_owned() + al2), i-1, j, s1, s2, d, mas, mis, gap);
  } else if i == 0 {
    nw_trace_back(&("-".to_owned() + al1), &((s2[j-1] as char).to_string() + al2), i, j-1, s1, s2, d, mas, mis, gap);
  } else {
    if d[i-1][j] + gap == d[i][j] {
      nw_trace_back(&((s1[i-1] as char).to_string() + al1), &("-".to_owned() + al2), i-1, j, s1, s2, d, mas, mis, gap);
    }
    if d[i][j-1] + gap == d[i][j] {
      nw_trace_back(&("-".to_owned() + al1), &((s2[j-1] as char).to_string() + al2), i, j-1, s1, s2, d, mas, mis, gap);
    }
    if s1[i-1] == s2[j-1] && d[i-1][j-1] + mas == d[i][j] {
      nw_trace_back(&((s1[i-1] as char).to_string() + al1), &((s2[j-1] as char).to_string() + al2), i-1, j-1, s1, s2, d, mas, mis, gap);
    } 
    if s1[i-1] != s2[j-1] && d[i-1][j-1] + mis == d[i][j] {
      nw_trace_back(&((s1[i-1] as char).to_string() + al1), &((s2[j-1] as char).to_string() + al2), i-1, j-1, s1, s2, d, mas, mis, gap);
    }
  }
}


fn smith_waterman(s1: &str, s2: &str, match_score: i32, miss_score: i32, gap: i32) {
  let s1_bytes = s1.as_bytes();
  let s2_bytes = s2.as_bytes();

  let n = s1_bytes.len();
  let m = s2_bytes.len();

  let mut d: Vec<Vec<i32>> = vec![vec![0; m+1]; n+1];

  for i in 0..n+1 {
    for j in 0..m+1 {
      if i == 0 {
        d[i][j] = 0;
      } else if j == 0 {
        d[i][j] = 0;
      } else {
        let f = if s1_bytes[i-1] == s2_bytes[j-1] { match_score } else { miss_score };
        d[i][j] = cmp::max(cmp::max(cmp::max(d[i-1][j]+gap, d[i][j-1]+gap), d[i-1][j-1]+f), 0);
      }
    }
  }

  for i in 0..n+1 { println!("{:?}", d[i])}
  let mut max_score = 0; let mut max_i = 0; let mut max_j = 0;
  for i in 0..n+1 {
    for j in 0..m+1 {
      if d[i][j] > max_score {
        max_score = d[i][j];
        max_i = i; max_j = j;
      }
    }
  }
  println!("最大スコア: {}", max_score);

  let max_alignment_1 = "";
  let max_alignment_2 = "";
  println!("最大スコアを与えるアラインメント:");
  sw_trace_back(max_alignment_1, max_alignment_2, max_i, max_j, s1_bytes, s2_bytes, &d, match_score, miss_score, gap);
}

fn sw_trace_back(al1: &str, al2: &str, i: usize, j: usize, s1: &[u8], s2: &[u8], d: &Vec<Vec<i32>>,
                mas: i32, mis: i32, gap: i32) {
  if d[i][j] == 0 {
    println!("{}\n{}", al1, al2);
  } else if j == 0 {
    sw_trace_back(&((s1[i-1] as char).to_string() + al1), &("-".to_owned() + al2), i-1, j, s1, s2, d, mas, mis, gap);
  } else if i == 0 {
    sw_trace_back(&("-".to_owned() + al1), &((s2[j-1] as char).to_string() + al2), i, j-1, s1, s2, d, mas, mis, gap);
  } else {
    if d[i-1][j] + gap == d[i][j] {
      sw_trace_back(&((s1[i-1] as char).to_string() + al1), &("-".to_owned() + al2), i-1, j, s1, s2, d, mas, mis, gap);
    }
    if d[i][j-1] + gap == d[i][j] {
      sw_trace_back(&("-".to_owned() + al1), &((s2[j-1] as char).to_string() + al2), i, j-1, s1, s2, d, mas, mis, gap);
    }
    if s1[i-1] == s2[j-1] && d[i-1][j-1] + mas == d[i][j] {
      sw_trace_back(&((s1[i-1] as char).to_string() + al1), &((s2[j-1] as char).to_string() + al2), i-1, j-1, s1, s2, d, mas, mis, gap);
    } 
    if s1[i-1] != s2[j-1] && d[i-1][j-1] + mis == d[i][j] {
      sw_trace_back(&((s1[i-1] as char).to_string() + al1), &((s2[j-1] as char).to_string() + al2), i-1, j-1, s1, s2, d, mas, mis, gap);
    }
  }
}


fn main() {
  needleman_wunsch("ACGTT", "ATCGTT", 3, -4, -5);
  needleman_wunsch("GATTACA", "GCATGCU", 1, -1, -1);
  needleman_wunsch("CGTT", "GTAT", 5, -5, -8);
  needleman_wunsch("GCTTA", "CCTA", 3, -4, -5);

  smith_waterman("GGTTGACTA", "TGTTACGG", 3, -3, -2);
  smith_waterman("CGTT", "GTAT", 5, -5, -8);
  smith_waterman("ACTGGAC", "CCTAGAGA", 3, -4, -5);
}