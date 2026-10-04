## SUBS (immediate)

Subtract immediate value, setting flags

This instruction subtracts an optionally-shifted immediate value from a register value, and writes the result to the destination register. It updates the condition flags based on the result.

This instruction is used by the alias [CMP (immediate)](cmp_subs_addsub_imm.html "Compare (immediate)").

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
<td>0</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">sh</td>
<td colspan="12" class="lr">imm12</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td class="droppedname">op</td>
<td class="droppedname">S</td>
<td colspan="6"></td>
<td></td>
<td colspan="12"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="SUBS_32S_addsub_imm"></span> Applies when <span class="bitdiff"> (sf == 0)</span>

SUBS [\<Wd\>](#WdOrWZR "Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Wn\|WSP\>](#WnWSP_option "Is the 32-bit name of the source general-purpose register or stack pointer, encoded in the "Rn" field."), \#[\<imm\>](#imm__17 "Is an unsigned immediate, in the range 0 to 4095, encoded in the "imm12" field."){, [\<shift\>](#shift_option "Is the optional left shift to apply to the immediate, defaulting to LSL #0 and ")}

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="SUBS_64S_addsub_imm"></span> Applies when <span class="bitdiff"> (sf == 1)</span>

SUBS [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Xn\|SP\>](#XnSP_option__3 "Is the 64-bit name of the source general-purpose register or stack pointer, encoded in the "Rn" field."), \#[\<imm\>](#imm__17 "Is an unsigned immediate, in the range 0 to 4095, encoded in the "imm12" field."){, [\<shift\>](#shift_option "Is the optional left shift to apply to the immediate, defaulting to LSL #0 and ")}

</div>

#### Decode for all variants of this encoding

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let datasize : integer{} = 32 \<\< UInt(sf); let imm : bits(24) = if sh == '0' then Zeros{12}::imm12 else imm12::Zeros{12};

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
<td>&lt;Wn|WSP&gt;</td>
<td><span id="WnWSP_option"></span>
<p>Is the 32-bit name of the source general-purpose register or stack pointer, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm__17"></span>
<p>Is an unsigned immediate, in the range 0 to 4095, encoded in the "imm12" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;shift&gt;</td>
<td><span id="shift_option"></span>
<p>Is the optional left shift to apply to the immediate, defaulting to LSL #0 and encoded in “sh”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">sh</th>
<th class="symbol">&lt;shift&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">LSL #0</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">LSL #12</td>
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
<td>&lt;Xn|SP&gt;</td>
<td><span id="XnSP_option__3"></span>
<p>Is the 64-bit name of the source general-purpose register or stack pointer, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

### Alias Conditions

| Alias | Is preferred when |
|----|----|
| [CMP (immediate)](cmp_subs_addsub_imm.html "Compare (immediate)") | <span class="pseudocode">Rd == '11111'</span> |

<div class="ps">

### Operation

let operand1 : bits(datasize) = if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){datasize}() else [X](shared_pseudocode.html#accessor_X_2){datasize}(n); let operand2 : bits(datasize) = ZeroExtend{}(imm); var result : bits(datasize); var nzcv : bits(4); (result, nzcv) = [AddWithCarry](shared_pseudocode.html#func_AddWithCarry_4){datasize}(operand1, NOT(operand2), '1'); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result; PSTATE.\[N,Z,C,V\] = nzcv;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
