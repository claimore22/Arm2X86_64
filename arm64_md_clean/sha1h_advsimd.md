## SHA1H

SHA1 fixed rotate SHA1 fixed rotate.

#### Decode

if !IsFeatureImplemented(FEAT_SHA1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(32) = V{}(n); // read element [0] only, [1-3] zeroed [V](shared_pseudocode.html#accessor_V_2){32}(d) = ROL(operand, 30);
