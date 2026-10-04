## LSR (wide elements, predicated)

Logical shift right by 64-bit wide elements (predicated)

This instruction shifts right, inserting zeroes, active elements of the first source vector by the corresponding overlapping 64-bit elements of the second source vector and destructively places the results in the corresponding elements of the first source vector. The shift amount is a vector of unsigned 64-bit doubleword elements in which all bits are significant, and not used modulo the destination element size. Inactive elements in the destination vector register remain unmodified.

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
<td colspan="2" class="lr">size</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>0</td>
<td class="r">0</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zm</td>
<td colspan="5" class="lr">Zdn</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="2"></td>
<td class="droppedname">R</td>
<td class="droppedname">L</td>
<td class="droppedname">U</td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="lsr_z_p_zw_"></span>

LSR [\<Zdn\>](#Zdn "Is the name of the first source and destination scalable vector register, encoded in the "Zdn" field.").[\<T\>](#T__15 "Is the size specifier, "), [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/M, [\<Zdn\>](#Zdn "Is the name of the first source and destination scalable vector register, encoded in the "Zdn" field.").[\<T\>](#T__15 "Is the size specifier, "), [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").D

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '11' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let g : integer = UInt(Pg); let dn : integer = UInt(Zdn); let m : integer = UInt(Zm);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zdn&gt;</td>
<td><span id="Zdn"></span>
<p>Is the name of the first source and destination scalable vector register, encoded in the "Zdn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__15"></span>
<p>Is the size specifier, encoded in “size”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">size</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">11</td>
<td class="symbol">RESERVED</td>
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
<td>&lt;Zm&gt;</td>
<td><span id="Zm"></span>
<p>Is the name of the second source scalable vector register, encoded in the "Zm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(VL) = Z{}(dn); let operand2 : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(m) else Zeros{VL}; var result : bits(VL); for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then let element1 : bits(esize) = operand1\[e\*:esize\]; let element2 : bits(64) = operand2\[((e \* esize) DIVRM 64)\*:64\]; let shift : integer = Min(UInt(element2), esize); result\[e\*:esize\] = LSR(element1, shift); else result\[e\*:esize\] = operand1\[e\*:esize\]; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.

This instruction might be immediately preceded in program order by a <span class="asm-code">MOVPRFX</span> instruction. The <span class="asm-code">MOVPRFX</span> must conform to all of the following requirements, otherwise the behavior of the <span class="asm-code">MOVPRFX</span> and this instruction is <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span>:

- The <span class="asm-code">MOVPRFX</span> can be predicated or unpredicated.
- A predicated <span class="asm-code">MOVPRFX</span> must use the same governing predicate register as this instruction.
- A predicated <span class="asm-code">MOVPRFX</span> must use the larger of the destination element size and first source element size in the preferred disassembly of this instruction.
- The <span class="asm-code">MOVPRFX</span> must specify the same destination register as this instruction.
- The destination register must not refer to architectural register state referenced by any other source operand register of this instruction.
