## MSRR

Move two adjacent general-purpose registers to System register This instruction allows the PE to write an AArch64 128-bit System register from two adjacent 64-bit general-purpose registers.

#### Decode

if !IsFeatureImplemented(FEAT_SYSREG128) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if Rt[0] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer = UInt(Rt); let t2 : integer = UInt(Rt+1); let sys_L : bits(1) = L; let sys_op0 : bits(2) = '1' :: o0; let sys_op1 : bits(3) = op1; let sys_op2 : bits(3) = op2; let sys_crn : bits(4) = CRn; let sys_crm : bits(4) = CRm;

### Operation

AArch64_SysRegWrite128(sys_op0, sys_op1, sys_crn, sys_crm, sys_op2, t, t2);
