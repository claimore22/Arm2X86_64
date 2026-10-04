## AND (immediate)

Bitwise AND with immediate (unpredicated) This instruction performs a bitwise AND between an immediate and each 64-bit element of the source vector, and destructively places the results in the corresponding elements of the source vector. The immediate is a 64-bit value consisting of a single run of ones or zeros repeating every 2, 4, 8, 16, 32 or 64 bits. This instruction is unpredicated. This instruction is used by the pseudo-instruction [BIC (immediate)](bic_and_z_zi.html "Bitwise clear bits using immediate (unpredicated)").

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let dn : integer = UInt(Zdn); var imm : bits(64); (imm, -) = [DecodeBitMasks](shared_pseudocode.html#func_DecodeBitMasks_5){64}(imm13[12], imm13[5:0], imm13[11:6], TRUE);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 64; let operand : bits(VL) = Z{}(dn); var result : bits(VL); for e = 0 to elements-1 do let element1 : bits(64) = operand[e\*:64]; result[e\*:64] = element1 AND imm; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;
