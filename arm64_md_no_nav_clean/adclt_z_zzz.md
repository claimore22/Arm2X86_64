## ADCLT

Add with carry long (top) This instruction adds the odd-numbered elements of the first source vector and the 1-bit carry from the least-significant bit of the odd-numbered elements of the second source vector to the even-numbered elements of the destination and accumulator vector. The 1-bit carry output is placed in the corresponding odd-numbered element of the destination vector.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32 << UInt(sz); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let pairs : integer = VL DIV (esize \* 2); let operand : bits(VL) = Z{}(n); let carries : bits(VL) = Z{}(m); var result : bits(VL) = Z{}(da); for p = 0 to pairs-1 do let element1 : bits(esize) = result[(2\*p + 0)\*:esize]; let element2 : bits(esize) = operand[(2\*p + 1)\*:esize]; let carry_in : bit = carries[(2\*p + 1)\*:esize][0]; let (res, nzcv) = [AddWithCarry](shared_pseudocode.html#func_AddWithCarry_4){esize}(element1, element2, carry_in); let carry_out : bit = nzcv[1]; result[(2\*p + 0)\*:esize] = res; result[(2\*p + 1)\*:esize] = ZeroExtend{esize}(carry_out); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
