## UZPQ1

Concatenate even elements within each pair of quadword vector segments This instruction concatenates adjacent even-numbered elements from the corresponding 128-bit vector segments of the first and second source vectors, and places them in the corresponding elements of the destination vector segment. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2p1) && !IsFeatureImplemented(FEAT_SME2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd); let part : integer = 0;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let segments : integer = VL DIV 128; let elements : integer = 128 DIV esize; let pairs : integer = elements DIV 2; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); for s = 0 to segments-1 do let soff : integer = s \* elements; for p = 0 to pairs-1 do result[(soff + p)\*:esize] = operand1[(soff + 2 \* p + part)\*:esize]; end; for p = 0 to pairs-1 do result[(soff + pairs + p)\*:esize] = operand2[(soff + 2 \* p + part)\*:esize]; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
