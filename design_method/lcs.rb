require 'set'

def lcs(x, y)
  n = x.length
  m = y.length
  res = Array.new(n+1).map{Array.new(m+1)}

  (0..n).each do |i|
    (0..m).each do |j|
      if i == 0 || j == 0
        res[i][j] = 0
      elsif 
        d = x[i-1] == y[j-1] ? 1 : 0
        res[i][j] = [res[i-1][j], res[i][j-1], res[i-1][j-1] + d].max
      end
    end
  end

  # 表の確認用
  (1..n).each do |i|
    (1..m).each { |j| print "#{res[i][j]} " }
    puts
  end

  hoge = Set.new
  back_trace(x, y, n, m, res, "", hoge)
  hoge
end

def back_trace(x, y, i, j, res, tmp, res_str_set)

  if i == 0 || j == 0
    res_str_set << tmp
  elsif
    d = x[i-1] == y[j-1] ? 1 : 0
    if res[i-1][j-1] + d == res[i][j]
      if d == 1
        back_trace(x, y, i-1, j-1, res, x[i-1] + tmp, res_str_set)
      elsif
        back_trace(x, y, i-1, j-1, res, tmp, res_str_set)
      end
    end
    if res[i-1][j] == res[i][j]
      back_trace(x, y, i-1, j, res, tmp, res_str_set)
    end
    if res[i][j-1] == res[i][j]
      back_trace(x, y, i, j-1, res, tmp, res_str_set)
    end
  end
end

puts lcs("abcbdab", "bdcaba")
puts lcs("まりものおまもり", "もりのおまわりさん")
