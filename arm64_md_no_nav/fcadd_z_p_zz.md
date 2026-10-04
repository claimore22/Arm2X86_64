## FCADD

Floating-point complex add (predicated)

This instruction adds the real and imaginary components of the active floating-point complex numbers from the first source vector to the complex numbers from the second source vector that have first been rotated by 90 or 270 degrees in the direction from the positive real axis towards the positive imaginary axis, when considered in polar representation, equivalent to multiplying the complex numbers in the second source vector by ±j beforehand. The results are destructively placed in the corresponding elements of the first source vector. Inactive elements in the destination vector register remain unmodified.

Each complex number is represented in a vector register as an even/odd pair of elements with the real part in the even-numbered element and the imaginary part in the odd-numbered element.

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
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="2" class="lr">size</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">rot</td>
<td class="l">1</td>
<td>0</td>
<td class="r">0</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zm</td>
<td colspan="5" class="lr">Zdn</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fcadd_z_p_zz_"></span>

FCADD [\<Zdn\>](#Zdn "Is the name of the first source and destination scalable vector register, encoded in the "Zdn" field.").[\<T\>](#T__37 "Is the size specifier, "), [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/M, [\<Zdn\>](#Zdn "Is the name of the first source and destination scalable vector register, encoded in the "Zdn" field.").[\<T\>](#T__37 "Is the size specifier, "), [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").[\<T\>](#T__37 "Is the size specifier, "), [\<const\>](#const__3 "Is the const specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let g : integer = UInt(Pg); let dn : integer = UInt(Zdn); let m : integer = UInt(Zm); let sub_i : boolean = (rot == '0'); let sub_r : boolean = (rot == '1');

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
<td><span id="T__37"></span>
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
<td class="symbol">RESERVED</td>
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
<td>&lt;Zm&gt;</td>
<td><span id="Zm"></span>
<p>Is the name of the second source scalable vector register, encoded in the "Zm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;const&gt;</td>
<td><span id="const__3"></span>
<p>Is the const specifier, encoded in “rot”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">rot</th>
<th class="symbol">&lt;const&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">#90</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">#270</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let pairs : integer = VL DIV (2 \* esize); let mask : bits(PL) = P{}(g); let operand1 : bits(VL) = Z{}(dn); let operand2 : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(m) else Zeros{VL}; var result : bits(VL); for p = 0 to pairs-1 do var acc_r : bits(esize) = operand1\[(2 \* p + 0)\*:esize\]; var acc_i : bits(esize) = operand1\[(2 \* p + 1)\*:esize\]; if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, 2 \* p + 0, esize) then var elt2_i : bits(esize) = operand2\[(2 \* p + 1)\*:esize\]; if sub_i then elt2_i = [FPNeg](shared_pseudocode.html#func_FPNeg_3){esize}(elt2_i, FPCR()); end; acc_r = [FPAdd](shared_pseudocode.html#func_FPAdd_4){esize}(acc_r, elt2_i, FPCR()); end; if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, 2 \* p + 1, esize) then var elt2_r : bits(esize) = operand2\[(2 \* p + 0)\*:esize\]; if sub_r then elt2_r = [FPNeg](shared_pseudocode.html#func_FPNeg_3){esize}(elt2_r, FPCR()); end; acc_i = [FPAdd](shared_pseudocode.html#func_FPAdd_4){esize}(acc_i, elt2_r, FPCR()); end; result\[(2 \* p + 0)\*:esize\] = acc_r; result\[(2 \* p + 1)\*:esize\] = acc_i; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;

</div>

### Operational information

This instruction might be immediately preceded in program order by a <span class="asm-code">MOVPRFX</span> instruction. The <span class="asm-code">MOVPRFX</span> must conform to all of the following requirements, otherwise the behavior of the <span class="asm-code">MOVPRFX</span> and this instruction is <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span>:

- The <span class="asm-code">MOVPRFX</span> can be predicated or unpredicated.
- A predicated <span class="asm-code">MOVPRFX</span> must use the same governing predicate register as this instruction.
- A predicated <span class="asm-code">MOVPRFX</span> must use the larger of the destination element size and first source element size in the preferred disassembly of this instruction.
- The <span class="asm-code">MOVPRFX</span> must specify the same destination register as this instruction.
- The destination register must not refer to architectural register state referenced by any other source operand register of this instruction.
