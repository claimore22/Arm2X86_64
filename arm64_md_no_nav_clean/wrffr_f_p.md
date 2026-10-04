## WRFFR

Write the first-fault register This instruction reads the source predicate register, and writes the value in the register to the first-fault register (FFR). This instruction is intended to restore a saved FFR and is not recommended for general use by applications. This instruction requires that the source predicate contains a monotonic predicate value, in which starting from bit 0 there are zero or more 1 bits, followed only by 0 bits in any remaining bit positions. If the source is not a monotonic predicate value, then the resulting value in the FFR will be UNPREDICTABLE. It is not possible to generate a non-monotonic value in FFR when using SETFFR followed by first-fault or non-fault loads. This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Pn);

### Operation

[CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let operand : bits(PL) = P{}(n); let hsb : integer{} = HighestSetBit(operand); if hsb < 0 \|\| IsOnes(operand[hsb:0]) then [FFR](shared_pseudocode.html#accessor_FFR_1){PL}() = operand; else // not a monotonic predicate [FFR](shared_pseudocode.html#accessor_FFR_1){PL}() = ARBITRARY : bits(PL); end;
