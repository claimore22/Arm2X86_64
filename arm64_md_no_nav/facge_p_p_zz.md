## FAC\<cc\>

Floating-point absolute compare

This instruction compares active absolute values of floating-point elements in the first source vector with the corresponding absolute values of elements in the second source vector, and places the boolean results of the specified comparison in the corresponding elements of the destination predicate. Inactive elements in the destination predicate register are set to zero. This instruction does not set the condition flags.

| \<cc\> | Comparison            |
|--------|-----------------------|
| GE     | greater than or equal |
| GT     | greater than          |
| LE     | less than or equal    |
| LT     | less than             |

This instruction is used by the pseudo-instructions [FACLE](facle_facge_p_p_zz.html "Floating-point absolute compare less than or equal"), and [FACLT](faclt_facge_p_p_zz.html "Floating-point absolute compare less than").

It has encodings from 2 classes: [Greater than](#iclass_greater_than) and [Greater than or equal](#iclass_greater_than_or_equal)

### <span id="iclass_greater_than"></span>Greater than<span style="font-size:smaller;">\
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
<td class="lr">1</td>
<td colspan="2" class="lr">size</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Zm</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">1</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td class="droppedname">op</td>
<td></td>
<td class="droppedname">o2</td>
<td colspan="3"></td>
<td colspan="5"></td>
<td class="droppedname">o3</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="facgt_p_p_zz_"></span>

FACGT [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").[\<T\>](#T__39 "Is the size specifier, "), [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/Z, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__39 "Is the size specifier, "), [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").[\<T\>](#T__39 "Is the size specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Pd); let cmp_op : [CmpOp](shared_pseudocode.html#type_CmpOp) = [Cmp_GT](shared_pseudocode.html#enum_Cmp_GT);

### <span id="iclass_greater_than_or_equal"></span>Greater than or equal<span style="font-size:smaller;">\
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
<td class="lr">1</td>
<td colspan="2" class="lr">size</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Zm</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">1</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td class="droppedname">op</td>
<td></td>
<td class="droppedname">o2</td>
<td colspan="3"></td>
<td colspan="5"></td>
<td class="droppedname">o3</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="facge_p_p_zz_"></span>

FACGE [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").[\<T\>](#T__39 "Is the size specifier, "), [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/Z, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__39 "Is the size specifier, "), [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").[\<T\>](#T__39 "Is the size specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Pd); let cmp_op : [CmpOp](shared_pseudocode.html#type_CmpOp) = [Cmp_GE](shared_pseudocode.html#enum_Cmp_GE);

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
<td><span id="T__39"></span>
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

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(n) else Zeros{VL}; let operand2 : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(m) else Zeros{VL}; var result : bits(PL); let psize : integer{} = esize DIV 8; for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then let element1 : bits(esize) = FPAbs{}(operand1\[e\*:esize\], FPCR()); let element2 : bits(esize) = FPAbs{}(operand2\[e\*:esize\], FPCR()); var res : boolean; case cmp_op of when [Cmp_GE](shared_pseudocode.html#enum_Cmp_GE) =\> res = [FPCompareGE](shared_pseudocode.html#func_FPCompareGE_4){esize}(element1, element2, FPCR()); when [Cmp_GT](shared_pseudocode.html#enum_Cmp_GT) =\> res = [FPCompareGT](shared_pseudocode.html#func_FPCompareGT_4){esize}(element1, element2, FPCR()); end; let pbit : bit = if res then '1' else '0'; result\[e\*:psize\] = ZeroExtend{psize}(pbit); else result\[e\*:psize\] = ZeroExtend{psize}('0'); end; end; [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;

</div>

### Operational information

If FEAT_SME is implemented and the PE is in Streaming SVE mode, then any subsequent instruction which is dependent on the predicate register written by this instruction might be significantly delayed.
