myAnd :: [Bool] -> Bool
myAnd [] = True
myAnd (x:xs) = x && myAnd xs

myConcat :: [[a]] -> [a]
myConcat [] = []
myConcat (xs:xss) = xs ++ myConcat xss

myReplicate :: Int -> a -> [a]
myReplicate n x
    | n <= 0 = []
    | otherwise  = [x] ++ myReplicate (n - 1) x

myIdx :: [a] -> Int -> a
myIdx (x:_)  0 = x
myIdx (_:xs) n = myIdx xs (n - 1)

myElem :: Eq a => a -> [a] -> Bool
myElem a [] = False
myElem a (x:xs)
    | x == a    = True
    | otherwise = myElem a xs

myInsert :: Int -> [Int] -> [Int]
myInsert item [] = [item]
myInsert item (x:xs)
    | item > x  = [x] ++ myInsert item xs
    | otherwise = [item] ++ [x] ++ xs

--model solution
myInsertMSol :: Int -> [Int] -> [Int]
myInsertMSol x [] = [x]
myInsertMSol x (y:ys) = if x <= y then
                            x : y : ys
                        else
                            y : myInsertMSol x ys

insertSort :: [Int] -> [Int]
insertSort [] = []
insertSort (x:xs) = myInsert x (insertSort xs)

merge :: [Int] -> [Int] -> [Int]
merge xs [] = xs
merge [] ys = ys
merge (x:xs) (y:ys)
    | x <= y    = x : merge xs (y:ys)
    | otherwise = y : merge (x:xs) ys

halve :: [Int] -> ([Int],[Int])
halve [] = ([], [])
halve xs = (take split xs, drop split xs)
           where
                split = (length xs) `div` 2
                num_items = length xs

mergeSort :: [Int] -> [Int]
mergeSort []  = []
mergeSort [x] = [x]
mergeSort xs = merge (mergeSort firstHalf) (mergeSort secondHalf)
               where (firstHalf, secondHalf) = halve xs

main = print "test"
