## INS (element)

Insert vector element from another vector element This instruction copies the vector element of the source SIMD&FP register to the specified vector element of the destination SIMD&FP register. This instruction can insert data into individual elements within a SIMD&FP register without clearing the remaining bits to zero. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. This instruction is used by the alias [MOV (element)](mov_ins_advsimd_elt.html "Move vector element to another vector element").

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if imm5 == 'x0000' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let size : integer{} = LowestSetBitNZ(imm5[3:0]); let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let dst_index : integer = UInt(imm5[4:size+1]); let src_index : integer = UInt(imm4[3:size]); let idxdsize : integer{} = 64 << UInt(imm4[3]); // imm4<size-1:0> is IGNORED let esize : integer{} = 8 << size;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(idxdsize) = V{}(n); var result : bits(128); result = [V](shared_pseudocode.html#accessor_V_2){128}(d); result[dst_index\*:esize] = operand[src_index\*:esize]; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
