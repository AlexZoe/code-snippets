-- type annotation
getDigitsFromPositiveIntegerReversed :: Integer -> [Integer]
getDigitsFromPositiveIntegerReversed n
    | n > 9 = (n `mod` 10) : (getDigitsFromPositiveIntegerReversed (n `div` 10))
    | otherwise = [n]

-- `mod` is equivalent to mod n 10

main = putStrLn $ show (getDigitsFromPositiveIntegerReversed 32421)
