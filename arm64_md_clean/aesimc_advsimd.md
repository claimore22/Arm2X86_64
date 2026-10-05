## AESIMC

AES inverse mix columns AES inverse mix columns.

#### Decode

if !IsFeatureImplemented(FEAT_AES) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(128) = V{}(n); [V](shared_pseudocode.html#accessor_V_2){128}(d) = [AESInvMixColumns](shared_pseudocode.html#func_AESInvMixColumns_1)(operand);
