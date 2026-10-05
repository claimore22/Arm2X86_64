## SYS

System instruction For more information, see op0 == 0b01, cache maintenance, TLB maintenance, address translation, prediction restriction, BRBE, Trace Extension, and Guarded Control Stack instructions for the encodings of System instructions. This instruction is used by the aliases [APAS](apas_sys.html "Associate physical address space"), [AT](at_sys.html "Address translate"), [BRB](brb_sys.html "Branch record buffer"), [CFP](cfp_sys.html "Control flow prediction restriction by context"), [COSP](cosp_sys.html "Clear other speculative prediction restriction by context"), [CPP](cpp_sys.html "Cache prefetch prediction restriction by context"), [DC](dc_sys.html "Data cache operation"), [DVP](dvp_sys.html "Data value prediction restriction by context"), [GCSPOPCX](gcspopcx_sys.html "Guarded Control Stack pop and compare exception return record"), [GCSPOPX](gcspopx_sys.html "Guarded Control Stack pop exception return record"), [GCSPUSHM](gcspushm_sys.html "Guarded Control Stack push"), [GCSPUSHX](gcspushx_sys.html "Guarded Control Stack push exception return record"), [GCSSS1](gcsss1_sys.html "Guarded Control Stack switch stack 1"), [IC](ic_sys.html "Instruction cache operation"), [TLBI](tlbi_sys.html "TLB invalidate operation"), and [TRCIT](trcit_sys.html "Trace instrumentation").

#### Decode

let t : integer = UInt(Rt); let sys_L : bits(1) = L; let sys_op0 : bits(2) = '01'; let sys_op1 : bits(3) = op1; let sys_op2 : bits(3) = op2; let sys_crn : bits(4) = CRn; let sys_crm : bits(4) = CRm;

### Operation

AArch64_SysInstr(sys_op0, sys_op1, sys_crn, sys_crm, sys_op2, t);
