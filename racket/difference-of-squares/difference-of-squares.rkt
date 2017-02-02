#lang racket

;; Natural -> Natural
;; produces sum of the squares of the first n natural numbers
(define (sum-of-squares n)
  (if (= n 0)
      0
      (+ (expt n 2)
         (sum-of-squares (sub1 n)))))

;; Natural -> Natural
;; produces square of sum of the first n natural numbers
(define (square-of-sums n)
  (expt
   (/ (* n (add1 n)) 2)
   2))

;; Natural -> Natural
;; produces difference between square of sum and sum of squares
;; of the first n natural numbers
(define (difference n)
  (- (square-of-sums n)
     (sum-of-squares n)))

(provide sum-of-squares square-of-sums difference)

