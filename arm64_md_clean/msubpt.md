## MSUBPT

Multiply-subtract checked pointer This instruction multiplies two register values, subtracts the product from a third register value, and writes the result to the destination register. The intermediate product is treated as the offset. If the operation would have generated a result where the most significant 8 bits of the result register differ from the most significant 8 bits of the base register, then the result is modified such that it is likely to be non-canonical when used as an address. If the intermediate product cannot be correctly represented as a 64-bit two's complement value, then the result is modified such that it is likely to be non-canonical when used as an address.

#### Decode

if !IsFeatureImplemented(FEAT_CPA) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let a : integer{} = UInt(Ra);

### Operation

let operand1 : bits(64) = X{}(n); let operand2 : bits(64) = X{}(m); let operand3 : bits(64) = X{}(a); var result : bits(64); let product : integer = SInt(operand1) \* SInt(operand2); // Signed and unsigned twos complement arithmetic are equivalent if only a // fixed number of bits are considered. result = operand3 - product[63:0]; let overflow : boolean = (product != SInt(product[63:0])); result = [PointerMultiplyAddCheck](shared_pseudocode.html#func_PointerMultiplyAddCheck_3)(result, operand3, overflow); [X](shared_pseudocode.html#accessor_X_2){64}(d) = result;
