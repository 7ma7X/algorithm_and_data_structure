# 検索文字列を逆向きに見た時何文字目に出てくるかをハッシュにする
def make_hash(str)
  n = str.length
  h = Hash.new([])

  (0...n).each do |i|
    unless h.has_key?(str[n-1-i])
      h[str[n-1-i]] = i
    end
  end

  h
end

# 検索文字列の照合中（逆向き）に不一致があったとき、いくつ進めれば良いかを表にする
def make_table(str)
  n = str.length
  t = Array.new(n)

  (0...n).each do |i|
    shift_num = 1
    b = true
    while shift_num <= n-1
      if (str[n-i-shift_num...n-shift_num] == str[n-i...n]) && (str[n-i-shift_num-1] != str[n-i-1])
        t[i] = shift_num
        b = false
        break
      end
      shift_num += 1
    end 

    if b
      t[i] = n
    end
  end

  t.reverse()
end

# 表を一つしか使わない基本的なBM法
def basic_kmp(text, pattern)
  skip = make_hash(pattern)
  skip2 = make_table(pattern)
  n = text.length
  m = pattern.length

  # iは文書中の位置（前から走査）
  i = 0

  while i <= n - m 
    # jは検索文字列中の位置（後ろから走査）
    j = m - 1

    while (j >= 0) && text[i+j] == pattern[j]
      j -= 1
      if j == -1
        return i 
      end
    end

    p "i=#{i}, j=#{j} で skip" # 過程の表示
    p "skip: #{(skip.has_key?(text[i+j]) ? skip[text[i+j]] : m) + j - m + 1} と 1 の大きい方" # 過程の表示
    i = i + [(skip.has_key?(text[i+j]) ? skip[text[i+j]] : m) + j - m + 1, 1].max
  end

  false
end

def kmp(text, pattern)
  skip = make_hash(pattern)
  skip2 = make_table(pattern)
  n = text.length
  m = pattern.length

  # iは文書中の位置（前から走査）
  i = 0

  while i <= n - m 
    # jは検索文字列中の位置（後ろから走査）
    j = m - 1

    while (j >= 0) && text[i+j] == pattern[j]
      j -= 1
      if j == -1
        return i 
      end
    end

    p "i=#{i}, j=#{j} で skip" # 過程の表示
    p "skip: #{(skip.has_key?(text[i+j]) ? skip[text[i+j]] : m) + j - m + 1}, skip2: #{skip2[j]}" # 過程の表示
    i = i + [(skip.has_key?(text[i+j]) ? skip[text[i+j]] : m) + j - m + 1, skip2[j]].max
  end

  false
end

p make_hash("abcabd")

p make_table("abcdbc")

p make_hash("ひくまふくま")
p make_table("ひくまふくま")
p kmp("くくまくまふくまひくまひくまふくまひ", "ひくまふくま")

p make_hash("たのみもみのみ")
p make_table("たのみもみのみ")
p kmp("はんなりののみすぎのみみたのみもみのみ", "たのみもみのみ")

p make_hash("DAACAA")
p basic_kmp("DAADAAACAADA", "DAACAA")