; ruledef for PuttPCv2
#once

#ruledef
{
    ldav  {value}   => 0x1 @ value`4
    ldam  {address} => 0x2 @ address`4
    sta   {address} => 0x3 @ address`4
    txb             => 0x4 @ 0x0
    add             => 0x5 @ 0x0
    addv  {value}   => 0x6 @ value`4
    addm  {address} => 0x7 @ address`4
    sub             => 0x8 @ 0x0
    subv  {value}   => 0x9 @ value`4
    subm  {address} => 0xA @ address`4
    jmp   {address} => 0xB @ address`4
    jz    {address} => 0xC @ address`4
    jc    {address} => 0xD @ address`4
    out             => 0xE @ 0x0
    hlt             => 0xF @ 0x0
}
