## LUTI2 (8-bit and 16-bit)

Lookup table read with 2-bit indices (8-bit and 16-bit)

This instruction copies indexed 8-bit or 16-bit elements from the low 128 bits of the table vector to the destination vector using packed 2-bit indices from a segment of the source vector. A segment corresponds to a portion of the source vector that is consumed in order to fill the destination vector. The segment is selected by the vector segment index. This instruction is unpredicated.

It has encodings from 2 classes: [Byte](#iclass_byte) and [Halfword](#iclass_halfword)

### <span id="iclass_byte"></span>Byte<span style="font-size:smaller;">\
((FEAT_SVE2 \|\| FEAT_SME2) && FEAT_LUT) </span>

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
<td colspan="2" class="lr">i2</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zm</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">1</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="luti2_z_zz_8"></span>

LUTI2 [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").B, { [\<Zn\>](#Zn__4 "Is the name of the table vector register, encoded in the "Zn" field.").B }, [\<Zm\>](#Zm__5 "Is the name of the source scalable vector register, encoded in the "Zm" field.")\[[\<index\>](#index__27 "For the "Byte" variant: is the vector segment index, in the range 0 to 3, encoded in the "i2" field.")\]

</div>

#### Decode for this encoding

if ((!IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME2)) \|\| !IsFeatureImplemented(FEAT_LUT)) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let isize : integer{} = 2; let esize : integer{} = 8; let m : integer = UInt(Zm); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let part : integer = UInt(i2);

### <span id="iclass_halfword"></span>Halfword<span style="font-size:smaller;">\
((FEAT_SVE2 \|\| FEAT_SME2) && FEAT_LUT) </span>

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
<td colspan="2" class="lr">i3h</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zm</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">i3l</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="luti2_z_zz_16"></span>

LUTI2 [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").H, { [\<Zn\>](#Zn__4 "Is the name of the table vector register, encoded in the "Zn" field.").H }, [\<Zm\>](#Zm__5 "Is the name of the source scalable vector register, encoded in the "Zm" field.")\[[\<index\>](#index__29 "For the "Halfword" variant: is the vector segment index, in the range 0 to 7, encoded in the "i3h:i3l" fields.")\]

</div>

#### Decode for this encoding

if ((!IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME2)) \|\| !IsFeatureImplemented(FEAT_LUT)) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let isize : integer{} = 2; let esize : integer{} = 16; let m : integer = UInt(Zm); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let part : integer = UInt(i3h::i3l);

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
<td>&lt;Zn&gt;</td>
<td><span id="Zn__4"></span>
<p>Is the name of the table vector register, encoded in the "Zn" field.</p></td>
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
<td>&lt;index&gt;</td>
<td><span id="index__27"></span>
<p>For the "Byte" variant: is the vector segment index, in the range 0 to 3, encoded in the "i2" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="index__29"></span>
<p>For the "Halfword" variant: is the vector segment index, in the range 0 to 7, encoded in the "i3h:i3l" fields.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

if IsFeatureImplemented(FEAT_SME2) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let ibase : integer = elements \* part; let indexes : bits(VL) = Z{}(m); let table : bits(VL) = Z{}(n); var result : bits(VL); for e = 0 to elements-1 do let index : integer = UInt(indexes\[(ibase + e)\*:isize\]); result\[e\*:esize\] = table\[index\*:esize\]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
