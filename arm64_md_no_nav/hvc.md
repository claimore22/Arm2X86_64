## HVC

Hypervisor call

This instruction causes an exception to EL2. Software executing at EL1 can use this instruction to call the hypervisor to request a service.

The <span class="asm-code">HVC</span> instruction is <span class="arm-defined-word">UNDEFINED</span>:

- When EL3 is implemented and SCR_EL3.HCE is set to 0.
- When EL3 is not implemented and HCR_EL2.HCD is set to 1.
- When EL2 is not implemented.
- At EL1 if EL2 is not enabled in the current Security state.
- At EL0.

On executing an <span class="asm-code">HVC</span> instruction, the PE records the exception as a Hypervisor Call exception in <span class="armarm-xref" data-linkend="ARMARM_ESR_ELx">ESR_ELx</span>, using the EC syndrome value 0x16, and the value of the immediate argument.

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
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="16" class="lr">imm16</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">1</td>
<td class="r">0</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="3" class="droppedname">opc</td>
<td colspan="16"></td>
<td colspan="3" class="droppedname">op2</td>
<td colspan="2" class="droppedname">LL</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="HVC_EX_exception"></span>

HVC \#[\<imm\>](#imm "Is a 16-bit unsigned immediate, in the range 0 to 65535, encoded in the "imm16" field.")

</div>

#### Decode for this encoding

if \![HaveEL](shared_pseudocode.html#func_HaveEL_1)([EL2](shared_pseudocode.html#global_EL2)) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let imm : bits(16) = imm16;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm"></span>
<p>Is a 16-bit unsigned immediate, in the range 0 to 65535, encoded in the "imm16" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

if [PSTATE](shared_pseudocode.html#global_PSTATE).EL == [EL0](shared_pseudocode.html#global_EL0) then Undefined(); end; if [PSTATE](shared_pseudocode.html#global_PSTATE).EL == [EL1](shared_pseudocode.html#global_EL1) && \![EL2Enabled](shared_pseudocode.html#func_EL2Enabled_0)() then Undefined(); end; if \![HaveEL](shared_pseudocode.html#func_HaveEL_1)([EL3](shared_pseudocode.html#global_EL3)) && HCR_EL2().HCD == '1' then Undefined(); end; if [HaveEL](shared_pseudocode.html#func_HaveEL_1)([EL3](shared_pseudocode.html#global_EL3)) && SCR_EL3().HCE == '0' then Undefined(); end; [AArch64_CallHypervisor](shared_pseudocode.html#func_AArch64_CallHypervisor_1)(imm);

</div>
