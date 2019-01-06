# 検索文字列の照合中に不一致があったとき、いくつ進めれば良いかを表にする
def make_table(str)
  n = str.length
  t = Array.new(n)

  (0...n).each do |i|
    shift_num = 1
    b = true
    while shift_num <= i
      if (str[0...i-shift_num] == str[shift_num...i]) && (str[i-shift_num] != str[i])
        t[i] = shift_num
        b = false
        break
      end
      shift_num += 1
    end 

    if b
      t[i] = i+1
    end
  end

  t
end

def kmp(text, pattern)
  skip = make_table(pattern)
  n = text.length
  m = pattern.length

  # iは文書中の位置, jは検索文字列中の位置
  i = 0
  j = 0

  while i <= n - m
    while j < m && text[i+j] == pattern[j]
      j += 1
      if j == m 
        return i
      end
    end

    p "i=#{i}, j=#{j} で skip=#{skip[j]}" # 過程の表示
    
    i += skip[j]
    j -= skip[j]

    if j == -1 
      j = 0
    end
  end

  false
end

p make_table("abcabd")
p make_table("aabaab")

p make_table("くまくたくまた")
p kmp("とくままくまくたたくまくまくたくまたま", "くまくたくまた")

p make_table("ささうささん")
p kmp("さあささうささよささうささうささん", "ささうささん")

p make_table("DADDAC")
p kmp("DDADADDADDAC", "DADDAC")
