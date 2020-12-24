import Data.List
import Debug.Trace

input = [3, 1, 8, 9, 4, 6, 5, 7, 2]

step :: Int -> [Int] -> [Int]
step len l@(current : r1 : r2 : r3 : rest) = take (idx + 1) rest ++ [r1, r2, r3] ++ drop (idx + 1) rest ++ [current]
  where
    r = [r1, r2, r3, current]
    candidates = reverse [1 .. current] ++ reverse [current + 1 .. len]
    dest = head . filter (`notElem` r) $ candidates
    i = elemIndex dest rest
    Just idx = {-trace ("\nr: " ++ show r ++ "\ncandidates: " ++ show candidates ++ "\ndest: " ++ show dest ++ "\ni: " ++ show i)-} i

repeatStep :: Int -> [Int] -> [Int]
repeatStep times list = foldr (\_ l -> step (length list) l) list [1 .. times]

parseResult :: [Int] -> Int
parseResult [] = 0
parseResult (1 : xs) = foldl (\acc curr -> acc * 10 + curr) 0 (traceShowId xs)
parseResult (x : xs) = parseResult (xs ++ [x])

solveExample = parseResult . repeatStep 100 $ [3, 8, 9, 1, 2, 5, 4, 6, 7]

solve1 = parseResult . repeatStep 100 $ input

solve2 = a * b
  where
    done = repeatStep 10000000 (input ++ [10 .. 1000000])
    Just i = elemIndex 1 done
    a = traceShowId (done !! i + 1)
    b = traceShowId (done !! i + 2)

main = print (show solve1 ++ " " ++ show solve2)
