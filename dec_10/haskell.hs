import Data.List

solve2 :: [Int] -> Int
solve2 from = helper from 0

helper :: [Int] -> Int -> Int
helper from prev
  | null options = 1
  | otherwise = sum next
  where
    options = filter (\x -> x >= prev && x <= prev + 3) from
    next = map (\x -> helper (remove x from) x) options

remove :: Eq a => a -> [a] -> [a]
remove x xs = take i xs ++ drop (i + 1) xs
  where Just i = elemIndex x xs

main :: IO ()
main = interact ((++ "\n") . show . solve2 . sort . map read . lines)
