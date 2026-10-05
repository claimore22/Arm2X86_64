## SLI

Shift left and insert (immediate) This instruction reads each vector element in the source SIMD&FP register, left shifts each vector element by an immediate value, and inserts the result into the corresponding vector element in the destination SIMD&FP register such that the new zero bits created by the shift are not inserted but retain their existing value. Bits shifted out of the left of each vector element in the source register are lost. The following figure shows an example of the operation of shift left by 3 for an 8-bit vector element.\ ![shift left by 3 for an 8-bit vector element](A64.sli_operation_shift_by_3.svg) Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. It has encodings from 2 classes: [Scalar](#iclass_scalar) and [Vector](#iclass_vector)

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if immh[3]::Q == '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 8 << HighestSetBitNZ(immh); let datasize : integer{} = 64 << UInt(Q); let elements : integer = datasize DIV esize; let shift : integer = UInt(immh::immb) - esize;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(datasize) = V{}(n); let operand2 : bits(datasize) = V{}(d); let mask : bits(esize) = LSL(Ones{esize}, shift); var result : bits(datasize); var shifted : bits(esize); for e = 0 to elements-1 do shifted = LSL(operand[e\*:esize], shift); result[e\*:esize] = (operand2[e\*:esize] AND NOT(mask)) OR shifted; end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
