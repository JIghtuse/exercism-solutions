#lang racket

(define NUMBER-OF-CELLS 64)

(define (square cell)
  (expt 2 (sub1 cell)))

(define (total)
  (define (total-acc acc cell)
    (if (> cell NUMBER-OF-CELLS)
      acc
      (total-acc (+ acc (square cell))
                 (add1 cell))))
  (total-acc 0 1))

(provide square total)
