module DNA where

nucleotideToRNA 'C' = 'G'
nucleotideToRNA 'G' = 'C'
nucleotideToRNA 'T' = 'A'
nucleotideToRNA 'A' = 'U'

toRNA :: [Char] -> [Char]
toRNA = map nucleotideToRNA
