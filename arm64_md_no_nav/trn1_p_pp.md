## TRN1, TRN2 (predicates)

Interleave even or odd elements from two predicates

This instruction interleaves alternating even or odd-numbered elements from the first and second source predicates, and places the values in the destination predicate. This instruction is unpredicated.

It has encodings from 2 classes: [Even](#iclass_even) and [Odd](#iclass_odd)

### <span id="iclass_even"></span>Even<span style="font-size:smaller;">\
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
<td class="lr">0</td>
<td colspan="4" class="lr">Pm</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pn</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td></td>
<td colspan="4"></td>
<td colspan="3"></td>
<td colspan="2" class="droppedname">opc</td>
<td class="droppedname">H</td>
<td></td>
<td colspan="4"></td>
<td></td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="trn1_p_pp_"></span>

TRN1 [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").[\<T\>](#T__100 "Is the size specifier, "), [\<Pn\>](#Pn__2 "Is the name of the first source scalable predicate register, encoded in the "Pn" field.").[\<T\>](#T__100 "Is the size specifier, "), [\<Pm\>](#Pm__2 "Is the name of the second source scalable predicate register, encoded in the "Pm" field.").[\<T\>](#T__100 "Is the size specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let n : integer = UInt(Pn); let m : integer = UInt(Pm); let d : integer = UInt(Pd); let part : integer = 0;

### <span id="iclass_odd"></span>Odd<span style="font-size:smaller;">\
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
<td class="lr">0</td>
<td colspan="4" class="lr">Pm</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pn</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td></td>
<td colspan="4"></td>
<td colspan="3"></td>
<td colspan="2" class="droppedname">opc</td>
<td class="droppedname">H</td>
<td></td>
<td colspan="4"></td>
<td></td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="trn2_p_pp_"></span>

TRN2 [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").[\<T\>](#T__100 "Is the size specifier, "), [\<Pn\>](#Pn__2 "Is the name of the first source scalable predicate register, encoded in the "Pn" field.").[\<T\>](#T__100 "Is the size specifier, "), [\<Pm\>](#Pm__2 "Is the name of the second source scalable predicate register, encoded in the "Pm" field.").[\<T\>](#T__100 "Is the size specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let n : integer = UInt(Pn); let m : integer = UInt(Pm); let d : integer = UInt(Pd); let part : integer = 1;

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
<td><span id="T__100"></span>
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
<td>&lt;Pn&gt;</td>
<td><span id="Pn__2"></span>
<p>Is the name of the first source scalable predicate register, encoded in the "Pn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Pm&gt;</td>
<td><span id="Pm__2"></span>
<p>Is the name of the second source scalable predicate register, encoded in the "Pm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let pairs : integer = VL DIV (esize \* 2); let operand1 : bits(PL) = P{}(n); let operand2 : bits(PL) = P{}(m); var result : bits(PL); for p = 0 to pairs-1 do result\[(2\*p+0)\*:(esize DIV 8)\] = operand1\[(2\*p+part)\*:(esize DIV 8)\]; result\[(2\*p+1)\*:(esize DIV 8)\] = operand2\[(2\*p+part)\*:(esize DIV 8)\]; end; [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;

</div>
