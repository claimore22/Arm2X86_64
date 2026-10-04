## NMATCH

Detect no matching elements, setting the condition flags

This instruction compares each active 8-bit or 16-bit element in the first source vector with all of the elements in the corresponding 128-bit segment of the second source vector. For each element in the first source vector, if no matching element is found in the corresponding segment of the second source vector, the corresponding element of the destination predicate is set to true, otherwise it is set to false. Inactive elements in the destination predicate register are set to zero. This instruction sets the First (N), None (Z), and !Last (C) condition flags based on the predicate result, and sets the V flag to zero.

This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

### <span id="iclass_byte_and_halfword"></span>Byte and halfword<span style="font-size:smaller;">\
(FEAT_SVE2) </span>

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
<td class="lr">0</td>
<td class="lr">sz</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zm</td>
<td class="l">1</td>
<td>0</td>
<td class="r">0</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">1</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td colspan="5"></td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td class="droppedname">op</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="nmatch_p_p_zz_"></span>

NMATCH [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").[\<T\>](#T__66 "Is the size specifier, "), [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/Z, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__66 "Is the size specifier, "), [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").[\<T\>](#T__66 "Is the size specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(sz); let g : integer = UInt(Pg); let d : integer = UInt(Pd); let n : integer = UInt(Zn); let m : integer = UInt(Zm);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Pd&gt;</td>
<td><span id="Pd"></span>
<p>Is the name of the destination scalable predicate register, encoded in the "Pd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__66"></span>
<p>Is the size specifier, encoded in “sz”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">sz</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">H</td>
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
<td>&lt;Zn&gt;</td>
<td><span id="Zn__2"></span>
<p>Is the name of the first source scalable vector register, encoded in the "Zn" field.</p></td>
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

[CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let eltspersegment : integer = 128 DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(n) else Zeros{VL}; let operand2 : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(m) else Zeros{VL}; var result : bits(PL); let psize : integer{} = esize DIV 8; for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then let segmentbase : integer = e - (e MOD eltspersegment); result\[e\*:psize\] = ZeroExtend{psize}('1'); let element1 : bits(esize) = operand1\[e\*:esize\]; for i = segmentbase to (segmentbase + eltspersegment) - 1 do let element2 : bits(esize) = operand2\[i\*:esize\]; if element1 == element2 then result\[e\*:psize\] = ZeroExtend{psize}('0'); end; end; else result\[e\*:psize\] = ZeroExtend{psize}('0'); end; end; PSTATE.\[N,Z,C,V\] = [PredTest](shared_pseudocode.html#func_PredTest_4){PL}(mask, result, esize); [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;

</div>
