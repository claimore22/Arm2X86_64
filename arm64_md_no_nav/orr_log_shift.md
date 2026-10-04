## ORR (shifted register)

Bitwise OR (shifted register)

This instruction performs a bitwise (inclusive) OR between a register value and an optionally-shifted register value, and writes the result to the destination register.

This instruction is used by the alias [MOV (register)](mov_orr_log_shift.html "Move register value").

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
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td colspan="2" class="lr">shift</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rm</td>
<td colspan="6" class="lr">imm6</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="5"></td>
<td colspan="2"></td>
<td class="droppedname">N</td>
<td colspan="5"></td>
<td colspan="6"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="ORR_32_log_shift"></span> Applies when <span class="bitdiff"> (sf == 0)</span>

ORR [\<Wd\>](#WdOrWZR "Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Wn\>](#WnOrWZR__3 "Is the 32-bit name of the first general-purpose source register, encoded in the "Rn" field."), [\<Wm\>](#WmOrWZR__2 "Is the 32-bit name of the second general-purpose source register, encoded in the "Rm" field."){, [\<shift\>](#shift_option__3 "Is the optional shift to be applied to the final source, defaulting to LSL and ") \#[\<amount\>](#amount__5 "For the "32-bit" variant: is the shift amount, in the range 0 to 31, defaulting to 0 and encoded in the "imm6" field.")}

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="ORR_64_log_shift"></span> Applies when <span class="bitdiff"> (sf == 1)</span>

ORR [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Xn\>](#XnOrXZR__12 "Is the 64-bit name of the first general-purpose source register, encoded in the "Rn" field."), [\<Xm\>](#XmOrXZR__4 "Is the 64-bit name of the second general-purpose source register, encoded in the "Rm" field."){, [\<shift\>](#shift_option__3 "Is the optional shift to be applied to the final source, defaulting to LSL and ") \#[\<amount\>](#amount__6 "For the "64-bit" variant: is the shift amount, in the range 0 to 63, defaulting to 0 and encoded in the "imm6" field.")}

</div>

#### Decode for all variants of this encoding

if sf == '0' && imm6\[5\] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 32 \<\< UInt(sf); let shift_type : [ShiftType](shared_pseudocode.html#type_ShiftType) = [DecodeShift](shared_pseudocode.html#func_DecodeShift_1)(shift); let shift_amount : integer = UInt(imm6);

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
<td>&lt;shift&gt;</td>
<td><span id="shift_option__3"></span>
<p>Is the optional shift to be applied to the final source, defaulting to LSL and encoded in “shift”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">shift</th>
<th class="symbol">&lt;shift&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">LSL</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">LSR</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">ASR</td>
</tr>
<tr>
<td class="bitfield">11</td>
<td class="symbol">ROR</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;amount&gt;</td>
<td><span id="amount__5"></span>
<p>For the "32-bit" variant: is the shift amount, in the range 0 to 31, defaulting to 0 and encoded in the "imm6" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="amount__6"></span>
<p>For the "64-bit" variant: is the shift amount, in the range 0 to 63, defaulting to 0 and encoded in the "imm6" field.</p></td>
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
| [MOV (register)](mov_orr_log_shift.html "Move register value") | <span class="pseudocode">shift == '00' && imm6 == '000000' && Rn == '11111'</span> |

<div class="ps">

### Operation

let operand1 : bits(datasize) = X{}(n); let operand2 : bits(datasize) = ShiftReg{}(m, shift_type, shift_amount); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = operand1 OR operand2;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
