## AESE

AES single round encryption AES single round encryption.

#### Decode

if !IsFeatureImplemented(FEAT_AES) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(128) = V{}(d); let operand2 : bits(128) = V{}(n); var result : bits(128) = operand1 XOR operand2; result = [AESShiftRows](shared_pseudocode.html#func_AESShiftRows_1)(result); [V](shared_pseudocode.html#accessor_V_2){128}(d) = [AESSubBytes](shared_pseudocode.html#func_AESSubBytes_1)(result);
