## CCMP (register)

Conditional compare (register)

This instruction sets the value of the condition flags to the result of the comparison of two registers if the condition is TRUE, and an immediate value otherwise.

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
<td class="lr">sf</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rm</td>
<td colspan="4" class="lr">cond</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rn</td>
<td class="lr">0</td>
<td colspan="4" class="lr">nzcv</td>
</tr>
<tr class="secondrow">
<td></td>
<td class="droppedname">op</td>
<td class="droppedname">S</td>
<td colspan="8"></td>
<td colspan="5"></td>
<td colspan="4"></td>
<td></td>
<td class="droppedname">o2</td>
<td colspan="5"></td>
<td class="droppedname">o3</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="CCMP_32_condcmp_reg"></span> Applies when <span class="bitdiff"> (sf == 0)</span>

CCMP [\<Wn\>](#WnOrWZR__3 "Is the 32-bit name of the first general-purpose source register, encoded in the "Rn" field."), [\<Wm\>](#WmOrWZR__2 "Is the 32-bit name of the second general-purpose source register, encoded in the "Rm" field."), \#[\<nzcv\>](#nzcv "Is the flag bit specifier, an immediate in the range 0 to 15, giving the alternative state for the 4-bit NZCV condition flags, encoded in the "nzcv" field."), [\<cond\>](#cond_option "Is one of the standard conditions, encoded in the standard way, and ")

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="CCMP_64_condcmp_reg"></span> Applies when <span class="bitdiff"> (sf == 1)</span>

CCMP [\<Xn\>](#XnOrXZR__12 "Is the 64-bit name of the first general-purpose source register, encoded in the "Rn" field."), [\<Xm\>](#XmOrXZR__4 "Is the 64-bit name of the second general-purpose source register, encoded in the "Rm" field."), \#[\<nzcv\>](#nzcv "Is the flag bit specifier, an immediate in the range 0 to 15, giving the alternative state for the 4-bit NZCV condition flags, encoded in the "nzcv" field."), [\<cond\>](#cond_option "Is one of the standard conditions, encoded in the standard way, and ")

</div>

#### Decode for all variants of this encoding

let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 32 \<\< UInt(sf); let condition : bits(4) = cond; var flags : bits(4) = nzcv;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wn&gt;</td>
<td><span id="WnOrWZR__3"></span>
<p>Is the 32-bit name of the first general-purpose source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Wm&gt;</td>
<td><span id="WmOrWZR__2"></span>
<p>Is the 32-bit name of the second general-purpose source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;nzcv&gt;</td>
<td><span id="nzcv"></span>
<p>Is the flag bit specifier, an immediate in the range 0 to 15, giving the alternative state for the 4-bit NZCV condition flags, encoded in the "nzcv" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;cond&gt;</td>
<td><span id="cond_option"></span>
<p>Is one of the standard conditions, encoded in the standard way, and encoded in “cond”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">cond</th>
<th class="symbol">&lt;cond&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0000</td>
<td class="symbol">EQ</td>
</tr>
<tr>
<td class="bitfield">0001</td>
<td class="symbol">NE</td>
</tr>
<tr>
<td class="bitfield">0010</td>
<td class="symbol">CS</td>
</tr>
<tr>
<td class="bitfield">0011</td>
<td class="symbol">CC</td>
</tr>
<tr>
<td class="bitfield">0100</td>
<td class="symbol">MI</td>
</tr>
<tr>
<td class="bitfield">0101</td>
<td class="symbol">PL</td>
</tr>
<tr>
<td class="bitfield">0110</td>
<td class="symbol">VS</td>
</tr>
<tr>
<td class="bitfield">0111</td>
<td class="symbol">VC</td>
</tr>
<tr>
<td class="bitfield">1000</td>
<td class="symbol">HI</td>
</tr>
<tr>
<td class="bitfield">1001</td>
<td class="symbol">LS</td>
</tr>
<tr>
<td class="bitfield">1010</td>
<td class="symbol">GE</td>
</tr>
<tr>
<td class="bitfield">1011</td>
<td class="symbol">LT</td>
</tr>
<tr>
<td class="bitfield">1100</td>
<td class="symbol">GT</td>
</tr>
<tr>
<td class="bitfield">1101</td>
<td class="symbol">LE</td>
</tr>
<tr>
<td class="bitfield">1110</td>
<td class="symbol">AL</td>
</tr>
<tr>
<td class="bitfield">1111</td>
<td class="symbol">NV</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xn&gt;</td>
<td><span id="XnOrXZR__12"></span>
<p>Is the 64-bit name of the first general-purpose source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xm&gt;</td>
<td><span id="XmOrXZR__4"></span>
<p>Is the 64-bit name of the second general-purpose source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

if [ConditionHolds](shared_pseudocode.html#func_ConditionHolds_1)(condition) then let operand1 : bits(datasize) = X{}(n); let operand2 : bits(datasize) = [X](shared_pseudocode.html#accessor_X_2){datasize}(m); (-, flags) = [AddWithCarry](shared_pseudocode.html#func_AddWithCarry_4){datasize}(operand1, NOT(operand2), '1'); end; PSTATE.\[N,Z,C,V\] = flags;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
