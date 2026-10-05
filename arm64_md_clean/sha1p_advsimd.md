## SHA1P

SHA1 hash update (parity) SHA1 hash update (parity).

#### Decode

if !IsFeatureImplemented(FEAT_SHA1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); var x : bits(128) = V{}(d); var y : bits(32) = V{}(n); // Note: 32 not 128 bits wide let w : bits(128) = V{}(m); for e = 0 to 3 do let t : bits(32) = [SHAparity](shared_pseudocode.html#func_SHAparity_3)(x[63:32], x[95:64], x[127:96]); y = y + ROL(x[31:0], 5) + t + w[e\*:32]; x[63:32] = ROL(x[63:32], 30); let yx : bits(160) = ROL(y::x, 32); (y, x) = (yx[159:128], yx[127:0]); end; [V](shared_pseudocode.html#accessor_V_2){128}(d) = x;
