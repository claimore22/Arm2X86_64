## SYS

System instruction

For more information, see <span class="armarm-xref" data-linkend="ARMARM_BABEJJJE">op0 == 0b01, cache maintenance, TLB maintenance, address translation, prediction restriction, BRBE, Trace Extension, and Guarded Control Stack instructions</span> for the encodings of System instructions.

This instruction is used by the aliases [APAS](apas_sys.html "Associate physical address space"), [AT](at_sys.html "Address translate"), [BRB](brb_sys.html "Branch record buffer"), [CFP](cfp_sys.html "Control flow prediction restriction by context"), [COSP](cosp_sys.html "Clear other speculative prediction restriction by context"), [CPP](cpp_sys.html "Cache prefetch prediction restriction by context"), [DC](dc_sys.html "Data cache operation"), [DVP](dvp_sys.html "Data value prediction restriction by context"), [GCSPOPCX](gcspopcx_sys.html "Guarded Control Stack pop and compare exception return record"), [GCSPOPX](gcspopx_sys.html "Guarded Control Stack pop exception return record"), [GCSPUSHM](gcspushm_sys.html "Guarded Control Stack push"), [GCSPUSHX](gcspushx_sys.html "Guarded Control Stack push exception return record"), [GCSSS1](gcsss1_sys.html "Guarded Control Stack switch stack 1"), [IC](ic_sys.html "Instruction cache operation"), [TLBI](tlbi_sys.html "TLB invalidate operation"), and [TRCIT](trcit_sys.html "Trace instrumentation").

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
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="3" class="lr">op1</td>
<td colspan="4" class="lr">CRn</td>
<td colspan="4" class="lr">CRm</td>
<td colspan="3" class="lr">op2</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="4"></td>
<td class="droppedname">L</td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="4"></td>
<td colspan="4"></td>
<td colspan="3"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="SYS_CR_systeminstrs"></span>

SYS \#[\<op1\>](#op1 "Is a 3-bit unsigned immediate, in the range 0 to 7, encoded in the "op1" field."), [\<Cn\>](#Cn "Is a name 'Cn', with 'n' in the range 0 to 15, encoded in the "CRn" field."), [\<Cm\>](#Cm "Is a name 'Cm', with 'm' in the range 0 to 15, encoded in the "CRm" field."), \#[\<op2\>](#op2 "Is a 3-bit unsigned immediate, in the range 0 to 7, encoded in the "op2" field."){, [\<Xt\>](#XtOrXZR__2 "Is the 64-bit name of the optional general-purpose source register, defaulting to '11111', encoded in the "Rt" field.")}

</div>

#### Decode for this encoding

let t : integer = UInt(Rt); let sys_L : bits(1) = L; let sys_op0 : bits(2) = '01'; let sys_op1 : bits(3) = op1; let sys_op2 : bits(3) = op2; let sys_crn : bits(4) = CRn; let sys_crm : bits(4) = CRm;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;op1&gt;</td>
<td><span id="op1"></span>
<p>Is a 3-bit unsigned immediate, in the range 0 to 7, encoded in the "op1" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Cn&gt;</td>
<td><span id="Cn"></span>
<p>Is a name 'Cn', with 'n' in the range 0 to 15, encoded in the "CRn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Cm&gt;</td>
<td><span id="Cm"></span>
<p>Is a name 'Cm', with 'm' in the range 0 to 15, encoded in the "CRm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;op2&gt;</td>
<td><span id="op2"></span>
<p>Is a 3-bit unsigned immediate, in the range 0 to 7, encoded in the "op2" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xt&gt;</td>
<td><span id="XtOrXZR__2"></span>
<p>Is the 64-bit name of the optional general-purpose source register, defaulting to '11111', encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

### Alias Conditions

| Alias | Is preferred when |
|----|----|
| [APAS](apas_sys.html "Associate physical address space") | <span class="pseudocode">op1 == '110' && CRn == '0111' && CRm == '0000' && op2 == '000'</span> |
| [AT](at_sys.html "Address translate") | <span class="pseudocode">CRn == '0111' && CRm IN {'100x'} && SysOp(op1, '0111', CRm, op2) == Sys_AT</span> |
| [BRB](brb_sys.html "Branch record buffer") | <span class="pseudocode">op1 == '001' && CRn == '0111' && CRm == '0010' && SysOp('001', '0111', '0010', op2) == Sys_BRB</span> |
| [CFP](cfp_sys.html "Control flow prediction restriction by context") | <span class="pseudocode">op1 == '011' && CRn == '0111' && CRm == '0011' && op2 == '100'</span> |
| [COSP](cosp_sys.html "Clear other speculative prediction restriction by context") | <span class="pseudocode">op1 == '011' && CRn == '0111' && CRm == '0011' && op2 == '110'</span> |
| [CPP](cpp_sys.html "Cache prefetch prediction restriction by context") | <span class="pseudocode">op1 == '011' && CRn == '0111' && CRm == '0011' && op2 == '111'</span> |
| [DC](dc_sys.html "Data cache operation") | <span class="pseudocode">CRn == '0111' && SysOp(op1, '0111', CRm, op2) == Sys_DC</span> |
| [DVP](dvp_sys.html "Data value prediction restriction by context") | <span class="pseudocode">op1 == '011' && CRn == '0111' && CRm == '0011' && op2 == '101'</span> |
| [GCSPOPCX](gcspopcx_sys.html "Guarded Control Stack pop and compare exception return record") | <span class="pseudocode">op1 == '000' && CRn == '0111' && CRm == '0111' && op2 == '101'</span> |
| [GCSPOPX](gcspopx_sys.html "Guarded Control Stack pop exception return record") | <span class="pseudocode">op1 == '000' && CRn == '0111' && CRm == '0111' && op2 == '110'</span> |
| [GCSPUSHM](gcspushm_sys.html "Guarded Control Stack push") | <span class="pseudocode">op1 == '011' && CRn == '0111' && CRm == '0111' && op2 == '000'</span> |
| [GCSPUSHX](gcspushx_sys.html "Guarded Control Stack push exception return record") | <span class="pseudocode">op1 == '000' && CRn == '0111' && CRm == '0111' && op2 == '100'</span> |
| [GCSSS1](gcsss1_sys.html "Guarded Control Stack switch stack 1") | <span class="pseudocode">op1 == '011' && CRn == '0111' && CRm == '0111' && op2 == '010'</span> |
| [IC](ic_sys.html "Instruction cache operation") | <span class="pseudocode">CRn == '0111' && SysOp(op1, '0111', CRm, op2) == Sys_IC</span> |
| [TLBI](tlbi_sys.html "TLB invalidate operation") | <span class="pseudocode">CRn IN {'100x'} && SysOp(op1, CRn, CRm, op2) == Sys_TLBI</span> |
| [TRCIT](trcit_sys.html "Trace instrumentation") | <span class="pseudocode">op1 == '011' && CRn == '0111' && CRm == '0010' && op2 == '111'</span> |

<div class="ps">

### Operation

AArch64_SysInstr(sys_op0, sys_op1, sys_crn, sys_crm, sys_op2, t);

</div>
