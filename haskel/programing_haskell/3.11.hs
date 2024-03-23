-- 3.11.1
-- ['a', 'b', 'c'] :: [Char]
-- ('a', 'b', 'c') :: (Char, Char, Char)
-- [(False, '0'), (True, '1')] :: [(Bool, Char)]
-- ([False, True], ['0', '1']) :: ([Bool], [Char])
-- [tail, init, reverse] :: [[a] -> [a]]

-- 3.11.2
bools :: [Bool]
bools = [True, False]

nums :: [[Int]]
nums = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]

add :: Int -> Int -> Int -> Int
add a b c = a + b + c

copy :: a -> (a, a)
copy a = (a, a)

apply :: (a -> b) -> a -> b
apply a b = a b
