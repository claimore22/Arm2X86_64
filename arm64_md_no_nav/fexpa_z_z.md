## FEXPA

Floating-point exponential accelerator

This instruction calculates an exponentiation acceleration operation on each floating-point element in the source vector, where the result sign is zero, the result exponent field is copied from a set of significant bits of the input fraction, and the result fraction is inserted from a lookup table indexed by the least-significant input fraction bits, and returns each result in the corresponding element of the destination vector.

This instruction is fully defined by its bit-manipulation semantics, does not generate floating-point exceptions, and does not guarantee NaN propagation.

For double-precision variants, the result element exponent is copied from the source element bits\<16:6\>, and the result fraction is set based on the source element to the rounded value of 2<sup>52</sup> × (2<sup>bits\<5:0\>/64</sup> - 1).

For single-precision variants, the result element exponent is copied from the source element bits\<13:6\>, and the result fraction is set based on the source element to the rounded value of 2<sup>23</sup> × (2<sup>bits\<5:0\>/64</sup> - 1).

For half-precision variants, the result element exponent is copied from the source element bits\<9:5\>, and the result fraction is set based on the source element to the rounded value of 2<sup>10</sup> × (2<sup>bits\<4:0\>/32</sup> - 1).

This instruction is unpredicated.

This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled, or FEAT_SSVE_FEXPA is implemented.

<div class="note">

------------------------------------------------------------------------

#### Note

For a double-precision input value x in the range 70,368,744,177,655 \<= x \< 70,368,744,179,711, the operation performed by this instruction is equivalent to calculating 2<sup>x-70,368,744,178,687</sup>.

For a single-precision input value x in the range 131,073 \<= x \< 131,327, the operation performed by this instruction is equivalent to calculating 2<sup>x-131,199</sup>.

For a half-precision input value x in the range 33 \<= x \< 63, the operation performed by this instruction is equivalent to calculating 2<sup>x-47</sup>.

------------------------------------------------------------------------

</div>

### <span id="iclass_sve"></span>SVE<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SSVE_FEXPA) </span>

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
<td class="lr">1</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td>1</td>
<td class="r">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="5" class="droppedname">opc</td>
<td colspan="4"></td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fexpa_z_z_"></span>

FEXPA [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").[\<T\>](#T__50 "Is the size specifier, "), [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__50 "Is the size specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SSVE_FEXPA) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let n : integer = UInt(Zn); let d : integer = UInt(Zd);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zd&gt;</td>
<td><span id="Zd"></span>
<p>Is the name of the destination scalable vector register, encoded in the "Zd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__50"></span>
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
<td>&lt;Zn&gt;</td>
<td><span id="Zn"></span>
<p>Is the name of the source scalable vector register, encoded in the "Zn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

if IsFeatureImplemented(FEAT_SSVE_FEXPA) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand : bits(VL) = Z{}(n); var result : bits(VL); for e = 0 to elements-1 do let element : bits(esize) = operand\[e\*:esize\]; result\[e\*:esize\] = [FPExpA](shared_pseudocode.html#func_FPExpA_2){esize}(element); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;

</div>
