## BFMLALB, BFMLALT (vector)

BFloat16 multiply-add to single-precision (vector) This instruction widens the even-numbered (bottom) or odd-numbered (top) 16-bit elements in the first and second source vectors from Bfloat16 to single-precision format. The instruction then multiplies and adds these values without intermediate rounding to the single-precision elements of the destination vector that overlap with the corresponding BFloat16 elements in the source vectors. ID_AA64ISAR1_EL1.BF16 indicates whether this instruction is supported.

#### Decode

if !IsFeatureImplemented(FEAT_BF16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let elements : integer = 128 DIV 32; let sel : integer = UInt(Q);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(128) = V{}(n); let operand2 : bits(128) = V{}(m); let operand3 : bits(128) = V{}(d); var result : bits(128); for e = 0 to elements-1 do let element1 : bits(16) = operand1[(2 \* e + sel)\*:16]; let element2 : bits(16) = operand2[(2 \* e + sel)\*:16]; let addend : bits(32) = operand3[e\*:32]; result[e\*:32] = [BFMulAddH](shared_pseudocode.html#func_BFMulAddH_4)(addend, element1, element2, FPCR()); end; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
