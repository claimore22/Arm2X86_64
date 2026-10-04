## RSHRN, RSHRN2

Rounding shift right narrow (immediate) This instruction reads each unsigned integer value from the vector in the source SIMD&FP register, right shifts each result by an immediate value, writes the final result to a vector, and writes the vector to the lower or upper half of the destination SIMD&FP register. The destination vector elements are half as long as the source vector elements. The results are rounded. For truncated results, see SHRN. The RSHRN instruction writes the vector to the lower half of the destination register and clears the upper half. The RSHRN2 instruction writes the vector to the upper half of the destination register without affecting the other bits of the register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if immh[3] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 8 << HighestSetBitNZ(immh[2:0]); let datasize : integer{} = 64; let part : integer = UInt(Q); let elements : integer = datasize DIV esize; let shift : integer = (2 \* esize) - UInt(immh::immb); let round : boolean = TRUE;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(datasize\*2) = V{}(n); var result : bits(datasize); var element : integer; for e = 0 to elements-1 do element = [RShr](shared_pseudocode.html#func_RShr_3)(UInt(operand[e\*:(2\*esize)]), shift, round); result[e\*:esize] = element[esize-1:0]; end; [Vpart](shared_pseudocode.html#accessor_Vpart_3){datasize}(d, part) = result;
