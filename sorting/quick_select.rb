# 配列のi番目からj番目の要素のうち、小さい方からm番目を返す

def quick_select(arr, i, j, m)
  p [arr, i, j, m] # 検証用

  if i >= j
    return arr[i]
  end

  pivot = find_pivot(arr, i, j)

  unless pivot
    return
  end

  # iからk-1にpivotより小さい要素、kからjにpivot以上の要素
  k = partition(arr, i, j, pivot)

  if m < (k - i)
    # 前半を走査
    return quick_select(arr, i, k-1, m)
  else
    # 後半を走査
    return quick_select(arr, k, j, m-k+i)
  end
end

# arrayのiからjまでを走査し、異なる2つの値が得られたら大きい方を返す

def find_pivot(arr, i, j)
  b = arr[i]

  (i+1).upto(j) do |m|
    if arr[m] > b
      return arr[m]
    elsif arr[m] < b
      return b
    end
  end

  false
end

# 左側にはpivotより小さい要素が、右側にはpivot以上の要素が入るようにする

def partition(arr, l, r, pivot)
  while true
    while arr[l] < pivot
      l += 1
    end
    while arr[r] >= pivot
      r -= 1
    end

    if l > r
      return l
    end

    arr[l], arr[r] = arr[r], arr[l]
  end
end

0.upto(6) do |i|
  p "#{i}番目は#{quick_select([6, 2, 8, 4, 7, 3, 9], 0, 6, i)}"
end

0.upto(7) do |i|
  p "#{i}番目は#{quick_select([18, 35, 76, 23, 49, 42, 31, 12], 0, 7, i)}"
end
