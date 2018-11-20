def heap_sort(arr)
  n = arr.size

  # 半順序性の確立
  (n/2-1).downto(0) do |i|
    pushdown(arr, i, n-1)
  end

  # 最小値の取得
  (n-1).downto(1) do |i|
    p "#{arr[0]}と#{arr[i]}を入れ替えます" # 経過表示
    arr[0], arr[i] = arr[i], arr[0] # ヒープの先頭から最小値を取り除く
    p arr # 経過表示

    pushdown(arr, 0, i-1) # 半順序性の回復
  end

  arr

end

def pushdown(arr, b, e)
  i = b
  while i <= (e-1)/2
    if (2 * i + 1) == e # 範囲に左の子だけ含まれる場合
       j = 2 * i + 1
    else
       j = arr[2 * i + 1] < arr [2 * i + 2] ? 2 * i + 1 : 2 * i + 2
    end

    if arr[j] < arr[i]
      p "#{arr[i]}と#{arr[j]}を入れ替えます" # 経過表示
      arr[i], arr[j] = arr[j], arr[i]
      p arr # 経過表示

      i = j
    else
      return
    end
  end
end

p heap_sort([12, 23, 24, 45, 18, 11, 34])
p heap_sort([18, 35, 76, 23, 49, 42, 31, 12])
