; ruledef for PuttPCv3
#once

#ruledef
{
    mov %a         %b         => 0x00
    mov %a         {value}    => 0x01 @ value`8
    mov %a         ${address} => 0x02 @ address`8
    mov %b         %a         => 0x03
    mov %b         {value}    => 0x04 @ value`8
    mov %b         ${address} => 0x05 @ address`8
    mov ${address} %a         => 0x06 @ address`8
    mov ${address} %b         => 0x07 @ address`8
    mov ${address} {value}    => 0x08 @ address`8 @ value`8
    mov ${address} ${address} => 0x09 @ address`8 @ address`8
    add %a         %b         => 0x10
    add %a         {value}    => 0x11 @ value`8
    add %a         ${address} => 0x12 @ address`8
    add {value}    %b         => 0x13 @ value`8
    add {value}    {value}    => 0x14 @ value`8 @ value`8
    add {value}    ${address} => 0x15 @ value`8 @ address`8
    add ${address} %b         => 0x16 @ address`8
    add ${address} {value}    => 0x17 @ address`8 @ value`8
    add ${address} ${address} => 0x18 @ address`8 @ address`8
    sub %a         %b         => 0x20
    sub %a         {value}    => 0x21 @ value`8
    sub %a         ${address} => 0x22 @ address`8
    sub {value}    %b         => 0x23 @ value`8
    sub {value}    {value}    => 0x24 @ value`8 @ value`8
    sub {value}    ${address} => 0x25 @ value`8 @ address`8
    sub ${address} %b         => 0x26 @ address`8
    sub ${address} {value}    => 0x27 @ address`8 @ value`8
    sub ${address} ${address} => 0x28 @ address`8 @ address`8
    jmp ${address}            => 0xD0 @ address`8
    jz  ${address}            => 0xD1 @ address`8
    jnz ${address}            => 0xD2 @ address`8
    jc  ${address}            => 0xD3 @ address`8
    jnc ${address}            => 0xD4 @ address`8
    out                       => 0xE0
    hlt                       => 0xFF
}
