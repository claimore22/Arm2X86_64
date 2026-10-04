## SUBS (extended register)

Subtract extended and scaled register, setting flags This instruction subtracts a sign or zero-extended register value, followed by an optional left shift amount, from a register value, and writes the result to the destination register. The argument that is extended from the <Rm> register can be a byte, halfword, word, or doubleword. It updates the condition flags based on the result. This instruction is used by the alias [CMP (extended register)](cmp_subs_addsub_ext.html "Compare (extended register)").

#### Decode

if imm3 IN {'101', '110', '111'} then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let shift : integer{} = UInt(imm3) as integer{0..4}; let datasize : integer{} = 32 << UInt(sf); let extend_type : [ExtendType](shared_pseudocode.html#type_ExtendType) = [DecodeRegExtend](shared_pseudocode.html#func_DecodeRegExtend_1)(option);

### Operation

let operand1 : bits(datasize) = if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){datasize}() else [X](shared_pseudocode.html#accessor_X_2){datasize}(n); let operand2 : bits(datasize) = NOT([ExtendReg](shared_pseudocode.html#func_ExtendReg_4){datasize}(m, extend_type, shift)); var result : bits(datasize); var nzcv : bits(4); (result, nzcv) = [AddWithCarry](shared_pseudocode.html#func_AddWithCarry_4){datasize}(operand1, operand2, '1'); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result; PSTATE.[N,Z,C,V] = nzcv;
