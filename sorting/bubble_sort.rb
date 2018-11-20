def bubble_sort(arr)
  n = arr.size

  0.upto(n-2) do |i|
    (n-1).downto(i+1) do |j|
      if arr[j-1] > arr[j]
        arr[j-1], arr[j] = arr[j], arr[j-1]
        p arr  # 経過確認用
      end 
    end
  end
end

bubble_sort([4, 5, 2, 6, 3])
bubble_sort([18, 35, 76, 23, 49, 42, 31, 12])
