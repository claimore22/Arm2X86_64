## SVC

Supervisor call

This instruction causes an exception to be taken to EL1.

On executing an <span class="asm-code">SVC</span> instruction, the PE records the exception as a Supervisor Call exception in <span class="armarm-xref" data-linkend="ARMARM_ESR_ELx">ESR_ELx</span>, using the EC syndrome value 0x15, and the value of the immediate argument.

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

<span id="SVC_EX_exception"></span>

SVC \#[\<imm\>](#imm "Is a 16-bit unsigned immediate, in the range 0 to 65535, encoded in the "imm16" field.")

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

[AArch64_CheckForSVCTrap](shared_pseudocode.html#func_AArch64_CheckForSVCTrap_1)(imm); [AArch64_CallSupervisor](shared_pseudocode.html#func_AArch64_CallSupervisor_1)(imm);

</div>
