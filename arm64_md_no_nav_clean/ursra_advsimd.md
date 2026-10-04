## URSRA

Unsigned rounding shift right and accumulate (immediate) This instruction reads each vector element in the source SIMD&FP register, right shifts each result by an immediate value, and accumulates the final results with the vector elements of the destination SIMD&FP register. All the values in this instruction are unsigned integer values. The results are rounded. For truncated results, see USRA. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. It has encodings from 2 classes: [Scalar](#iclass_scalar) and [Vector](#iclass_vector)

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if immh[3]::Q == '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 8 << HighestSetBitNZ(immh); let datasize : integer{} = 64 << UInt(Q); let elements : integer = datasize DIV esize; let shift : integer = (esize \* 2) - UInt(immh::immb); let unsigned : boolean = TRUE; let round : boolean = TRUE;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(datasize) = V{}(n); let operand2 : bits(datasize) = V{}(d); var result : bits(datasize); var element : integer; for e = 0 to elements-1 do let opelt : bits(esize) = operand[e\*:esize]; if unsigned then element = [RShr](shared_pseudocode.html#func_RShr_3)(UInt(opelt), shift, round); else element = [RShr](shared_pseudocode.html#func_RShr_3)(SInt(opelt), shift, round); end; result[e\*:esize] = operand2[e\*:esize] + element[esize-1:0]; end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
