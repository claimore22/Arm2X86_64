## CLASTB (scalar)

Conditionally extract last element to general-purpose register

This instruction extracts and zero-extends the Last active element from the source vector register, and destructively places the zero-extended value in the destination and first source general-purpose register. If there are no Active elements, the least significant element-size bits of the destination and first source general-purpose register are destructively zero-extended.

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
<td class="lr">1</td>
<td colspan="2" class="lr">size</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zm</td>
<td colspan="5" class="lr">Rdn</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td></td>
<td colspan="3"></td>
<td class="droppedname">B</td>
<td colspan="2"></td>
<td></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="clastb_r_p_z_"></span>

CLASTB [\<R\>](#R__2 "Is a width specifier, ")[\<dn\>](#dn "Is the number [0-30] of the source and destination general-purpose register or the name ZR (31), encoded in the "Rdn" field."), [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field."), [\<R\>](#R__2 "Is a width specifier, ")[\<dn\>](#dn "Is the number [0-30] of the source and destination general-purpose register or the name ZR (31), encoded in the "Rdn" field."), [\<Zm\>](#Zm__5 "Is the name of the source scalable vector register, encoded in the "Zm" field.").[\<T\>](#T__19 "Is the size specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let g : integer = UInt(Pg); let dn : integer = UInt(Rdn); let m : integer = UInt(Zm); let csize : integer{} = if esize \< 64 then 32 else 64; let isBefore : boolean = TRUE;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;R&gt;</td>
<td><span id="R__2"></span>
<p>Is a width specifier, encoded in “size”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">size</th>
<th class="symbol">&lt;R&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">W</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">W</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">W</td>
</tr>
<tr>
<td class="bitfield">11</td>
<td class="symbol">X</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;dn&gt;</td>
<td><span id="dn"></span>
<p>Is the number [0-30] of the source and destination general-purpose register or the name ZR (31), encoded in the "Rdn" field.</p></td>
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
<td><span id="Zm__5"></span>
<p>Is the name of the source scalable vector register, encoded in the "Zm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__19"></span>
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
<td class="symbol">D</td>
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

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(esize) = X{}(dn); let operand2 : bits(VL) = Z{}(m); var result : bits(csize); var last : integer = [LastActiveElement](shared_pseudocode.html#func_LastActiveElement_3){PL}(mask, esize); if last \< 0 then result = ZeroExtend{csize}(operand1); else if !isBefore then last = last + 1; if last \>= elements then last = 0; end; end; result = ZeroExtend{csize}(operand2\[last\*:esize\]); end; [X](shared_pseudocode.html#accessor_X_2){csize}(dn) = result;

</div>

### Operational information

If FEAT_SME is implemented and the PE is in Streaming SVE mode, then any subsequent instruction which is dependent on the general-purpose register written by this instruction might be significantly delayed.
