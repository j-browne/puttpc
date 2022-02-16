#include "ruledef.S"

    mov %a 0
    mov %b 1
loop:
    out
    add %a %b
    jnc ${loop}
    hlt
