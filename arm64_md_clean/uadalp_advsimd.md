## UADALP

Unsigned add and accumulate long pairwise This instruction adds pairs of adjacent unsigned integer values from the vector in the source SIMD&FP register and accumulates the results with the vector elements of the destination SIMD&FP register. The destination vector elements are twice as long as the source vector elements. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '11' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 8 << UInt(size); let datasize : integer{} = 64 << UInt(Q); let elements : integer = datasize DIV (2 \* esize);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(datasize) = V{}(n); var result : bits(datasize) = V{}(d); var sum : bits(2\*esize); var op1 : integer; var op2 : integer; for e = 0 to elements-1 do op1 = UInt(operand[(2\*e+0)\*:esize]); op2 = UInt(operand[(2\*e+1)\*:esize]); sum = (op1+op2)[2\*esize-1:0]; result[e\*:(2\*esize)] = result[e\*:(2\*esize)] + sum; end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
