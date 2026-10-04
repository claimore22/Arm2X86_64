## PNEXT

Find next active predicate

This instruction constructs a loop that iterates over all true elements in the vector select predicate register. If all elements in the first source predicate register are false it determines the first true element in the vector select predicate register, otherwise it determines the next true element in the vector select predicate register that follows the last true element in the first source predicate register. All elements of the destination predicate register are set to false, except the element corresponding to the determined vector select element, if any, which is set to true. This instruction sets the First (N), None (Z), and !Last (C) condition flags based on the predicate result, and sets the V flag to zero.

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
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">size</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td class="r">1</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="4" class="lr">Pv</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pdn</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="pnext_p_p_p_"></span>

PNEXT [\<Pdn\>](#Pdn__2 "Is the name of the first source and destination scalable predicate register, encoded in the "Pdn" field.").[\<T\>](#T__69 "Is the size specifier, "), [\<Pv\>](#Pv "Is the name of the vector select predicate register, encoded in the "Pv" field."), [\<Pdn\>](#Pdn__2 "Is the name of the first source and destination scalable predicate register, encoded in the "Pdn" field.").[\<T\>](#T__69 "Is the size specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let v : integer = UInt(Pv); let dn : integer = UInt(Pdn);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Pdn&gt;</td>
<td><span id="Pdn__2"></span>
<p>Is the name of the first source and destination scalable predicate register, encoded in the "Pdn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__69"></span>
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

<table>
<tbody>
<tr>
<td>&lt;Pv&gt;</td>
<td><span id="Pv"></span>
<p>Is the name of the vector select predicate register, encoded in the "Pv" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer{} = VL DIV esize; let mask : bits(PL) = P{}(v); let operand : bits(PL) = P{}(dn); var result : bits(PL) = Zeros{PL}; let psize : integer{} = esize DIV 8; var next : integer = [LastActiveElement](shared_pseudocode.html#func_LastActiveElement_3){PL}(operand, esize) + 1; while next \< elements && (\![ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, next, esize)) looplimit elements do next = next + 1; end; if next \< elements then result\[next\*:psize\] = ZeroExtend{psize}('1'); end; PSTATE.\[N,Z,C,V\] = [PredTest](shared_pseudocode.html#func_PredTest_4){PL}(mask, result, esize); [P](shared_pseudocode.html#accessor_P_2){PL}(dn) = result;

</div>

### Operational information

If FEAT_SME is implemented and the PE is in Streaming SVE mode, then any subsequent instruction which is dependent on the NZCV condition flags written by this instruction might be significantly delayed.
