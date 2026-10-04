## SSHLL, SSHLL2

Signed shift left long (immediate) This instruction reads each vector element from the source SIMD&FP register, left shifts each vector element by the specified shift amount, places the result into a vector, and writes the vector to the destination SIMD&FP register. The destination vector elements are twice as long as the source vector elements. All the values in this instruction are signed integer values. The SSHLL instruction extracts vector elements from the lower half of the source register. The SSHLL2 instruction extracts vector elements from the upper half of the source register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. This instruction is used by the alias [SXTL, SXTL2](sxtl_sshll_advsimd.html "Signed extend long").

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if immh[3] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 8 << HighestSetBitNZ(immh[2:0]); let datasize : integer{} = 64; let part : integer = UInt(Q); let elements : integer = datasize DIV esize; let shift : integer = UInt(immh::immb) - esize; let unsigned : boolean = FALSE;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(datasize) = Vpart{}(n, part); var result : bits(datasize\*2); for e = 0 to elements-1 do let opelt : bits(esize) = operand[e\*:esize]; let element : integer = if unsigned then UInt(opelt) else SInt(opelt); result[e\*:(2\*esize)] = (element << shift)[2\*esize-1:0]; end; [V](shared_pseudocode.html#accessor_V_2){datasize\*2}(d) = result;
