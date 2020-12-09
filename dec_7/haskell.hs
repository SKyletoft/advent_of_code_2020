import Data.List (nub)

solve1 :: [(String, [String])] -> Int
solve1 parsed = (\x -> x - 1) . length . f [] $ ["shiny gold"]
  where
    f :: [String] -> [String] -> [String]
    f can [] = can
    f can (n : new) = f (nub (n : can)) (nub (toNew ++ new))
      where
        toNew = map fst . contains parsed $ n

solve2 :: [(String, [(Int, String)])] -> Int
solve2 parsed = sum . f [] ["shiny gold"]
  where
    f can [] = can
    f can (n@(name, vec) : new) = f (nub (n : can)) (nub (toNew ++ new))
      where
        toNew = map (\(n,v) -> (n, map (\(a,b) -> a * ) v)) . containsWithCounts parsed $ name

contains :: [(String, [String])] -> String -> [(String, [String])]
contains source target = filter (\(_, xs) -> target `elem` xs) source

containsWithCounts :: [(String, [(Int, String)])] -> String -> [(String, [(Int, String)])]
containsWithCounts source target = filter (\(_, xs) -> target `elem` map snd xs) source

parse :: String -> (String, [String])
parse line
  | empty = (name, [])
  | otherwise = (name, rest)
  where
    wds = words line
    name = unwords . take 2 $ wds
    rest = map (unwords . take 2 . drop 1) . chunksOf4 . drop 4 $ wds
    empty = wds !! 4 == "no"

parseWithCounts :: String -> (String, [(Int, String)])
parseWithCounts line
  | empty = (name, [])
  | otherwise = (name, rest)
  where
    wds = words line
    name = unwords . take 2 $ wds
    rest = map wordsParse . chunksOf4 . drop 4 $ wds
    wordsParse [a, b, c, _] = (read a, unwords [b, c])
    empty = wds !! 4 == "no"

chunksOf4 :: [a] -> [[a]]
chunksOf4 (a : b : c : d : xs) = [a, b, c, d] : chunksOf4 xs
chunksOf4 _ = []

main = interact ((++ "\n") . show . solve1 . map parse . lines)

--main = interact (unlines . map (show . parse) . lines)
