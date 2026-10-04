## SUBS (extended register)

Subtract extended and scaled register, setting flags

This instruction subtracts a sign or zero-extended register value, followed by an optional left shift amount, from a register value, and writes the result to the destination register. The argument that is extended from the \<Rm\> register can be a byte, halfword, word, or doubleword. It updates the condition flags based on the result.

This instruction is used by the alias [CMP (extended register)](cmp_subs_addsub_ext.html "Compare (extended register)").

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
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rm</td>
<td colspan="3" class="lr">option</td>
<td colspan="3" class="lr">imm3</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td class="droppedname">op</td>
<td class="droppedname">S</td>
<td colspan="5"></td>
<td colspan="2" class="droppedname">opt</td>
<td></td>
<td colspan="5"></td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="SUBS_32S_addsub_ext"></span> Applies when <span class="bitdiff"> (sf == 0)</span>

SUBS [\<Wd\>](#WdOrWZR "Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Wn\|WSP\>](#WnWSP_option__2 "Is the 32-bit name of the first source general-purpose register or stack pointer, encoded in the "Rn" field."), [\<Wm\>](#WmOrWZR__2 "Is the 32-bit name of the second general-purpose source register, encoded in the "Rm" field."){, [\<extend\>](#extend_option__6 "For the "32-bit" variant: is the extension to be applied to the second source operand, ") {#[\<amount\>](#amount__4 "Is the left shift amount to be applied after extension in the range 0 to 4, defaulting to 0, encoded in the "imm3" field. It must be absent when <extend> is absent, is required when <extend> is LSL, and is optional when <extend> is present but not LSL.")}}

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="SUBS_64S_addsub_ext"></span> Applies when <span class="bitdiff"> (sf == 1)</span>

SUBS [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Xn\|SP\>](#XnSP_option__6 "Is the 64-bit name of the first source general-purpose register or stack pointer, encoded in the "Rn" field."), [\<R\>](#R_option__2 "Is a width specifier, ")[\<m\>](#Rm_option "Is the number [0-30] of the second general-purpose source register or the name ZR (31), encoded in the "Rm" field."){, [\<extend\>](#extend_option__8 "For the "64-bit" variant: is the extension to be applied to the second source operand, ") {#[\<amount\>](#amount__4 "Is the left shift amount to be applied after extension in the range 0 to 4, defaulting to 0, encoded in the "imm3" field. It must be absent when <extend> is absent, is required when <extend> is LSL, and is optional when <extend> is present but not LSL.")}}

</div>

#### Decode for all variants of this encoding

if imm3 IN {'101', '110', '111'} then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let shift : integer{} = UInt(imm3) as integer{0..4}; let datasize : integer{} = 32 \<\< UInt(sf); let extend_type : [ExtendType](shared_pseudocode.html#type_ExtendType) = [DecodeRegExtend](shared_pseudocode.html#func_DecodeRegExtend_1)(option);

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
<td><span id="WnWSP_option__2"></span>
<p>Is the 32-bit name of the first source general-purpose register or stack pointer, encoded in the "Rn" field.</p></td>
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
<td>&lt;extend&gt;</td>
<td><span id="extend_option__6"></span>
<p>For the "32-bit" variant: is the extension to be applied to the second source operand, encoded in “option”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">option</th>
<th class="symbol">&lt;extend&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">000</td>
<td class="symbol">UXTB</td>
</tr>
<tr>
<td class="bitfield">001</td>
<td class="symbol">UXTH</td>
</tr>
<tr>
<td class="bitfield">010</td>
<td class="symbol">LSL|UXTW</td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="symbol">UXTX</td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="symbol">SXTB</td>
</tr>
<tr>
<td class="bitfield">101</td>
<td class="symbol">SXTH</td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="symbol">SXTW</td>
</tr>
<tr>
<td class="bitfield">111</td>
<td class="symbol">SXTX</td>
</tr>
</tbody>
</table>
If "Rn" is '11111' (WSP) and "option" is '010' then LSL is preferred, but may be omitted when "imm3" is '000'. In all other cases &lt;extend&gt; is required and must be UXTW when "option" is '010'.</td>
</tr>
<tr>
<td></td>
<td><span id="extend_option__8"></span>
<p>For the "64-bit" variant: is the extension to be applied to the second source operand, encoded in “option”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">option</th>
<th class="symbol">&lt;extend&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">000</td>
<td class="symbol">UXTB</td>
</tr>
<tr>
<td class="bitfield">001</td>
<td class="symbol">UXTH</td>
</tr>
<tr>
<td class="bitfield">010</td>
<td class="symbol">UXTW</td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="symbol">LSL|UXTX</td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="symbol">SXTB</td>
</tr>
<tr>
<td class="bitfield">101</td>
<td class="symbol">SXTH</td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="symbol">SXTW</td>
</tr>
<tr>
<td class="bitfield">111</td>
<td class="symbol">SXTX</td>
</tr>
</tbody>
</table>
If "Rn" is '11111' (SP) and "option" is '011' then LSL is preferred, but may be omitted when "imm3" is '000'. In all other cases &lt;extend&gt; is required and must be UXTX when "option" is '011'.</td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;amount&gt;</td>
<td><span id="amount__4"></span>
<p>Is the left shift amount to be applied after extension in the range 0 to 4, defaulting to 0, encoded in the "imm3" field. It must be absent when &lt;extend&gt; is absent, is required when &lt;extend&gt; is LSL, and is optional when &lt;extend&gt; is present but not LSL.</p></td>
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
<td><span id="XnSP_option__6"></span>
<p>Is the 64-bit name of the first source general-purpose register or stack pointer, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;R&gt;</td>
<td><span id="R_option__2"></span>
<p>Is a width specifier, encoded in “option”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">option</th>
<th class="symbol">&lt;R&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00x</td>
<td class="symbol">W</td>
</tr>
<tr>
<td class="bitfield">010</td>
<td class="symbol">W</td>
</tr>
<tr>
<td class="bitfield">x11</td>
<td class="symbol">X</td>
</tr>
<tr>
<td class="bitfield">10x</td>
<td class="symbol">W</td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="symbol">W</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;m&gt;</td>
<td><span id="Rm_option"></span>
<p>Is the number [0-30] of the second general-purpose source register or the name ZR (31), encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

### Alias Conditions

| Alias | Is preferred when |
|----|----|
| [CMP (extended register)](cmp_subs_addsub_ext.html "Compare (extended register)") | <span class="pseudocode">Rd == '11111'</span> |

<div class="ps">

### Operation

let operand1 : bits(datasize) = if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){datasize}() else [X](shared_pseudocode.html#accessor_X_2){datasize}(n); let operand2 : bits(datasize) = NOT([ExtendReg](shared_pseudocode.html#func_ExtendReg_4){datasize}(m, extend_type, shift)); var result : bits(datasize); var nzcv : bits(4); (result, nzcv) = [AddWithCarry](shared_pseudocode.html#func_AddWithCarry_4){datasize}(operand1, operand2, '1'); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result; PSTATE.\[N,Z,C,V\] = nzcv;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
