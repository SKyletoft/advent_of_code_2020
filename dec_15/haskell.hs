import Data.List (elemIndex)
import Data.Maybe (isJust)
import Debug.Trace (traceShowId)

solve :: Int -> [Int] -> [Int]
solve i xs = helper toTake . reverse $ xs
  where
    toTake = i - length xs

helper :: Int -> [Int] -> [Int]
helper 0 xs = xs
helper l (x : xs)
  | containsX = helper next (i + 1 : x : xs)
  | otherwise = helper next (0 : x : xs)
  where
    next = l - 1
    maybeIndex = elemIndex x xs
    containsX = isJust maybeIndex
    Just i = maybeIndex

replace :: Eq a => a -> a -> [a] -> [a]
replace _ _ [] = []
replace from to (x : xs)
  | x == from = to : replace from to xs
  | otherwise = x : replace from to xs

main :: IO ()
main = interact ((++ "\n") . show . head . solve 300000 . map read . words . replace ',' ' ')
