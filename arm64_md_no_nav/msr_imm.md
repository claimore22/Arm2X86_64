## MSR (immediate)

Move immediate value to special register

This instruction moves an immediate value to selected bits of the PSTATE. For more information, see <span class="armarm-xref" data-linkend="ARMARM_PSTATE">Process state, PSTATE</span>.

The bits that can be written by this instruction are:

- PSTATE.D, PSTATE.A, PSTATE.I, PSTATE.F, and PSTATE.SP.
- If <span class="armarm-xref" data-linkend="ARMARM_FEAT_SSBS">FEAT_SSBS</span> is implemented, PSTATE.SSBS.
- If <span class="armarm-xref" data-linkend="ARMARM_FEAT_PAN">FEAT_PAN</span> is implemented, PSTATE.PAN.
- If <span class="armarm-xref" data-linkend="ARMARM_FEAT_UAO">FEAT_UAO</span> is implemented, PSTATE.UAO.
- If <span class="armarm-xref" data-linkend="ARMARM_FEAT_DIT">FEAT_DIT</span> is implemented, PSTATE.DIT.
- If <span class="armarm-xref" data-linkend="ARMARM_FEAT_MTE">FEAT_MTE</span> is implemented, PSTATE.TCO.
- If <span class="armarm-xref" data-linkend="ARMARM_FEAT_NMI">FEAT_NMI</span> is implemented, PSTATE.ALLINT.
- If <span class="armarm-xref" data-linkend="ARMARM_FEAT_SME">FEAT_SME</span> is implemented, PSTATE.SM and PSTATE.ZA.
- If FEAT_EBEP is implemented, PSTATE.PM.

If <span class="armarm-xref" data-linkend="ARMARM_FEAT_MTE">FEAT_MTE</span> is implemented and <span class="armarm-xref" data-linkend="ARMARM_FEAT_MTE2">FEAT_MTE2</span> is not implemented, it is <span class="arm-defined-word">IMPLEMENTATION DEFINED</span> whether writes to PSTATE.TCO by this instruction are ignored.

This instruction is used by the aliases [SMSTART](smstart_msr_imm.html "Enable access to Streaming SVE mode and SME architectural state"), and [SMSTOP](smstop_msr_imm.html "Disable access to Streaming SVE mode and SME architectural state").

<div class="regdiagram-32">

<table class="regdiagram">
<thead>
<tr>
<th>31</th>
<th>30</th>
<th>29</th>
<th>28</th>
<th>27</th>
<th>26</th>
<th>25</th>
<th>24</th>
<th>23</th>
<th>22</th>
<th>21</th>
<th>20</th>
<th>19</th>
<th>18</th>
<th>17</th>
<th>16</th>
<th>15</th>
<th>14</th>
<th>13</th>
<th>12</th>
<th>11</th>
<th>10</th>
<th>9</th>
<th>8</th>
<th>7</th>
<th>6</th>
<th>5</th>
<th>4</th>
<th>3</th>
<th>2</th>
<th>1</th>
<th>0</th>
</tr>
</thead>
<tbody>
<tr class="firstrow">
<td class="l">1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="3" class="lr">op1</td>
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td colspan="4" class="lr">CRm</td>
<td colspan="3" class="lr">op2</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="7"></td>
<td colspan="3"></td>
<td colspan="4"></td>
<td colspan="4"></td>
<td colspan="3"></td>
<td colspan="5" class="droppedname">Rt</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="MSR_SI_pstate"></span> Applies when <span class="bitdiff"> (!(op1 == 000 && op2 IN {00x, 010}))</span>

