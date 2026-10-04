## REV

Reverse bytes

This instruction reverses the byte order in a register.

This instruction is used by the pseudo-instruction [REV64](rev64_rev.html "Reverse bytes").

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
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">1</td>
<td class="r">x</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td class="droppedname">S</td>
<td colspan="8"></td>
<td colspan="5" class="droppedname">opcode2</td>
<td colspan="4"></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="REV_32_dp_1src"></span> Applies when <span class="bitdiff"> (sf == 0 && opc == 10)</span>

REV [\<Wd\>](#WdOrWZR "Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Wn\>](#WnOrWZR "Is the 32-bit name of the general-purpose source register, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="REV_64_dp_1src"></span> Applies when <span class="bitdiff"> (sf == 1 && opc == 11)</span>

REV [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Xn\>](#XnOrXZR__11 "Is the 64-bit name of the general-purpose source register, encoded in the "Rn" field.")

</div>

#### Decode for all variants of this encoding

if opc == '11' && sf == '0' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let datasize : integer{} = 32 \<\< UInt(sf); let container_size : integer{} = 8 \<\< UInt(opc);

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
<td><span id="WnOrWZR"></span>
<p>Is the 32-bit name of the general-purpose source register, encoded in the "Rn" field.</p></td>
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
<td><span id="XnOrXZR__11"></span>
<p>Is the 64-bit name of the general-purpose source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

let operand : bits(datasize) = X{}(n); var result : bits(datasize); let containers : integer = datasize DIV container_size; for c = 0 to containers-1 do let container : bits(container_size) = operand\[c\*:container_size\]; result\[c\*:container_size\] = [Reverse](shared_pseudocode.html#func_Reverse_3){container_size}(container, 8); end; [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
