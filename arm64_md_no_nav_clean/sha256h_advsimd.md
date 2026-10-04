## SHA256H

SHA-256 hash update (part 1) SHA-256 hash update (part 1).

#### Decode

if !IsFeatureImplemented(FEAT_SHA256) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let part1 : boolean = TRUE; [V](shared_pseudocode.html#accessor_V_2){128}(d) = [SHA256hash](shared_pseudocode.html#func_SHA256hash_4)([V](shared_pseudocode.html#accessor_V_2){128}(d), [V](shared_pseudocode.html#accessor_V_2){128}(n), [V](shared_pseudocode.html#accessor_V_2){128}(m), part1);
