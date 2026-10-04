## SPLICE

Splice two vectors under predicate control

This instruction selects a region from the first source vector and copies it to the lowest-numbered elements of the result. Any remaining elements of the result are set to a copy of the lowest-numbered elements from the second source vector. The region is selected using the first and last true elements in the vector select predicate register. The result is destructively placed in the destination and first source vector, or constructively placed in the destination vector.

It has encodings from 2 classes: [Constructive](#iclass_constructive) and [Destructive](#iclass_destructive)

### <span id="iclass_constructive"></span>Constructive<span style="font-size:smaller;">\
(FEAT_SVE2 \|\| FEAT_SME) </span>

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
<td class="l">1</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="3" class="lr">Pv</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="splice_z_p_zz_con"></span>

SPLICE [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").[\<T\>](#T__86 "Is the size specifier, "), [\<Pv\>](#Pv__2 "Is the name of the vector select predicate register P0-P7, encoded in the "Pv" field."), { [\<Zn1\>](#Zn1__5 "Is the name of the first scalable vector register of the source multi-vector group, encoded in the "Zn" field.").[\<T\>](#T__86 "Is the size specifier, "), [\<Zn2\>](#Zn2__4 "Is the name of the second scalable vector register of the source multi-vector group, encoded in the "Zn" field.").[\<T\>](#T__86 "Is the size specifier, ") }

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let v : integer = UInt(Pv); let dst : integer = UInt(Zd); let s1 : integer = UInt(Zn); let s2 : integer = (s1 + 1) MOD 32;

### <span id="iclass_destructive"></span>Destructive<span style="font-size:smaller;">\
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
<td class="l">1</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="3" class="lr">Pv</td>
<td colspan="5" class="lr">Zm</td>
<td colspan="5" class="lr">Zdn</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="splice_z_p_zz_des"></span>

SPLICE [\<Zdn\>](#Zdn "Is the name of the first source and destination scalable vector register, encoded in the "Zdn" field.").[\<T\>](#T__86 "Is the size specifier, "), [\<Pv\>](#Pv__2 "Is the name of the vector select predicate register P0-P7, encoded in the "Pv" field."), [\<Zdn\>](#Zdn "Is the name of the first source and destination scalable vector register, encoded in the "Zdn" field.").[\<T\>](#T__86 "Is the size specifier, "), [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").[\<T\>](#T__86 "Is the size specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let v : integer = UInt(Pv); let dst : integer = UInt(Zdn); let s1 : integer = dst; let s2 : integer = UInt(Zm);

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
<td><span id="T__86"></span>
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
<td><span id="Pv__2"></span>
<p>Is the name of the vector select predicate register P0-P7, encoded in the "Pv" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn1&gt;</td>
<td><span id="Zn1__5"></span>
<p>Is the name of the first scalable vector register of the source multi-vector group, encoded in the "Zn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn2&gt;</td>
<td><span id="Zn2__4"></span>
<p>Is the name of the second scalable vector register of the source multi-vector group, encoded in the "Zn" field.</p></td>
</tr>
</tbody>
</table>

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

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(v); let operand1 : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(s1) else Zeros{VL}; let operand2 : bits(VL) = Z{}(s2); var result : bits(VL); var x : integer = 0; var active : boolean = FALSE; let lastnum : integer = [LastActiveElement](shared_pseudocode.html#func_LastActiveElement_3){PL}(mask, esize); if lastnum \>= 0 then for e = 0 to lastnum do active = active \|\| [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize); if active then result\[x\*:esize\] = operand1\[e\*:esize\]; x = x + 1; end; end; end; let nelements : integer = (elements - x) - 1; for e = 0 to nelements do result\[x\*:esize\] = operand2\[e\*:esize\]; x = x + 1; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dst) = result;

</div>

### Operational information

For the "Destructive" variant:

The destructive variant of this instruction might be immediately preceded in program order by a <span class="asm-code">MOVPRFX</span> instruction. The <span class="asm-code">MOVPRFX</span> must conform to all of the following requirements, otherwise the behavior of the <span class="asm-code">MOVPRFX</span> and the destructive variant of this instruction is <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span>:

- The <span class="asm-code">MOVPRFX</span> must be unpredicated.
- The <span class="asm-code">MOVPRFX</span> must specify the same destination register as this instruction.
- The destination register must not refer to architectural register state referenced by any other source operand register of this instruction.
