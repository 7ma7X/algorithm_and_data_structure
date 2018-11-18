def bubble_sort(arr)
  n = arr.size

  (0..n-2).each do |i|
    (i+1..n-1).reverse_each do |j|
      if arr[j-1] > arr[j]
        arr[j-1], arr[j] = arr[j], arr[j-1]
        p arr  # 経過確認用
      end 
    end
  end
end

bubble_sort([4, 5, 2, 6, 3])
bubble_sort([18, 35, 76, 23, 49, 42, 31, 12])
