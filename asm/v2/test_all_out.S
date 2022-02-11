#include "ruledef.S"

    ldav 1
    txb
    ldav 0
loop:
    out
    add
    jc end
    jmp loop
end:
    hlt
