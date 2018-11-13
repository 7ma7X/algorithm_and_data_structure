enum Tree<T> {
  Empty,
  Node {
    value: T,
    left: Box<Tree<T>>,
    right: Box<Tree<T>>,
  }
}

impl Tree<i32> {
  fn new() -> Tree<i32> {
    Tree::Empty
  }

  // 挿入のコード. 再帰的に下りていって目的の場所を見つけてノードを追加する
  fn insert(&mut self, object: i32) {
    match self {
      Tree::Empty => {
        // 新しいノードを追加
        *self = Tree::Node {value: object, left: Box::new(Tree::Empty), right: Box::new(Tree::Empty)}
      }
      Tree::Node {value: v, left: left_node, right: right_node} => {
        if object < *v 
        {
          left_node.insert(object); // 左の子をたどる
        }
        else 
        {
          right_node.insert(object); // 右の子をたどる
        }
      }
    }
  }

  // 結果表示用の関数
  fn result(&self) {
    match self {
      Tree::Empty => {
        return;
      }
      Tree::Node {value: v, left: left_node, right: right_node} => {
        println!("{}", v);
        println!("{} の左の子を見ます↓", v);
        left_node.result();
        println!("{} の右の子を見ます↓", v);
        right_node.result();
      }
    }
  }
}

fn main() {
  let mut tree = Tree::new();
  tree.insert(34);
  tree.result(); println!("---end---");
  tree.insert(51);
  tree.result(); println!("---end---");
  tree.insert(72);
  tree.result(); println!("---end---");
  tree.insert(17);
  tree.result(); println!("---end---");
  tree.insert(66);
  tree.result(); println!("---end---");

  /*
    34
    34 の左の子を見ます↓
    34 の右の子を見ます↓
    ---end---
    34
    34 の左の子を見ます↓
    34 の右の子を見ます↓
    51
    51 の左の子を見ます↓
    51 の右の子を見ます↓
    ---end---
    34
    34 の左の子を見ます↓
    34 の右の子を見ます↓
    51
    51 の左の子を見ます↓
    51 の右の子を見ます↓
    72
    72 の左の子を見ます↓
    72 の右の子を見ます↓
    ---end---
    34
    34 の左の子を見ます↓
    17
    17 の左の子を見ます↓
    17 の右の子を見ます↓
    34 の右の子を見ます↓
    51
    51 の左の子を見ます↓
    51 の右の子を見ます↓
    72
    72 の左の子を見ます↓
    72 の右の子を見ます↓
    ---end---
    34
    34 の左の子を見ます↓
    17
    17 の左の子を見ます↓
    17 の右の子を見ます↓
    34 の右の子を見ます↓
    51
    51 の左の子を見ます↓
    51 の右の子を見ます↓
    72
    72 の左の子を見ます↓
    66
    66 の左の子を見ます↓
    66 の右の子を見ます↓
    72 の右の子を見ます↓
    ---end---
  */
}