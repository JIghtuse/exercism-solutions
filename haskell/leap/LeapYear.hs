module LeapYear (isLeapYear) where

isLeapYear :: Int -> Bool
isLeapYear year = year `mod` 400 == 0 || year `mod` 100 /= 0 && year `mod` 4 == 0
