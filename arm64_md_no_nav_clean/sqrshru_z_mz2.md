## SQRSHRU (two registers)

Multi-vector signed 32-bit integer saturating rounding shift right narrow by immediate to unsigned 16-bit integer This instruction shifts right by an immediate value the signed integer value in each element of the two source vectors, and places the rounded results in the half-width destination elements. Each result element is saturated to the half-width N-bit element's unsigned integer range 0 to (2<sup>N</sup>)-1. The immediate shift amount is an unsigned value in the range 1 to 16. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 16; let n : integer = UInt(Zn::'0'); let d : integer = UInt(Zd); let shift : integer = esize - UInt(imm4);

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV (2 \* esize); var result : bits(VL); for r = 0 to 1 do let operand : bits(VL) = Z{}(n+r); for e = 0 to elements-1 do let element : bits(2 \* esize) = operand[e\*:(2 \* esize)]; let res : integer = (SInt(element) + (1 << (shift-1))) >> shift; result[(r\*elements + e)\*:esize] = [UnsignedSat](shared_pseudocode.html#func_UnsignedSat_2){esize}(res); end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
