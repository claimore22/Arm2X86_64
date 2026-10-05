## SYSP

128-bit system instruction 128-bit system instruction. This instruction is used by the alias [TLBIP](tlbip_sysp.html "TLB invalidate pair operation").

#### Decode

if !IsFeatureImplemented(FEAT_SYSINSTR128) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if Rt[0] == '1' && Rt != '11111' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer = UInt(Rt); let t2 : integer = if t == 31 then 31 else t + 1; let sys_L : bits(1) = L; let sys_op0 : bits(2) = '01'; let sys_op1 : bits(3) = op1; let sys_op2 : bits(3) = op2; let sys_crn : bits(4) = CRn; let sys_crm : bits(4) = CRm;

### Operation

AArch64_SysInstr128(sys_op0, sys_op1, sys_crn, sys_crm, sys_op2, t, t2);
