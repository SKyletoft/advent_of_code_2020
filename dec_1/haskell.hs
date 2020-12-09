solve1 :: [Int] -> Int
solve1 xs = head [lhs * rhs | lhs <- xs, rhs <- xs, lhs + rhs == 2020]

solve2 :: [Int] -> Int
solve2 xs = head [lhs * rhs * ths | lhs <- xs, rhs <- xs, ths <- xs, ths + lhs + rhs == 2020]

main :: IO ()
main = interact ((++ "\n") . show . solve2 . map read . lines)
