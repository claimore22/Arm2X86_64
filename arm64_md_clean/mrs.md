## MRS

Move System register to general-purpose register This instruction allows the PE to read an AArch64 System register into a general-purpose register.

#### Decode

let t : integer = UInt(Rt); let sys_L : bits(1) = L; let sys_op0 : bits(2) = '1' :: o0; let sys_op1 : bits(3) = op1; let sys_op2 : bits(3) = op2; let sys_crn : bits(4) = CRn; let sys_crm : bits(4) = CRm;

### Operation

AArch64_SysRegRead(sys_op0, sys_op1, sys_crn, sys_crm, sys_op2, t);
