# 開番地法

class Hash
  attr_accessor :data, :hash_size

  def initialize(hash_size)
    @hash_size = hash_size
    @data = Array.new(hash_size)
  end

  # ハッシュ関数
  # 例: 文字列の長さ
  
  def self.get_index(object, count)
    object.size + count
  end

  # 要素が含まれているかどうかチェック

  def member(object)
    count = 0

    while count <= @hash_size
      i = Hash.get_index(object, count).modulo(@hash_size)

      # 含まれていなかった
      unless @data[i]
        return false
      end
      # 含まれていた
      if @data[i] == object
        return true
      end
      # 他のもの, "deleted"
      count += 1 # 再ハッシュとして一次ハッシュを使う

    end
  end

  # 要素の追加

  def insert(object)
    # 重複チェック
    if member(object)
      return
    end
    
    count = 0

    while count <= @hash_size
      i = Hash.get_index(object, count).modulo(@hash_size)

      # 発見
      if @data[i] == nil or @data[i] == "deleted"
        @data[i] = object
        return
      end 
      # 他のもの
      
      count += 1 # 再ハッシュとして一次ハッシュを使う

    end
  end

  # 要素の削除
  
  def delete(object)
    count = 0
  
    while count <= @hash_size
      i = Hash.get_index(object, count).modulo(@hash_size)

      # 発見
      if @data[i] == object
        @data[i] = "deleted"
        return
      end
      # 他のもの, deleted
      count += 1 # 再ハッシュとして一次ハッシュを使う

    end
  end

end


hash2 = Hash.new(5)
hash2.insert("dog")
p hash2.data
hash2.insert("bird")
p hash2.data
hash2.insert("cat")
p hash2.data
hash2.delete("dog")
p hash2.data
hash2.insert("rat")
p hash2.data

hash3 = Hash.new(5)
hash3.insert("うま")
p hash3.data
hash3.insert("かめ")
p hash3.data
hash3.insert("へび")
p hash3.data
hash3.insert("きつね")
p hash3.data
hash3.delete("かめ")
p hash3.data
