solve1 :: [Int] -> Int
solve1 nums
  | not valid = n
  | otherwise = solve1 . drop 1 $ nums
  where
    n = nums !! 25
    valid = isSumOf (take 25 nums) n

isSumOf :: [Int] -> Int -> Bool
isSumOf nums n
  | length nums /= 25 = error ("wrong length in isSumOf! " ++ show (length nums) ++ " " ++ show nums)
  | otherwise = or $ [a + b == n | a <- nums, b <- nums, a /= b]

main = interact ((++ "\n") . show . solve1 . map read . lines)