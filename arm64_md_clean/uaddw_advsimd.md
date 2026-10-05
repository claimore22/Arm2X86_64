## UADDW, UADDW2

Unsigned add wide This instruction adds the vector elements of the first source SIMD&FP register to the corresponding vector elements in the lower or upper half of the second source SIMD&FP register, places the result in a vector, and writes the vector to the SIMD&FP destination register. The vector elements of the destination register and the first source register are twice as long as the vector elements of the second source register. All the values in this instruction are unsigned integer values. The UADDW instruction extracts vector elements from the lower half of the second source register. The UADDW2 instruction extracts vector elements from the upper half of the second source register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '11' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let esize : integer{} = 8 << UInt(size); let datasize : integer{} = 64; let part : integer = UInt(Q); let elements : integer = datasize DIV esize;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(2\*datasize) = V{}(n); let operand2 : bits(datasize) = Vpart{}(m, part); var result : bits(2\*datasize); var element1 : integer; var element2 : integer; var sum : integer; for e = 0 to elements-1 do element1 = UInt(operand1[e\*:(2\*esize)]); element2 = UInt(operand2[e\*:esize]); sum = element1 + element2; result[e\*:(2\*esize)] = sum[2\*esize-1:0]; end; [V](shared_pseudocode.html#accessor_V_2){2\*datasize}(d) = result;
