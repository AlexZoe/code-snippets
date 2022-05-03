safetailCond :: [a] -> [a]
safetailCond xs = if null xs then
                      []
                  else
                      tail xs

safetailGuard :: [a] -> [a]
safetailGuard xs
    | null xs   = []
    | otherwise = tail xs

safetailPattern :: [a] -> [a]
safetailPattern [] = []
safetailPattern (_:xs) = xs

main = print("Test")
