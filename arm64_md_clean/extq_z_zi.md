## EXTQ

Extract vector segment from each pair of quadword vector segments This instruction copies, for each 128-bit vector segment of the result, the indexed byte up to and including the last byte of the corresponding first source vector segment to the bottom of the result segment, and then fills the remainder of the result segment starting from the first byte of the corresponding second source vector segment. The result segments are destructively placed in the corresponding first source vector segment. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2p1) && !IsFeatureImplemented(FEAT_SME2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let dn : integer = UInt(Zdn); let m : integer = UInt(Zm); let position : integer{} = UInt(imm4) << 3;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let segments : integer = VL DIV 128; let operand1 : bits(VL) = Z{}(dn); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); for s = 0 to segments-1 do let concat : bits(256) = operand2[s\*:128] :: operand1[s\*:128]; result[s\*:128] = concat[position+127:position]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;
