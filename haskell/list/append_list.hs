getDigitsFromPositiveInteger n
-- left nesting leads to O(n^2); should be right nested for O(n)
    | n > 9 = (getDigitsFromPositiveInteger (div n 10)) ++ [(n `mod` 10)]
    | otherwise = [n]

main = putStrLn $ show (getDigitsFromPositiveInteger 123456)
