## UUNPK

Unpack and zero-extend multi-vector elements This instruction unpacks and zero-extends elements from one or two source vectors, and writes the zero-extended values into double width elements of the two or four destination vectors. This instruction is unpredicated. It has encodings from 2 classes: [Two registers](#iclass_two_registers) and [Four registers](#iclass_four_registers)

#### Decode

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn::'0'); let d : integer = UInt(Zd::'00'); let nreg : integer{} = 4; let unsigned : boolean = TRUE;

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let hsize : integer{} = esize DIV 2; let sreg : integer = nreg DIV 2; var results : array [[4]] of bits(VL); for r = 0 to sreg-1 do let operand : bits(VL) = Z{}(n+r); for i = 0 to 1 do for e = 0 to elements-1 do let element : bits(hsize) = operand[(i\*elements + e)\*:hsize]; results[[2\*r+i]][e\*:esize] = Extend{esize}(element, unsigned); end; end; end; for r = 0 to nreg-1 do [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+r) = results[[r]]; end;
