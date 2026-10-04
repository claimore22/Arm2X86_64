## SYSL

System instruction with result For more information, see op0 == 0b01, cache maintenance, TLB maintenance, address translation, prediction restriction, BRBE, Trace Extension, and Guarded Control Stack instructions for the encodings of System instructions. This instruction is used by the aliases [GCSPOPM](gcspopm_sysl.html "Guarded Control Stack pop"), and [GCSSS2](gcsss2_sysl.html "Guarded Control Stack switch stack 2").

#### Decode

let t : integer = UInt(Rt); let sys_L : bits(1) = L; let sys_op0 : bits(2) = '01'; let sys_op1 : bits(3) = op1; let sys_op2 : bits(3) = op2; let sys_crn : bits(4) = CRn; let sys_crm : bits(4) = CRm;

### Operation

AArch64_SysInstrWithResult(sys_op0, sys_op1, sys_crn, sys_crm, sys_op2, t);
