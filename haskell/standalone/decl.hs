-- similar to typedef in C
type Vector = (Int, Int)
type Pair a = (a, a)

mult :: Pair Int -> Int
mult (x,y) = x * y

vector2d :: Vector -> Vector -> Vector
vector2d (x_head,y_head) (x_tail, y_tail) =
        (x_head - x_tail, y_head - y_tail)

-- completely new type with constructors following
data Answer = Yes | No | Unknown

flip :: Answer -> Answer
flip Yes = No
flip No = Yes
flip Unknown = Unknown

-- constructors can also take arguments
data Shape = Circle Float |
             Rect Float Float

-- recursive data decl
data Expr = Val Int
    | Add Expr Expr
    | Mul Expr Expr

data Tree a = Leaf a
    | Node (Tree a) (Tree a)

main = print "test"
