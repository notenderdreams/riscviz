basic:
    addi x21, x0, 111
    print x21
    beq x0, x0, end

_start:
    print x2
    beq x0, x0, basic

end:
   addi x1, x2, -200
   print x1