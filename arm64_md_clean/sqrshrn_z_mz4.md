## SQRSHRN

Multi-vector signed saturating rounding shift right narrow by immediate to interleaved integer This instruction shifts right by an immediate value the signed integer value in each element of the four source vectors, and places the four-way interleaved rounded results in the quarter-width destination elements. Each result element is saturated to the quarter-width N-bit element's signed integer range -2<sup>(N-1)</sup> to (2<sup>(N-1)</sup>)-1. The immediate shift amount is an unsigned value in the range 1 to number of bits per source element. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if tsize == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << HighestSetBitNZ(tsize); let n : integer = UInt(Zn::'00'); let d : integer = UInt(Zd); let shift : integer = (8 \* esize) - UInt(tsize::imm5);

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV (4 \* esize); var result : bits(VL); for e = 0 to elements-1 do for i = 0 to 3 do let operand : bits(VL) = Z{}(n+i); let element : bits(4 \* esize) = operand[e\*:(4 \* esize)]; let res : integer = (SInt(element) + (1 << (shift-1))) >> shift; result[(4\*e + i)\*:esize] = [SignedSat](shared_pseudocode.html#func_SignedSat_2){esize}(res); end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
