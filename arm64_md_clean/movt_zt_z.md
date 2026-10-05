## MOVT (vector to table)

Move vector register to ZT0 This instruction copies the source vector register to ZT0 at the vector length offset specified by the immediate index. When the index is zero, the instruction writes zeroes to the most significant (512-VL) bits of the ZT0 register. When the index is not zero, the unindexed portions of ZT0 remain unchanged. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SME_LUTv2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer = UInt(Zt); let imm : integer = UInt(off2);

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); [CheckSMEZT0Enabled](shared_pseudocode.html#func_CheckSMEZT0Enabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let tsize : integer{} = (if VL <= 512 then VL else 512) as integer{128, 256, 512}; let offset : integer = imm MOD (512 DIV tsize); var result : bits(512) = if imm == 0 then Zeros{512} else [ZT0](shared_pseudocode.html#accessor_ZT0_1){512}(); result[offset\*:tsize] = [Z](shared_pseudocode.html#accessor_Z_2){VL}(t)[tsize-1:0]; [ZT0](shared_pseudocode.html#accessor_ZT0_1){512}() = result;
