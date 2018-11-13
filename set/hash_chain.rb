# チェイン法

class Hash
  attr_accessor :data

  def initialize(hash_size)
    @data = Array.new(hash_size).map{[]}
  end

  # ハッシュ関数
  # 例: 文字列の長さ
  
  def self.get_index(object)
    object.size
  end

  # 要素が含まれているかどうかチェック
  
  def member(object)
    i = Hash.get_index(object)
    @data[i].include?(object)
  end

  # 要素の追加
  
  def insert(object)
    i = Hash.get_index(object)
    @data[i].push(object)
  end

  # 要素の削除
  
  def delete(object)
    i = Hash.get_index(object)
    @data[i].delete(object)
  end

end


hash1 = Hash.new(5)
hash1.insert("dog")
p hash1.data
hash1.insert("bird")
p hash1.data
hash1.insert("cat")
p hash1.data
hash1.delete("dog")
p hash1.data
hash1.insert("rat")
p hash1.data
