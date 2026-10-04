## SMC

Secure monitor call

This instruction causes an exception to EL3.

<span class="asm-code">SMC</span> is available only for software executing at EL1 or higher. It is <span class="arm-defined-word">UNDEFINED</span> at EL0.

If the values of HCR_EL2.TSC and SCR_EL3.SMD are both 0, execution of an <span class="asm-code">SMC</span> instruction at EL1 or higher generates a Secure Monitor Call exception, recording it in <span class="armarm-xref" data-linkend="ARMARM_ESR_ELx">ESR_ELx</span>, using the EC syndrome value 0x17, that is taken to EL3.

If the value of HCR_EL2.TSC is 1 and EL2 is enabled in the current Security state, execution of an <span class="asm-code">SMC</span> instruction at EL1 generates an exception that is taken to EL2, regardless of the value of SCR_EL3.SMD.

If the value of HCR_EL2.TSC is 0 and the value of SCR_EL3.SMD is 1, the SMC instruction is <span class="arm-defined-word">UNDEFINED</span>.

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
<td class="r">1</td>
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

<span id="SMC_EX_exception"></span>

SMC \#[\<imm\>](#imm "Is a 16-bit unsigned immediate, in the range 0 to 65535, encoded in the "imm16" field.")

</div>

#### Decode for this encoding

let imm : bits(16) = imm16;

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

if [PSTATE](shared_pseudocode.html#global_PSTATE).EL == [EL0](shared_pseudocode.html#global_EL0) then Undefined(); end; [AArch64_CheckForSMCUndefOrTrap](shared_pseudocode.html#func_AArch64_CheckForSMCUndefOrTrap_1)(imm); [AArch64_CallSecureMonitor](shared_pseudocode.html#func_AArch64_CallSecureMonitor_1)(imm);

</div>
