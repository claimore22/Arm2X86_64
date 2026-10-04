## INS (general)

Insert vector element from general-purpose register This instruction copies the contents of the source general-purpose register to the specified vector element in the destination SIMD&FP register. This instruction can insert data into individual elements within a SIMD&FP register without clearing the remaining bits to zero. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. This instruction is used by the alias [MOV (from general)](mov_ins_advsimd_gen.html "Move general-purpose register to a vector element").

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if imm5 == 'x0000' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let size : integer{} = LowestSetBitNZ(imm5[3:0]); let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let index : integer = UInt(imm5[4:size+1]); let esize : integer{} = 8 << size;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let element : bits(esize) = X{}(n); var result : bits(128) = V{}(d); result[index\*:esize] = element; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
