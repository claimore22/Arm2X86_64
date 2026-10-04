## DCPS1

Debug change PE state to EL1

This instruction, when executed in Debug state:

- If executed at EL0, changes the current Exception level and SP to EL1 using SP_EL1.
- Otherwise, if executed at ELx, selects <span class="armarm-xref" data-linkend="ARMARM_SP_ELx">SP_ELx</span>.

The target Exception level of a DCPS1 instruction is:

- EL1 if the instruction is executed at EL0.
- Otherwise, the Exception level at which the instruction is executed.

When the target Exception level of a DCPS1 instruction is ELx, on executing this instruction:

- <span class="armarm-xref" data-linkend="ARMARM_ELR_ELx">ELR_ELx</span> becomes <span class="arm-defined-word">UNKNOWN</span>.
- <span class="armarm-xref" data-linkend="ARMARM_SPSR_ELx">SPSR_ELx</span> becomes <span class="arm-defined-word">UNKNOWN</span>.
- <span class="armarm-xref" data-linkend="ARMARM_ESR_ELx">ESR_ELx</span> becomes <span class="arm-defined-word">UNKNOWN</span>.
- DLR_EL0 and DSPSR_EL0 become <span class="arm-defined-word">UNKNOWN</span>.
- The endianness is set according to <span class="armarm-xref" data-linkend="ARMARM_SCTLR_ELx">SCTLR_ELx</span>.EE.

This instruction is always <span class="arm-defined-word">UNDEFINED</span> in Non-debug state.

This instruction is <span class="arm-defined-word">UNDEFINED</span> at EL0 if EL2 is implemented and enabled in the current Security state and HCR_EL2.TGE == 1.

For more information on the operation of the DCPS\<n\> instructions, see <span class="armarm-xref" data-linkend="ARMARM_dcps">DCPS</span>.

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
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td colspan="16" class="lr">imm16</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
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

<span id="DCPS1_DC_exception"></span>

DCPS1 {#[\<imm\>](#imm__2 "Is an optional 16-bit unsigned immediate, in the range 0 to 65535, defaulting to 0 and encoded in the "imm16" field.")}

</div>

#### Decode for this encoding

// Empty.

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm__2"></span>
<p>Is an optional 16-bit unsigned immediate, in the range 0 to 65535, defaulting to 0 and encoded in the "imm16" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

if \![Halted](shared_pseudocode.html#func_Halted_0)() then Undefined(); end; [DCPSInstruction](shared_pseudocode.html#func_DCPSInstruction_1)([EL1](shared_pseudocode.html#global_EL1));

</div>
