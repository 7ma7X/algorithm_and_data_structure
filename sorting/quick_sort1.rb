def quick_sort(arr)
  if arr.size <= 1
    return arr
  end

  pivot = [arr[0], arr[1]].max
  
  new_array_0 = []
  new_array_1 = []

  arr.each do |x|
    if x < pivot
      new_array_0 << x
    elsif
      new_array_1 << x
    end
  end

  new_array_0 = quick_sort(new_array_0)
  new_array_1 = quick_sort(new_array_1)

  p "#{arr}を#{new_array_0}と#{new_array_1}に分割" # 経過表示用

  new_array_0.concat(new_array_1)
end

quick_sort([5, 6, 3, 9, 2, 8, 4, 7])
quick_sort([18, 35, 76, 23, 49, 42, 31, 12])
