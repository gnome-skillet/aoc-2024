(define mylist (let ((p (open-input-file "/Users/carlo/src/aoc/aoc-2024/aoc/inputs/day1b.txt")))
  (let f ((x (read p)))
    (if (eof-object? x)
        (begin
          (close-input-port p)
          '())
        (cons x (f (read p)))))))

(define (lhs lst)
  (define (iter lst lhs)
    (cond
      ((empty? lst) lhs)
      (else (iter (cddr lst) (append lhs (list (car lst)))))))
    (iter lst '()))

(define (rhs lst)
  (define (iter lst rhs)
    (cond
      ((empty? lst) rhs)
      (else (iter (cddr lst) (append rhs (list (cadr lst)))))))
    (iter lst '()))

(define (abs-diff-sum lst)
    (define (iter l r sum)
      (cond
       ((empty? l) sum)
       (else (iter (cdr l) (cdr r) (+ sum (abs (- (car l) (car r))))))))
    (iter (sort (lhs lst) <) (sort (rhs lst) <) 0))

(define result (abs-diff-sum mylist))
