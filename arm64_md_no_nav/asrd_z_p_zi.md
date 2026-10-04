## ASRD

Arithmetic shift right for divide by immediate (predicated)

This instruction shifts right by immediate, preserving the sign bit, each active element of the source vector, and destructively places the results in the corresponding elements of the source vector. The result rounds toward zero as in a signed division. The immediate shift amount is an unsigned value in the range 1 to number of bits per element. Inactive elements in the destination vector register remain unmodified.

### <span id="iclass_sve"></span>SVE<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="2" class="lr">tszh</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">0</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="2" class="lr">tszl</td>
<td colspan="3" class="lr">imm3</td>
<td colspan="5" class="lr">Zdn</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td class="droppedname">L</td>
<td class="droppedname">U</td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="asrd_z_p_zi_"></span>

ASRD [\<Zdn\>](#Zdn__2 "Is the name of the source and destination scalable vector register, encoded in the "Zdn" field.").[\<T\>](#T__14 "Is the size specifier, "), [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/M, [\<Zdn\>](#Zdn__2 "Is the name of the source and destination scalable vector register, encoded in the "Zdn" field.").[\<T\>](#T__14 "Is the size specifier, "), \#[\<const\>](#const__2 "Is the immediate shift amount, in the range 1 to number of bits per element, encoded in "tszh:tszl:imm3".")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let tsize : bits(4) = tszh::tszl; if tsize == '0000' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< HighestSetBitNZ(tsize); let g : integer = UInt(Pg); let dn : integer = UInt(Zdn); let shift : integer = (2 \* esize) - UInt(tsize::imm3);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zdn&gt;</td>
<td><span id="Zdn__2"></span>
<p>Is the name of the source and destination scalable vector register, encoded in the "Zdn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__14"></span>
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
<td>&lt;Pg&gt;</td>
<td><span id="Pg"></span>
<p>Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.</p></td>
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

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(VL) = Z{}(dn); var result : bits(VL); for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then var element1 : integer = SInt(operand1\[e\*:esize\]); if element1 \< 0 then element1 = element1 + ((1 \<\< shift) - 1); end; result\[e\*:esize\] = (element1 \>\> shift)\[esize-1:0\]; else result\[e\*:esize\] = operand1\[e\*:esize\]; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.

This instruction might be immediately preceded in program order by a <span class="asm-code">MOVPRFX</span> instruction. The <span class="asm-code">MOVPRFX</span> must conform to all of the following requirements, otherwise the behavior of the <span class="asm-code">MOVPRFX</span> and this instruction is <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span>:

- The <span class="asm-code">MOVPRFX</span> can be predicated or unpredicated.
- A predicated <span class="asm-code">MOVPRFX</span> must use the same governing predicate register as this instruction.
- A predicated <span class="asm-code">MOVPRFX</span> must use the larger of the destination element size and first source element size in the preferred disassembly of this instruction.
- The <span class="asm-code">MOVPRFX</span> must specify the same destination register as this instruction.
- The destination register must not refer to architectural register state referenced by any other source operand register of this instruction.
