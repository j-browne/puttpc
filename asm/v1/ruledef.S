; ruledef for PuttPCv1
#once

#ruledef
{
    ldav  {value}   => 0x1 @ value`4
    ldam  {address} => 0x2 @ address`4
    sta   {address} => 0x3 @ address`4
    txb             => 0x4 @ 0x0
    addm  {address} => 0x5 @ address`4
    subm  {address} => 0x6 @ address`4
    jmp   {address} => 0x7 @ address`4
    jz    {address} => 0x8 @ address`4
    jc    {address} => 0x9 @ address`4
    out             => 0xE @ 0x0
    hlt             => 0xF @ 0x0
}
