struct Pque {
  heap: [i32; 100000],
  last: usize,
}

impl Pque {
  fn new() -> Pque {
    Pque {heap: [0; 100000], last: 0}
  }

  fn insert(&mut self, object: i32) {
    // 最後尾に要素を追加
    if self.heap[self.last] != 0 // ヒープが空でない場合
    {
      self.last += 1;
    }
    self.heap[self.last] += object;

    // 上へとたどりながら逆転を解消していく
    let mut i = self.last;
    while i > 0 {
      if self.heap[(i - 1) / 2] > self.heap[i] // 親の方が大きい
      { 
        self.heap.swap((i - 1) / 2, i); // 入れ替え
        i = (i - 1) / 2; // 親へ移動
      } 
      else {
        return;
      }
    }
  }

  fn deletemin(&mut self) -> i32 {
    // 根の要素を取り出す
    let object = self.heap[0];
    // 最後尾を根にもってくる
    self.heap[0] = self.heap[self.last];
    self.heap[self.last] = 0;
    self.last -= 1;

    // 下へとたどりながら逆転を解消していく
    let mut i: usize = 0;
    while i < self.last / 2 {
      if self.heap[i] > self.heap[i * 2 + 1] // 親 > 左の子
      {
        if self.heap[i * 2 + 2] < self.heap[i * 2 + 1] // 右の子 < 左の子
        {
          self.heap.swap(i, i * 2 + 2); // 右の子と交換
          i = i * 2 + 2; // 右の子へ移動
        }
        else // 右の子 > 左の子
        {
          self.heap.swap(i, i * 2 + 1); // 左の子と交換
          i = i * 2 + 1; // 左の子へ移動            
        }
      } 
      else if self.heap[i] > self.heap[i * 2 + 2] // 親 > 右の子
      {
        self.heap.swap(i, i * 2 + 2); // 右の子と交換
        i = i * 2 + 2; // 右の子へ移動           
      }
    }
    object
  }

  // 結果表示用の関数
  fn result(&self) {
    for x in 0..self.last+1 {
      print!("{} ", self.heap[x]);
    }
    print!("\n");
  }
}

fn main () {
  let mut pque = Pque::new();

  pque.insert(31);
  pque.result();
  pque.insert(21);
  pque.result();
  pque.insert(48);
  pque.result();
  pque.insert(9);
  pque.result();
  println!("{} が取り出されました", pque.deletemin());
  pque.result();
  pque.insert(26);
  pque.result();
  pque.insert(13);
  pque.result();
  println!("{} が取り出されました", pque.deletemin());
  pque.result();

  /*
    31
    21 31
    21 31 48
    9 21 48 31
    9 が取り出されました
    21 31 48
    21 26 48 31
    13 21 48 31 26
    13 が取り出されました
    21 26 48 31
  */

  pque = Pque::new();
  pque.insert(12);
  pque.result();
  pque.insert(24);
  pque.result();
  pque.insert(8);
  pque.result();
  pque.insert(16);
  pque.result();
  println!("{} が取り出されました", pque.deletemin());
  pque.result();
  pque.insert(14);
  pque.result();
  println!("{} が取り出されました", pque.deletemin());
  pque.result();

  /*
    12
    12 24
    8 24 12
    8 16 12 24
    8 が取り出されました
    12 16 24
    12 14 24 16
    12 が取り出されました
    14 16 24
  */
}