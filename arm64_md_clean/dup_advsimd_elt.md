## DUP (element)

Duplicate vector element to vector or scalar This instruction duplicates the vector element at the specified element index in the source SIMD&FP register into a scalar or each element in a vector, and writes the result to the destination SIMD&FP register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. This instruction is used by the alias [MOV (scalar)](mov_dup_advsimd_elt.html "Move vector element to scalar"). It has encodings from 2 classes: [Scalar](#iclass_scalar) and [Vector](#iclass_vector)

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if imm5 == 'x0000' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if imm5 == 'x1000' && Q == '0' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let size : integer{} = LowestSetBitNZ(imm5[3:0]); let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let index : integer = UInt(imm5[4:size+1]); let idxdsize : integer{} = 64 << UInt(imm5[4]); let esize : integer{} = 8 << size; let datasize : integer{} = 64 << UInt(Q); let elements : integer = datasize DIV esize;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(idxdsize) = V{}(n); var result : bits(datasize); var element : bits(esize); element = operand[index\*:esize]; for e = 0 to elements-1 do result[e\*:esize] = element; end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
