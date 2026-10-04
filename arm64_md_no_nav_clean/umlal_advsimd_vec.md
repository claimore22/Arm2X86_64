## UMLAL, UMLAL2 (vector)

Unsigned multiply-add long (vector) This instruction multiplies the vector elements in the lower or upper half of the first source SIMD&FP register by the corresponding vector elements of the second source SIMD&FP register, and accumulates the results with the vector elements of the destination SIMD&FP register. The destination vector elements are twice as long as the elements that are multiplied. The UMLAL instruction extracts vector elements from the lower half of the first source register. The UMLAL2 instruction extracts vector elements from the upper half of the first source register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '11' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let esize : integer{} = 8 << UInt(size); let datasize : integer{} = 64; let part : integer = UInt(Q); let elements : integer = datasize DIV esize;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(datasize) = Vpart{}(n, part); let operand2 : bits(datasize) = Vpart{}(m, part); let operand3 : bits(2\*datasize) = V{}(d); var result : bits(2\*datasize); var element1 : integer; var element2 : integer; var product : bits(2\*esize); var accum : bits(2\*esize); for e = 0 to elements-1 do element1 = UInt(operand1[e\*:esize]); element2 = UInt(operand2[e\*:esize]); product = (element1 \* element2)[2\*esize-1:0]; accum = operand3[e\*:(2\*esize)] + product; result[e\*:(2\*esize)] = accum; end; [V](shared_pseudocode.html#accessor_V_2){2\*datasize}(d) = result;
