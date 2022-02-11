; ruledef for PuttPCv3
#once

#ruledef
{
    ldav  {value}             => 0x01 @ value`8
    ldam  {address}           => 0x02 @ address`8
    sta   {address}           => 0x03 @ address`8
    txab                      => 0x04
    ldbv  {value}             => 0x05 @ value`8
    ldbm  {address}           => 0x06 @ address`8
    stb   {address}           => 0x07 @ address`8
    txba                      => 0x08
    add                       => 0x10
    addav {value}             => 0x11 @ value`8
    addam {address}           => 0x12 @ address`8
    addvb {value}             => 0x13 @ value`8
    addvv {value}   {value}   => 0x14 @ value`8 @ value`8
    addvm {value}   {address} => 0x15 @ value`8 @ address`8
    addmb {address}           => 0x16 @ address`8
    addmv {address} {value}   => 0x17 @ address`8 @ value`8
    addmm {address} {address} => 0x18 @ address`8 @ address`8
    sub                       => 0x20
    subav {value}             => 0x21 @ value`8
    subam {address}           => 0x22 @ address`8
    subvb {value}             => 0x23 @ value`8
    subvv {value}   {value}   => 0x24 @ value`8 @ value`8
    subvm {value}   {address} => 0x25 @ value`8 @ address`8
    submb {address}           => 0x26 @ address`8
    submv {address} {value}   => 0x27 @ address`8 @ value`8
    submm {address} {address} => 0x28 @ address`8 @ address`8
    jmp   {address}           => 0xD0 @ address`8
    jz    {address}           => 0xD1 @ address`8
    jnz   {address}           => 0xD2 @ address`8
    jc    {address}           => 0xD3 @ address`8
    jnc   {address}           => 0xD4 @ address`8
    out                       => 0xE0
    hlt                       => 0xFF
}
