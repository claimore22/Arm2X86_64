## SABAL, SABAL2

Signed absolute difference and accumulate long This instruction subtracts the vector elements in the lower or upper half of the second source SIMD&FP register from the corresponding vector elements of the first source SIMD&FP register, and accumulates the absolute values of the results into the vector elements of the destination SIMD&FP register. The destination vector elements are twice as long as the source vector elements. The SABAL instruction extracts each source vector from the lower half of each source register. The SABAL2 instruction extracts each source vector from the upper half of each source register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '11' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let esize : integer{} = 8 << UInt(size); let datasize : integer{} = 64; let part : integer = UInt(Q); let elements : integer = datasize DIV esize;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(datasize) = Vpart{}(n, part); let operand2 : bits(datasize) = Vpart{}(m, part); var result : bits(2\*datasize) = [V](shared_pseudocode.html#accessor_V_2){2\*datasize}(d); var element1 : integer; var element2 : integer; var absdiff : bits(2\*esize); for e = 0 to elements-1 do element1 = SInt(operand1[e\*:esize]); element2 = SInt(operand2[e\*:esize]); absdiff = Abs(element1-element2)[2\*esize-1:0]; result[e\*:(2\*esize)] = result[e\*:(2\*esize)] + absdiff; end; [V](shared_pseudocode.html#accessor_V_2){2\*datasize}(d) = result;
