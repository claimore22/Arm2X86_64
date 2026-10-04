## SSRA

Signed shift right and accumulate (immediate)

This instruction shifts right by immediate each signed element of the source vector, preserving the sign bit, and destructively adds the truncated intermediate result to the corresponding elements of the addend vector. The immediate shift amount is an unsigned value in the range 1 to number of bits per element. This instruction is unpredicated.

### <span id="iclass_sve2"></span>SVE2<span style="font-size:smaller;">\
(FEAT_SVE2 \|\| FEAT_SME) </span>

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
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">tszh</td>
<td class="lr">0</td>
<td colspan="2" class="lr">tszl</td>
<td colspan="3" class="lr">imm3</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zda</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td class="droppedname">R</td>
<td class="droppedname">U</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="ssra_z_zi_"></span>

SSRA [\<Zda\>](#Zda "Is the name of the third source and destination scalable vector register, encoded in the "Zda" field.").[\<T\>](#T__92 "Is the size specifier, "), [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__92 "Is the size specifier, "), \#[\<const\>](#const__2 "Is the immediate shift amount, in the range 1 to number of bits per element, encoded in "tszh:tszl:imm3".")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let tsize : bits(4) = tszh::tszl; if tsize == '0000' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< HighestSetBitNZ(tsize); let n : integer = UInt(Zn); let da : integer = UInt(Zda); let shift : integer = (2 \* esize) - UInt(tsize::imm3);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zda&gt;</td>
<td><span id="Zda"></span>
<p>Is the name of the third source and destination scalable vector register, encoded in the "Zda" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__92"></span>
<p>Is the size specifier, encoded in “(tszh :: tszl)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">tszh</th>
<th class="bitfield">tszl</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="bitfield">00</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">00</td>
<td class="bitfield">01</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">00</td>
<td class="bitfield">1x</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="bitfield">xx</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">1x</td>
<td class="bitfield">xx</td>
<td class="symbol">D</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn&gt;</td>
<td><span id="Zn__2"></span>
<p>Is the name of the first source scalable vector register, encoded in the "Zn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;const&gt;</td>
<td><span id="const__2"></span>
<p>Is the immediate shift amount, in the range 1 to number of bits per element, encoded in "tszh:tszl:imm3".</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(da); var result : bits(VL); for e = 0 to elements-1 do let element : integer = SInt(operand1\[e\*:esize\]) \>\> shift; result\[e\*:esize\] = operand2\[e\*:esize\] + element\[esize-1:0\]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.

This instruction might be immediately preceded in program order by a <span class="asm-code">MOVPRFX</span> instruction. The <span class="asm-code">MOVPRFX</span> must conform to all of the following requirements, otherwise the behavior of the <span class="asm-code">MOVPRFX</span> and this instruction is <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span>:

- The <span class="asm-code">MOVPRFX</span> must be unpredicated.
- The <span class="asm-code">MOVPRFX</span> must specify the same destination register as this instruction.
- The destination register must not refer to architectural register state referenced by any other source operand register of this instruction.
