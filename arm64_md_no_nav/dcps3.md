## DCPS3

Debug change PE state to EL3

This instruction, when executed in Debug state:

- If executed at EL3, selects SP_EL3.
- Otherwise, changes the current Exception level and SP to EL3 using SP_EL3.

The target Exception level of a DCPS3 instruction is EL3.

On executing a DCPS3 instruction:

- ELR_EL3 becomes <span class="arm-defined-word">UNKNOWN</span>.
- SPSR_EL3 becomes <span class="arm-defined-word">UNKNOWN</span>.
- ESR_EL3 becomes <span class="arm-defined-word">UNKNOWN</span>.
- DLR_EL0 and DSPSR_EL0 become <span class="arm-defined-word">UNKNOWN</span>.
- The endianness is set according to SCTLR_EL3.EE.

This instruction is always <span class="arm-defined-word">UNDEFINED</span> in Non-debug state.

This instruction is <span class="arm-defined-word">UNDEFINED</span> at all Exception levels if either:

- <span class="armarm-xref" data-linkend="ARMARM_ext.edscr">EDSCR</span>.SDD == 1.
- EL3 is not implemented.

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

<span id="DCPS3_DC_exception"></span>

DCPS3 {#[\<imm\>](#imm__2 "Is an optional 16-bit unsigned immediate, in the range 0 to 65535, defaulting to 0 and encoded in the "imm16" field.")}

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

if \![Halted](shared_pseudocode.html#func_Halted_0)() then Undefined(); end; [DCPSInstruction](shared_pseudocode.html#func_DCPSInstruction_1)([EL3](shared_pseudocode.html#global_EL3));

</div>
