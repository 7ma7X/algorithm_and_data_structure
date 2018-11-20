def merge(arr0, arr1)
  i, j, res = 0, 0, []
  l0 = arr0.size
  l1 = arr1.size

  while i != l0 || j != l1
    if j == l1
      res << arr0[i]
      i += 1
      next
    end

    if i == l0
      res << arr1[j]
      j += 1
      next
    end

    if arr0[i] < arr1[j]
      res << arr0[i]
      i += 1
    elsif
      res << arr1[j]
      j += 1
    end
  end

  res
end

def merge_sort(arr)
  l = arr.size

  if l <= 1
    return arr
  end

  p "#{arr}を#{arr[0...(l/2)]}と#{arr[(l/2)...l]}に分割" # 経過表示

  arr0 = merge_sort(arr[0...(l/2)])
  arr1 = merge_sort(arr[(l/2)...l])

  p "#{merge(arr0, arr1)}に並び替え" # 経過表示
  merge(arr0, arr1)
end

p merge_sort([5, 7, 9, 4, 6, 8, 3, 2])
p merge_sort([18, 35, 76, 23, 49, 42, 31, 12])
