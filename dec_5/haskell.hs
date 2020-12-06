parse :: String -> Int
parse = foldl (\acc curr -> acc * 2 + f curr) 0

f 'B' = 1
f 'R' = 1
f _   = 0

solve1 :: [String] -> Int
solve1 = maximum . map parse

solve2 :: [String] -> Int
solve2 ids =
  (\(_, a, _) -> a) .
  head .
  filter (\(a, b, c) -> a `elem` idsp && b `notElem` idsp && c `elem` idsp) .
  zip3 [1 ..] [2 ..] $
  [3 ..]
  where
    idsp = map parse ids

solve :: [String] -> (Int, Int)
solve xs = (solve1 xs, solve2 xs)

main = interact ((++ "\n") . show . solve . lines)
