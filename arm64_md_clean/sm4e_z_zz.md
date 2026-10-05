## SM4E

SM4 encryption and decryption This instruction reads 16 bytes of input data from each 128-bit segment of the first source vector, together with four iterations of 32-bit round keys from the corresponding 128-bit segments of the second source vector. Each block of data is encrypted by four rounds in accordance with the SM4 standard, and destructively placed in the corresponding segments of the first source vector. This instruction is unpredicated. ID_AA64ZFR0_EL1.SM4 indicates whether this instruction is implemented. This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

#### Decode

if !IsFeatureImplemented(FEAT_SVE_SM4) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let m : integer = UInt(Zm); let dn : integer = UInt(Zdn);

### Operation

[CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let segments : integer = VL DIV 128; let operand1 : bits(VL) = Z{}(dn); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); for s = 0 to segments-1 do let key : bits(128) = operand2[s\*:128]; var intval : bits(32); var roundresult : bits(128) = operand1[s\*:128]; var roundkey : bits(32); for index = 0 to 3 do roundkey = key[index\*:32]; intval = roundresult[127:96] XOR roundresult[95:64] XOR roundresult[63:32] XOR roundkey; for i = 0 to 3 do intval[i\*:8] = [Sbox](shared_pseudocode.html#func_Sbox_1)(intval[i\*:8]); end; intval = (intval XOR ROL(intval, 2) XOR ROL(intval, 10) XOR ROL(intval, 18) XOR ROL(intval, 24)); intval = intval XOR roundresult[31:0]; roundresult[31:0] = roundresult[63:32]; roundresult[63:32] = roundresult[95:64]; roundresult[95:64] = roundresult[127:96]; roundresult[127:96] = intval; end; result[s\*:128] = roundresult; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;
