#include "ruledef.S"

    ldav 0
    ldbv 1
loop:
    out
    add
    jnc loop
    hlt
