import           Data.Char
import           Data.List

splitOn :: String -> String -> [String]
splitOn divider str = splitOn' (reverse str) [""]
  where
    splitOn' :: String -> [String] -> [String]
    splitOn' "" xs = xs
    splitOn' str xs
      | divider `isPrefixOf` str =
        splitOn' (drop (length divider) str) ("" : xs)
    splitOn' (s:str) (x:xs) = splitOn' str ((s : x) : xs)

solve1 :: String -> Int
solve1 = sum . map (length . filter isLetter . nub) . splitOn "\n\n"

solve2 :: String -> Int
solve2 =
  sum .
  map
    (\s ->
       length .
       filter (\c -> all (c `elem`) . lines $ s) . nub . filter isLetter $
       s) .
  splitOn "\n\n"

solve :: String -> (Int, Int)
solve s = (solve1 s, solve2 s)

main = interact ((++ "\n") . show . solve)
