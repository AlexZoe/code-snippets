isGreater :: Ord a => a -> a -> Bool
isGreater a b
    | a > b = True
    | otherwise = False

-- E.g.:
-- print (isGreater 'b' 'c')
-- print (isGreater 5 2)

main = (print (isGreater 5 2))
