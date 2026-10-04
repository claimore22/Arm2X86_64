## SM4EKEY

SM4 key updates This instruction reads four rounds of 32-bit input key values from each 128-bit segment of the first source vector, along with four rounds of 32-bit constants from the corresponding 128-bit segment of the second source vector. The four rounds of output key values are derived in accordance with the SM4 standard, and placed in the corresponding segments of the destination vector. This instruction is unpredicated. ID_AA64ZFR0_EL1.SM4 indicates whether this instruction is implemented. This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

#### Decode

if !IsFeatureImplemented(FEAT_SVE_SM4) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd);

### Operation

[CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let segments : integer = VL DIV 128; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); for s = 0 to segments-1 do let source : bits(128) = operand2[s\*:128]; var intval : bits(32); var const : bits(32); var roundresult : bits(128) = operand1[s\*:128]; for index = 0 to 3 do const = source[index\*:32]; intval = roundresult[127:96] XOR roundresult[95:64] XOR roundresult[63:32] XOR const; for i = 0 to 3 do intval[i\*:8] = [Sbox](shared_pseudocode.html#func_Sbox_1)(intval[i\*:8]); end; intval = intval XOR ROL(intval, 13) XOR ROL(intval, 23); intval = intval XOR roundresult[31:0]; roundresult[31:0] = roundresult[63:32]; roundresult[63:32] = roundresult[95:64]; roundresult[95:64] = roundresult[127:96]; roundresult[127:96] = intval; end; result[s\*:128] = roundresult; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
