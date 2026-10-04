## FMIN (multiple and single vector)

Multi-vector floating-point minimum by vector

This instruction determines the minimum of floating-point elements of the second source vector and the corresponding floating-point elements of the two or four first source vectors and destructively places the results in the corresponding elements of the two or four first source vectors.

When FPCR.AH is 0, the behavior is as follows:

- Negative zero compares less than positive zero.
- When FPCR.DN is 0, if either element is a NaN, the result is a quiet NaN.
- When FPCR.DN is 1, if either element is a NaN, the result is Default NaN.

When FPCR.AH is 1, the behavior is as follows:

- If both elements are zeros, regardless of the sign of either zero, the result is the second element.
- If either element is a NaN, regardless of the value of FPCR.DN, the result is the second element.

This instruction follows SME2 floating-point numerical behaviors corresponding to instructions that place their results in one or more SVE Z vectors.

This instruction is unpredicated.

It has encodings from 2 classes: [Two registers](#iclass_two_registers) and [Four registers](#iclass_four_registers)

### <span id="iclass_two_registers"></span>Two registers<span style="font-size:smaller;">\
(FEAT_SME2) </span>

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
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">!= 00</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="4" class="lr">Zm</td>
<td class="l">1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Zdn</td>
<td class="lr">1</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td colspan="2"></td>
<td colspan="4"></td>
<td colspan="5"></td>
<td></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td class="droppedname">op</td>
<td colspan="4"></td>
<td class="droppedname">o2</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fmin_mz_zzv_2x1"></span>

FMIN { [\<Zdn1\>](#Zdn1 "For the "Two registers" variant: is the name of the first scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 2.").[\<T\>](#T__37 "Is the size specifier, ")-[\<Zdn2\>](#Zdn2 "Is the name of the second scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 2 plus 1.").[\<T\>](#T__37 "Is the size specifier, ") }, { [\<Zdn1\>](#Zdn1 "For the "Two registers" variant: is the name of the first scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 2.").[\<T\>](#T__37 "Is the size specifier, ")-[\<Zdn2\>](#Zdn2 "Is the name of the second scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 2 plus 1.").[\<T\>](#T__37 "Is the size specifier, ") }, [\<Zm\>](#Zm__2 "For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").[\<T\>](#T__37 "Is the size specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let dn : integer = UInt(Zdn::'0'); let m : integer = UInt('0'::Zm); let nreg : integer{} = 2;

### <span id="iclass_four_registers"></span>Four registers<span style="font-size:smaller;">\
(FEAT_SME2) </span>

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
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">!= 00</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="4" class="lr">Zm</td>
<td class="l">1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="3" class="lr">Zdn</td>
<td class="lr">0</td>
<td class="lr">1</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td colspan="2"></td>
<td colspan="4"></td>
<td colspan="5"></td>
<td></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td class="droppedname">op</td>
<td colspan="3"></td>
<td></td>
<td class="droppedname">o2</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fmin_mz_zzv_4x1"></span>

FMIN { [\<Zdn1\>](#Zdn1__2 "For the "Four registers" variant: is the name of the first scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 4.").[\<T\>](#T__37 "Is the size specifier, ")-[\<Zdn4\>](#Zdn4 "Is the name of the fourth scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 4 plus 3.").[\<T\>](#T__37 "Is the size specifier, ") }, { [\<Zdn1\>](#Zdn1__2 "For the "Four registers" variant: is the name of the first scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 4.").[\<T\>](#T__37 "Is the size specifier, ")-[\<Zdn4\>](#Zdn4 "Is the name of the fourth scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 4 plus 3.").[\<T\>](#T__37 "Is the size specifier, ") }, [\<Zm\>](#Zm__2 "For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").[\<T\>](#T__37 "Is the size specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let dn : integer = UInt(Zdn::'00'); let m : integer = UInt('0'::Zm); let nreg : integer{} = 4;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zdn1&gt;</td>
<td><span id="Zdn1"></span>
<p>For the "Two registers" variant: is the name of the first scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 2.</p></td>
</tr>
<tr>
<td></td>
<td><span id="Zdn1__2"></span>
<p>For the "Four registers" variant: is the name of the first scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 4.</p></td>
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
<td>&lt;Zdn2&gt;</td>
<td><span id="Zdn2"></span>
<p>Is the name of the second scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 2 plus 1.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zm&gt;</td>
<td><span id="Zm__2"></span>
<p>Is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zdn4&gt;</td>
<td><span id="Zdn4"></span>
<p>Is the name of the fourth scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 4 plus 3.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; var results : array \[\[4\]\] of bits(VL); for r = 0 to nreg-1 do let operand1 : bits(VL) = Z{}(dn+r); let operand2 : bits(VL) = Z{}(m); for e = 0 to elements-1 do let element1 : bits(esize) = operand1\[e\*:esize\]; let element2 : bits(esize) = operand2\[e\*:esize\]; results\[\[r\]\]\[e\*:esize\] = [FPMin](shared_pseudocode.html#func_FPMin_4){esize}(element1, element2, FPCR()); end; end; for r = 0 to nreg-1 do [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn+r) = results\[\[r\]\]; end;

</div>
