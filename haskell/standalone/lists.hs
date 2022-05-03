getDigitsFromPositiveInteger n
-- left nesting leads to O(n^2); should be right nested for O(n)
    | n > 9 = (getDigitsFromPositiveInteger (div n 10)) ++ [(n `mod` 10)]
    | otherwise = [n]

-- type annotation
getDigitsFromPositiveIntegerReversed :: Integer -> [Integer]
-- `mod` is equivalent to mod n 10
getDigitsFromPositiveIntegerReversed n
    | n > 9 = (n `mod` 10) : (getDigitsFromPositiveIntegerReversed (n `div` 10))
    | otherwise = [n]

pairs :: Int -> [(Int, Int)]
pairs num =
    [(x,y) | (x, y) <- zip [0..num] (reverse [0..num])]

pythagorean :: Int -> [(Int, Int, Int)]
pythagorean z =
    [(x, y, z) | x <- [1..z], y <- [1..z], x^2 + y^2 == z^2]


-- own solution
getFactors :: Int -> [Int]
getFactors num =
    [factor | factor <- [1..num-1], num `mod` factor == 0]

perfects :: Int -> [Int]
perfects num =
    [x | x <- [1..num], sum (getFactors x) == x]

-- model solution
factors :: Int -> [Int]
factors n = [x | x <- [1..n], n `mod` x == 0]

perfect :: Int -> Bool
-- "init" throws away the last item
perfect n = sum (init (factors n)) == n

perfectsModelSol n = [x | x <- [1..n], perfect x]


-- own solution
scalarProduct :: [Int] -> [Int] -> Int
scalarProduct xs ys =
    sum [xi*yi | (xi,yi) <- zip xs ys]

-- model solution
sPModelSol1 :: [Int] -> [Int] -> Int
-- "!!" is indexing into list
sPModelSol1 xs ys = sum [xs !! i * ys !! i | i <- [0..n-1]]
                   where n = length xs

sPModelSol2 :: [Int] -> [Int] -> Int
sPModelSol2 xs ys = sum [x*y | (x,y) <- zip xs ys]


main = putStrLn $ show (getDigitsFromPositiveIntegerReversed 32421)
