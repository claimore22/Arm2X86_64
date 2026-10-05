## AESE (vectors)

AES single round encryption This instruction reads a 16-byte state array from each 128-bit segment of the first source vector together with a round key from the corresponding 128-bit segment of the second source vector. Each state array undergoes a single round of the AddRoundKey(), ShiftRows(), and SubBytes() transformations in accordance with the AES standard. Each updated state array is destructively placed in the corresponding segment of the first source vector. This instruction is unpredicated. ID_AA64ZFR0_EL1.AES indicates whether this instruction is implemented. This instruction is legal when executed in Streaming SVE mode if one of the following is true: - Both FEAT_SSVE_AES and FEAT_SVE_AES are implemented. - FEAT_SME_FA64 is implemented and enabled.

#### Decode

if !IsFeatureImplemented(FEAT_SVE_AES) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let m : integer = UInt(Zm); let dn : integer = UInt(Zdn);

### Operation

if IsFeatureImplemented(FEAT_SSVE_AES) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let segments : integer = VL DIV 128; let operand1 : bits(VL) = Z{}(dn); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); result = operand1 XOR operand2; for s = 0 to segments-1 do result[s\*:128] = [AESSubBytes](shared_pseudocode.html#func_AESSubBytes_1)([AESShiftRows](shared_pseudocode.html#func_AESShiftRows_1)(result[s\*:128])); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;
