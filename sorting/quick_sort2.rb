def quick_sort(arr, i, j)
  if i >= j
    return
  end

  pivot = find_pivot(arr, i, j)

  unless pivot
    return
  end
 
  p "pivot=#{pivot}" # 経過確認用

  k = partition(arr, i, j, pivot)

  p arr # 経過確認用

  quick_sort(arr, i, k-1)
  quick_sort(arr, k, j)
end

# arrayのiからjまでを走査し、異なる2つの値が得られたら大きい方を返す

def find_pivot(arr, i, j)
  b = arr[i]

  (i+1..j).each do |m|
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


quick_sort([6, 2, 8, 4, 7, 3, 9], 0, 6)
