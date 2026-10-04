## AND (immediate)

Bitwise AND (immediate)

This instruction performs a bitwise AND between a register value and an immediate value, and writes the result to the destination register.

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
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">N</td>
<td colspan="6" class="lr">immr</td>
<td colspan="6" class="lr">imms</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="6"></td>
<td></td>
<td colspan="6"></td>
<td colspan="6"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="AND_32_log_imm"></span> Applies when <span class="bitdiff"> (sf == 0 && N == 0)</span>

AND [\<Wd\|WSP\>](#WdWSP_option "Is the 32-bit name of the destination general-purpose register or stack pointer, encoded in the "Rd" field."), [\<Wn\>](#WnOrWZR "Is the 32-bit name of the general-purpose source register, encoded in the "Rn" field."), \#[\<imm\>](#imm__bitmask "Is the bitmask immediate. For encoding information, see x[Logical (immediate)](CHDBGEIB).")

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="AND_64_log_imm"></span> Applies when <span class="bitdiff"> (sf == 1)</span>

AND [\<Xd\|SP\>](#XdSP_option "Is the 64-bit name of the destination general-purpose register or stack pointer, encoded in the "Rd" field."), [\<Xn\>](#XnOrXZR__11 "Is the 64-bit name of the general-purpose source register, encoded in the "Rn" field."), \#[\<imm\>](#imm__bitmask "Is the bitmask immediate. For encoding information, see x[Logical (immediate)](CHDBGEIB).")

</div>

#### Decode for all variants of this encoding

if sf == '0' && N != '0' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let datasize : integer{} = 32 \<\< UInt(sf); var imm : bits(datasize); (imm, -) = [DecodeBitMasks](shared_pseudocode.html#func_DecodeBitMasks_5){datasize}(N, imms, immr, TRUE);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wd|WSP&gt;</td>
<td><span id="WdWSP_option"></span>
<p>Is the 32-bit name of the destination general-purpose register or stack pointer, encoded in the "Rd" field.</p></td>
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
<td>&lt;imm&gt;</td>
<td><span id="imm__bitmask"></span>
<p>Is the bitmask immediate. For encoding information, see <span class="armarm-xref" data-linkend="CHDBGEIB">Logical (immediate)</span>.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xd|SP&gt;</td>
<td><span id="XdSP_option"></span>
<p>Is the 64-bit name of the destination general-purpose register or stack pointer, encoded in the "Rd" field.</p></td>
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

let operand1 : bits(datasize) = X{}(n); let operand2 : bits(datasize) = imm; let result : bits(datasize) = operand1 AND operand2; if d == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = ZeroExtend{64}(result); else [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result; end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
