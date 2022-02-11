#include "ruledef.S"

loop:
    ldam data
    txb
    ldam (data+1)
    sta data
    out
    add
    sta (data+1)
    jc end
    jmp loop
end:
    hlt


data:
    #d 0`8, 1`8
