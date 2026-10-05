## HISTSEG

Count matching elements in vector segments This instruction compares each 8-bit byte element of the first source vector with all of the elements in the corresponding 128-bit segment of the second source vector and places the count of matching elements in the corresponding element of the destination vector. This instruction is unpredicated. This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size != '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8; let d : integer = UInt(Zd); let n : integer = UInt(Zn); let m : integer = UInt(Zm);

### Operation

[CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let segments : integer = VL DIV 128; let eltspersegment : integer = 128 DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); for b = 0 to segments-1 do for s = 0 to eltspersegment-1 do var count : integer = 0; let e : integer = eltspersegment \* b + s; let element1 : bits(esize) = operand1[e\*:esize]; for i = 0 to eltspersegment-1 do let e2 : integer = eltspersegment \* b + i; let element2 : bits(esize) = operand2[e2\*:esize]; if element1 == element2 then count = count + 1; end; end; result[e\*:esize] = count[esize-1:0]; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
