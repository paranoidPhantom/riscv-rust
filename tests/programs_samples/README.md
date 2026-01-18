# Примеры программ

В данной папке вы можете найти некоторое количество программ, на которых вы можете проверить
работу вашей реализации процессора:

## [`arith.dat`](./arith.dat)

Проверка работы reg-reg и reg-imm арифметических операций.

```
addi s0, zero, 7
addi s1, zero, 8
add s2, s1, s0
sub s3, s1, s0
and s4, s1, s0
or s5, s1, s0
slt s6, s1, s0
```

Ожидаемый результат:
```
Register:          8, value:          7
Register:          9, value:          8
...
Register:         18, value:         15
Register:         19, value:          1
Register:         20, value:          0
Register:         21, value:         15
Register:         22, value:          0
```

## [`memory.dat`](./memory.dat)

Проверка работы с памятью.

```
addi t0, zero, 2047
sw t0, 0(zero)
lw t1, 0(zero)
addi t4, zero, 511
sw t4, 32(zero)
```

Ожидаемый результат:

```
Register:          5, value:       2047
Register:          6, value:       2047
...
Register:         29, value:        511
...
Addr:          0, value:       2047
...
Addr:          32, value:        511
```

## [`beq.dat`](./beq.dat)

Проверка работы инструкции beq.

```
addi s0, zero, 8
addi s1, zero, 8
beq s0, s1, eq # eq = 12 (relative)
add t1, s0, s1
beq zero, zero, end # end = 8 (relative)
eq:
add t0, s0, s1
end:
```

Ожидаемый результат:
```
...
Register:          5, value:          16
...
Register:          8, value:          8
Register:          9, value:          8
```

## [`goto.dat`](./goto.dat)

```
jal s0, label2 # label2 = 8 (relative)
label1:
jal s2, end # end = 12 (relative)
label2:
addi t2, zero, 4
jalr s1, 0(t2) # jump to label1 (PC = 4)
end:
```

Ожидаемый результат:
```
Register:           8, value: 4
Register:           9, value: 16
...
Register:          18, value: 8
```

## [`bne1.dat`](./bne1.dat)

```
addi s0, zero, 9
addi s1, zero, 8
bne s0, s1, neq # neq = 12 (relative)
add t1, s0, s1
beq zero, zero, end # end = 8 (relative)
neq:
sub t0, s0, s1
end:
```
Ожидаемый результат:
```
Register:          5, value: 1
...
Register:          8, value: 9
Register:          9, value: 8
```

## [`bne2.dat`](./bne2.dat)

```
addi s0, zero, 8
addi s1, zero, 8
bne s0, s1, neq # neq = 12 (relative)
add t1, s0, s1
beq zero, zero, end, # end = 8 (relative)
neq:
sub $t0, $s0, $s1
end:
```
Ожидаемый результат:
```
Register:          6, value: 16
...
Register:          8, value: 8
Register:          9, value: 8
```