MSR [\<pstatefield\>](#pstatefield_option "Is a PSTATE field name. For the MSR instruction, this is "), \#[\<imm\>](#msr_imm "Is a 4-bit unsigned immediate, in the range 0 to 15, encoded in the "CRm" field. Restricted to the range 0 to 1, encoded in "CRm<0>", when <pstatefield> is ALLINT, PM, SVCRSM, SVCRSMZA, or SVCRZA.")

</div>

#### Decode for this encoding

if op1 == '000' && op2 == '000' then See("CFINV"); end; if op1 == '000' && op2 == '001' then See("XAFLAG"); end; if op1 == '000' && op2 == '010' then See("AXFLAG"); end; var min_EL : bits(2); var need_secure : boolean = FALSE; case op1 of when '00x' =\> min_EL = [EL1](shared_pseudocode.html#global_EL1); when '010' =\> min_EL = [EL1](shared_pseudocode.html#global_EL1); when '011' =\> min_EL = [EL0](shared_pseudocode.html#global_EL0); when '100' =\> min_EL = [EL2](shared_pseudocode.html#global_EL2); when '101' =\> if !IsFeatureImplemented(FEAT_VHE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; min_EL = [EL2](shared_pseudocode.html#global_EL2); when '110' =\> min_EL = [EL3](shared_pseudocode.html#global_EL3); when '111' =\> min_EL = [EL1](shared_pseudocode.html#global_EL1); need_secure = TRUE; end; let operand : bits(4) = CRm; var field : [PSTATEField](shared_pseudocode.html#type_PSTATEField); case op1::op2 of when '000 011' =\> if !IsFeatureImplemented(FEAT_UAO) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; field = [PSTATEField_UAO](shared_pseudocode.html#enum_PSTATEField_UAO); when '000 100' =\> if !IsFeatureImplemented(FEAT_PAN) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; field = [PSTATEField_PAN](shared_pseudocode.html#enum_PSTATEField_PAN); when '000 101' =\> field = [PSTATEField_SP](shared_pseudocode.html#enum_PSTATEField_SP); when '001 000' =\> case CRm of when '000x' =\> if !IsFeatureImplemented(FEAT_NMI) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; field = [PSTATEField_ALLINT](shared_pseudocode.html#enum_PSTATEField_ALLINT); when '001x' =\> if !IsFeatureImplemented(FEAT_EBEP) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; field = [PSTATEField_PM](shared_pseudocode.html#enum_PSTATEField_PM); otherwise =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; when '011 010' =\> if !IsFeatureImplemented(FEAT_DIT) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; field = [PSTATEField_DIT](shared_pseudocode.html#enum_PSTATEField_DIT); when '011 011' =\> case CRm of when '001x' =\> if !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; field = [PSTATEField_SVCRSM](shared_pseudocode.html#enum_PSTATEField_SVCRSM); when '010x' =\> if !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; field = [PSTATEField_SVCRZA](shared_pseudocode.html#enum_PSTATEField_SVCRZA); when '011x' =\> if !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; field = [PSTATEField_SVCRSMZA](shared_pseudocode.html#enum_PSTATEField_SVCRSMZA); otherwise =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; when '011 100' =\> if !IsFeatureImplemented(FEAT_MTE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; field = [PSTATEField_TCO](shared_pseudocode.html#enum_PSTATEField_TCO); when '011 110' =\> field = [PSTATEField_DAIFSet](shared_pseudocode.html#enum_PSTATEField_DAIFSet); when '011 111' =\> field = [PSTATEField_DAIFClr](shared_pseudocode.html#enum_PSTATEField_DAIFClr); when '011 001' =\> if !IsFeatureImplemented(FEAT_SSBS) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; field = [PSTATEField_SSBS](shared_pseudocode.html#enum_PSTATEField_SSBS); otherwise =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;pstatefield&gt;</td>
<td><span id="pstatefield_option"></span>
<p>Is a PSTATE field name. For the MSR instruction, this is encoded in “(CRm :: op1 :: op2)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">CRm</th>
<th class="bitfield">op1</th>
<th class="bitfield">op2</th>
<th class="symbol">&lt;pstatefield&gt;</th>
<th class="symbol">Architectural Feature</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">xxxx</td>
<td class="bitfield">000</td>
<td class="bitfield">011</td>
<td class="symbol">UAO</td>
<td class="feature">FEAT_UAO</td>
</tr>
<tr>
<td class="bitfield">xxxx</td>
<td class="bitfield">000</td>
<td class="bitfield">100</td>
<td class="symbol">PAN</td>
<td class="feature">FEAT_PAN</td>
</tr>
<tr>
<td class="bitfield">xxxx</td>
<td class="bitfield">000</td>
<td class="bitfield">101</td>
<td class="symbol">SPSel</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">xxxx</td>
<td class="bitfield">000</td>
<td class="bitfield">11x</td>
<td class="symbol">RESERVED</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">000x</td>
<td class="bitfield">001</td>
<td class="bitfield">000</td>
<td class="symbol">ALLINT</td>
<td class="feature">FEAT_NMI</td>
</tr>
<tr>
<td class="bitfield">xxxx</td>
<td class="bitfield">001</td>
<td class="bitfield">001</td>
<td class="symbol">RESERVED</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">xxxx</td>
<td class="bitfield">001</td>
<td class="bitfield">01x</td>
<td class="symbol">RESERVED</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">xxxx</td>
<td class="bitfield">001</td>
<td class="bitfield">1xx</td>
<td class="symbol">RESERVED</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">xxxx</td>
<td class="bitfield">010</td>
<td class="bitfield">xxx</td>
<td class="symbol">RESERVED</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">xxxx</td>
<td class="bitfield">011</td>
<td class="bitfield">000</td>
<td class="symbol">RESERVED</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">xxxx</td>
<td class="bitfield">011</td>
<td class="bitfield">001</td>
<td class="symbol">SSBS</td>
<td class="feature">FEAT_SSBS</td>
</tr>
<tr>
<td class="bitfield">xxxx</td>
<td class="bitfield">011</td>
<td class="bitfield">010</td>
<td class="symbol">DIT</td>
<td class="feature">FEAT_DIT</td>
</tr>
<tr>
<td class="bitfield">000x</td>
<td class="bitfield">011</td>
<td class="bitfield">011</td>
<td class="symbol">RESERVED</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">xxxx</td>
<td class="bitfield">011</td>
<td class="bitfield">100</td>
<td class="symbol">TCO</td>
<td class="feature">FEAT_MTE</td>
</tr>
<tr>
<td class="bitfield">xxxx</td>
<td class="bitfield">011</td>
<td class="bitfield">101</td>
<td class="symbol">RESERVED</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">xxxx</td>
<td class="bitfield">011</td>
<td class="bitfield">110</td>
<td class="symbol">DAIFSet</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">xxxx</td>
<td class="bitfield">011</td>
<td class="bitfield">111</td>
<td class="symbol">DAIFClr</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">xxxx</td>
<td class="bitfield">1xx</td>
<td class="bitfield">xxx</td>
<td class="symbol">RESERVED</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">001x</td>
<td class="bitfield">001</td>
<td class="bitfield">000</td>
<td class="symbol">PM</td>
<td class="feature">FEAT_EBEP</td>
</tr>
<tr>
<td class="bitfield">001x</td>
<td class="bitfield">011</td>
<td class="bitfield">011</td>
<td class="symbol">SVCRSM</td>
<td class="feature">FEAT_SME</td>
</tr>
<tr>
<td class="bitfield">01xx</td>
<td class="bitfield">001</td>
<td class="bitfield">000</td>
<td class="symbol">RESERVED</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">010x</td>
<td class="bitfield">011</td>
<td class="bitfield">011</td>
<td class="symbol">SVCRZA</td>
<td class="feature">FEAT_SME</td>
</tr>
<tr>
<td class="bitfield">011x</td>
<td class="bitfield">011</td>
<td class="bitfield">011</td>
<td class="symbol">SVCRSMZA</td>
<td class="feature">FEAT_SME</td>
</tr>
<tr>
<td class="bitfield">1xxx</td>
<td class="bitfield">001</td>
<td class="bitfield">000</td>
<td class="symbol">RESERVED</td>
<td class="feature">-</td>
</tr>
<tr>
<td class="bitfield">1xxx</td>
<td class="bitfield">011</td>
<td class="bitfield">011</td>
<td class="symbol">RESERVED</td>
<td class="feature">-</td>
</tr>
</tbody>
</table>
For the SMSTART and SMSTOP aliases, this is encoded in "CRm&lt;2:1&gt;", where 0b01 specifies SVCRSM, 0b10 specifies SVCRZA, and 0b11 specifies SVCRSMZA.</td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="msr_imm"></span>
<p>Is a 4-bit unsigned immediate, in the range 0 to 15, encoded in the "CRm" field. Restricted to the range 0 to 1, encoded in "CRm&lt;0&gt;", when &lt;pstatefield&gt; is ALLINT, PM, SVCRSM, SVCRSMZA, or SVCRZA.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

### Alias Conditions

| Alias | Is preferred when |
|----|----|
| [SMSTART](smstart_msr_imm.html "Enable access to Streaming SVE mode and SME architectural state") | <span class="pseudocode">op1 == '011' && CRm IN {'0xx1'} && op2 == '011'</span> |
| [SMSTOP](smstop_msr_imm.html "Disable access to Streaming SVE mode and SME architectural state") | <span class="pseudocode">op1 == '011' && CRm IN {'0xx0'} && op2 == '011'</span> |

<div class="ps">

### Operation

if UInt([PSTATE](shared_pseudocode.html#global_PSTATE).EL) \< UInt(min_EL) then Undefined(); end; if need_secure && [CurrentSecurityState](shared_pseudocode.html#func_CurrentSecurityState_0)() != [SS_Secure](shared_pseudocode.html#enum_SS_Secure) then Undefined(); end; case field of when [PSTATEField_SSBS](shared_pseudocode.html#enum_PSTATEField_SSBS) =\> [PSTATE](shared_pseudocode.html#global_PSTATE).SSBS = operand\[0\]; when [PSTATEField_SP](shared_pseudocode.html#enum_PSTATEField_SP) =\> [PSTATE](shared_pseudocode.html#global_PSTATE).SP = operand\[0\]; when [PSTATEField_DAIFSet](shared_pseudocode.html#enum_PSTATEField_DAIFSet) =\> [AArch64_CheckDAIFAccess](shared_pseudocode.html#func_AArch64_CheckDAIFAccess_1)([PSTATEField_DAIFSet](shared_pseudocode.html#enum_PSTATEField_DAIFSet)); [PSTATE](shared_pseudocode.html#global_PSTATE).D = [PSTATE](shared_pseudocode.html#global_PSTATE).D OR operand\[3\]; [PSTATE](shared_pseudocode.html#global_PSTATE).A = [PSTATE](shared_pseudocode.html#global_PSTATE).A OR operand\[2\]; [PSTATE](shared_pseudocode.html#global_PSTATE).I = [PSTATE](shared_pseudocode.html#global_PSTATE).I OR operand\[1\]; [PSTATE](shared_pseudocode.html#global_PSTATE).F = [PSTATE](shared_pseudocode.html#global_PSTATE).F OR operand\[0\]; when [PSTATEField_DAIFClr](shared_pseudocode.html#enum_PSTATEField_DAIFClr) =\> [AArch64_CheckDAIFAccess](shared_pseudocode.html#func_AArch64_CheckDAIFAccess_1)([PSTATEField_DAIFClr](shared_pseudocode.html#enum_PSTATEField_DAIFClr)); [PSTATE](shared_pseudocode.html#global_PSTATE).D = [PSTATE](shared_pseudocode.html#global_PSTATE).D AND NOT(operand\[3\]); [PSTATE](shared_pseudocode.html#global_PSTATE).A = [PSTATE](shared_pseudocode.html#global_PSTATE).A AND NOT(operand\[2\]); [PSTATE](shared_pseudocode.html#global_PSTATE).I = [PSTATE](shared_pseudocode.html#global_PSTATE).I AND NOT(operand\[1\]); [PSTATE](shared_pseudocode.html#global_PSTATE).F = [PSTATE](shared_pseudocode.html#global_PSTATE).F AND NOT(operand\[0\]); when [PSTATEField_PAN](shared_pseudocode.html#enum_PSTATEField_PAN) =\> [PSTATE](shared_pseudocode.html#global_PSTATE).PAN = operand\[0\]; when [PSTATEField_UAO](shared_pseudocode.html#enum_PSTATEField_UAO) =\> [PSTATE](shared_pseudocode.html#global_PSTATE).UAO = operand\[0\]; when [PSTATEField_DIT](shared_pseudocode.html#enum_PSTATEField_DIT) =\> [PSTATE](shared_pseudocode.html#global_PSTATE).DIT = operand\[0\]; when [PSTATEField_TCO](shared_pseudocode.html#enum_PSTATEField_TCO) =\> [PSTATE](shared_pseudocode.html#global_PSTATE).TCO = operand\[0\]; when [PSTATEField_ALLINT](shared_pseudocode.html#enum_PSTATEField_ALLINT) =\> if ([PSTATE](shared_pseudocode.html#global_PSTATE).EL == [EL1](shared_pseudocode.html#global_EL1) && [IsHCRXEL2Enabled](shared_pseudocode.html#func_IsHCRXEL2Enabled_0)() && HCRX_EL2().TALLINT == '1' && operand\[0\] == '1') then [AArch64_SystemAccessTrap](shared_pseudocode.html#func_AArch64_SystemAccessTrap_2)([EL2](shared_pseudocode.html#global_EL2), 0x18); end; [PSTATE](shared_pseudocode.html#global_PSTATE).ALLINT = operand\[0\]; when [PSTATEField_SVCRSM](shared_pseudocode.html#enum_PSTATEField_SVCRSM) =\> [CheckSMEAccess](shared_pseudocode.html#func_CheckSMEAccess_0)(); [SetPSTATE_SM](shared_pseudocode.html#func_SetPSTATE_SM_1)(operand\[0\]); when [PSTATEField_SVCRZA](shared_pseudocode.html#enum_PSTATEField_SVCRZA) =\> [CheckSMEAccess](shared_pseudocode.html#func_CheckSMEAccess_0)(); [SetPSTATE_ZA](shared_pseudocode.html#func_SetPSTATE_ZA_1)(operand\[0\]); when [PSTATEField_SVCRSMZA](shared_pseudocode.html#enum_PSTATEField_SVCRSMZA) =\> [CheckSMEAccess](shared_pseudocode.html#func_CheckSMEAccess_0)(); [SetPSTATE_SM](shared_pseudocode.html#func_SetPSTATE_SM_1)(operand\[0\]); [SetPSTATE_ZA](shared_pseudocode.html#func_SetPSTATE_ZA_1)(operand\[0\]); when [PSTATEField_PM](shared_pseudocode.html#enum_PSTATEField_PM) =\> [PSTATE](shared_pseudocode.html#global_PSTATE).PM = operand\[0\]; end;

</div>
