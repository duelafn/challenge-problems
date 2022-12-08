{-#  LANGUAGE TypeApplications  #-}

import System.Environment
import Data.List.Split

data Load = Load [Int]
data Expedition = Expedition [Load]


main :: IO ()
main = do
  args <- getArgs
  d <- fmap parse (today args)
  putStrLn ("Part 1: " ++ show (solve1 d))
  putStrLn ("Part 2: " ++ show (solve2 d))

today :: [String] -> IO String
today [] = today ["01.in"]
today (fname:_) = readFile fname

parse :: String -> Expedition
parse s = Expedition (map (Load . ((map (read @Int)) . words)) (splitOn "\n\n" s))


solve1 :: Expedition -> Int
solve1 (Expedition []) = 0
solve1 (Expedition [Load l]) = sum l
solve1 (Expedition (l:ls)) = max (solve1 (Expedition [l])) (solve1 (Expedition ls))

solve2 :: Expedition -> Int
solve2 e = solve2_ 3 e

solve2_ :: Int -> Expedition -> Int
solve2_ 0 _  = 0
solve2_ _ (Expedition []) = 0
solve2_ _ (Expedition [Load a]) = sum a
solve2_ n (Expedition (l:ls)) = max ((solve2_ 1 (Expedition [l])) + (solve2_ (n-1) (Expedition ls))) (solve2_ n (Expedition ls))
