#lang racket

(define (leap-year? year)
  (define (year-has-factor? n)
    (zero? (modulo year n)))
  (or
    (year-has-factor? 400)
    (and (year-has-factor? 4)
         (not (year-has-factor? 100)))))

(provide leap-year?)
