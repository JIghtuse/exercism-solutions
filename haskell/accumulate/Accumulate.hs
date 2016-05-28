module Accumulate (accumulate) where

accumulate fun xs = [ fun(e) | e <- xs ]
