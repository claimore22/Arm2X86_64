## CSINC

Conditional select increment

This instruction returns, in the destination register, the value of the first source register if the condition is TRUE, and otherwise returns the value of the second source register incremented by 1.

This instruction is used by the aliases [CINC](cinc_csinc.html "Conditional increment"), and [CSET](cset_csinc.html "Conditional set").

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
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rm</td>
<td colspan="4" class="lr">cond</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
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
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="CSINC_32_condsel"></span> Applies when <span class="bitdiff"> (sf == 0)</span>

CSINC [\<Wd\>](#WdOrWZR "Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Wn\>](#WnOrWZR__3 "Is the 32-bit name of the first general-purpose source register, encoded in the "Rn" field."), [\<Wm\>](#WmOrWZR__2 "Is the 32-bit name of the second general-purpose source register, encoded in the "Rm" field."), [\<cond\>](#cond_option "Is one of the standard conditions, encoded in the standard way, and ")

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="CSINC_64_condsel"></span> Applies when <span class="bitdiff"> (sf == 1)</span>

CSINC [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Xn\>](#XnOrXZR__12 "Is the 64-bit name of the first general-purpose source register, encoded in the "Rn" field."), [\<Xm\>](#XmOrXZR__4 "Is the 64-bit name of the second general-purpose source register, encoded in the "Rm" field."), [\<cond\>](#cond_option "Is one of the standard conditions, encoded in the standard way, and ")

</div>

#### Decode for all variants of this encoding

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 32 \<\< UInt(sf); let condition : bits(4) = cond;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wd&gt;</td>
<td><span id="WdOrWZR"></span>
<p>Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

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
<td>&lt;Xd&gt;</td>
<td><span id="XdOrXZR__6"></span>
<p>Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field.</p></td>
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

### Alias Conditions

| Alias | Is preferred when |
|----|----|
| [CINC](cinc_csinc.html "Conditional increment") | <span class="pseudocode">Rm != '11111' && Rn != '11111' && !(cond IN {'111x'}) && Rn == Rm</span> |
| [CSET](cset_csinc.html "Conditional set") | <span class="pseudocode">Rm == '11111' && Rn == '11111' && !(cond IN {'111x'})</span> |

<div class="ps">

### Operation

var result : bits(datasize); if [ConditionHolds](shared_pseudocode.html#func_ConditionHolds_1)(condition) then result = [X](shared_pseudocode.html#accessor_X_2){datasize}(n); else result = [X](shared_pseudocode.html#accessor_X_2){datasize}(m) + 1; end; [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
