import qualified Data.List
import Data.Maybe (isJust)
import Data.Vector (Vector, fromList)
import qualified Data.Vector
import Debug.Trace

solve :: Int -> [Int] -> Vector Int
solve i xs = Data.Vector.constructrN toTake (next (reverse xs))
  where
    toTake = i - length xs

next :: [Int] -> Vector Int -> Int
next l v
  | isJust maybeIndex = i + 1
  | isJust maybeIndexFallback = i + length v
  | otherwise = 0
  where
    first
      | null v = Data.List.head l
      | otherwise = Data.Vector.head v
    maybeIndex
      | length v > 1 = Data.Vector.elemIndex first . Data.Vector.drop 1 $ v
      | otherwise = Nothing
    maybeIndexFallback
      | null v = Data.List.elemIndex first . drop 1 $ l
      | otherwise = Data.List.elemIndex first l
    containsFirst = isJust maybeIndex
    Just i
      | isJust maybeIndex = maybeIndex
      | isJust maybeIndexFallback = maybeIndexFallback

replace :: Eq a => a -> a -> [a] -> [a]
replace _ _ [] = []
replace from to (x : xs)
  | x == from = to : replace from to xs
  | otherwise = x : replace from to xs

main :: IO ()
main = interact ((++ "\n") . show . Data.Vector.head . solve 300000 . map read . words . replace ',' ' ')
