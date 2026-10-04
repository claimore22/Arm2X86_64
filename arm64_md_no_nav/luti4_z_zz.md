## LUTI4 (8-bit and 16-bit)

Lookup table read with 4-bit indices (8-bit and 16-bit)

This instruction copies indexed 8-bit or 16-bit elements from the low 128 or 256 bits of the table vector, or from the low 128 bits of the two table vectors to the destination vector using packed 4-bit indices from a segment of the source vector. A segment corresponds to a portion of the source vector that is consumed in order to fill the destination vector. The segment is selected by the vector segment index. This instruction is unpredicated.

It has encodings from 3 classes: [Byte, single register table](#iclass_byte_single_register_table) , [Halfword, two register table](#iclass_halfword_two_register_table) and [Halfword, single register table](#iclass_halfword_single_register_table)

### <span id="iclass_byte_single_register_table"></span>Byte, single register table<span style="font-size:smaller;">\
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
<td class="lr">i1</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zm</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td class="r">1</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="luti4_z_zz_8"></span>

LUTI4 [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").B, { [\<Zn\>](#Zn__4 "Is the name of the table vector register, encoded in the "Zn" field.").B }, [\<Zm\>](#Zm__5 "Is the name of the source scalable vector register, encoded in the "Zm" field.")\[[\<index\>](#index__30 "For the "Byte, single register table" variant: is the vector segment index, in the range 0 to 1, encoded in the "i1" field.")\]

</div>

#### Decode for this encoding

if ((!IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME2)) \|\| !IsFeatureImplemented(FEAT_LUT)) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let isize : integer{} = 4; let esize : integer{} = 8; let ntblr : integer = 1; let m : integer = UInt(Zm); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let part : integer = UInt(i1);

### <span id="iclass_halfword_two_register_table"></span>Halfword, two register table<span style="font-size:smaller;">\
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
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td colspan="3"></td>
<td></td>
<td class="droppedname">op</td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="luti4_z_zz_2x16"></span>

LUTI4 [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").H, { [\<Zn1\>](#Zn1__7 "Is the name of the first table vector register, encoded as "Zn".").H, [\<Zn2\>](#Zn2__5 "Is the name of the second table vector register, encoded as "Zn" plus 1 modulo 32.").H }, [\<Zm\>](#Zm__5 "Is the name of the source scalable vector register, encoded in the "Zm" field.")\[[\<index\>](#index__27 "For the "Halfword, single register table" and "Halfword, two register table" variants: is the vector segment index, in the range 0 to 3, encoded in the "i2" field.")\]

</div>

#### Decode for this encoding

if ((!IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME2)) \|\| !IsFeatureImplemented(FEAT_LUT)) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let isize : integer{} = 4; let esize : integer{} = 16; let ntblr : integer = 2; let m : integer = UInt(Zm); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let part : integer = UInt(i2);

### <span id="iclass_halfword_single_register_table"></span>Halfword, single register table<span style="font-size:smaller;">\
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
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td colspan="3"></td>
<td></td>
<td class="droppedname">op</td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="luti4_z_zz_1x16"></span>

LUTI4 [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").H, { [\<Zn\>](#Zn__4 "Is the name of the table vector register, encoded in the "Zn" field.").H }, [\<Zm\>](#Zm__5 "Is the name of the source scalable vector register, encoded in the "Zm" field.")\[[\<index\>](#index__27 "For the "Halfword, single register table" and "Halfword, two register table" variants: is the vector segment index, in the range 0 to 3, encoded in the "i2" field.")\]

</div>

#### Decode for this encoding

if ((!IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME2)) \|\| !IsFeatureImplemented(FEAT_LUT)) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if [MaxImplementedAnyVL](shared_pseudocode.html#func_MaxImplementedAnyVL_0)() \< 256 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let isize : integer{} = 4; let esize : integer{} = 16; let ntblr : integer = 1; let m : integer = UInt(Zm); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let part : integer = UInt(i2);

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
<td><span id="index__30"></span>
<p>For the "Byte, single register table" variant: is the vector segment index, in the range 0 to 1, encoded in the "i1" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="index__27"></span>
<p>For the "Halfword, single register table" and "Halfword, two register table" variants: is the vector segment index, in the range 0 to 3, encoded in the "i2" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn1&gt;</td>
<td><span id="Zn1__7"></span>
<p>Is the name of the first table vector register, encoded as "Zn".</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn2&gt;</td>
<td><span id="Zn2__5"></span>
<p>Is the name of the second table vector register, encoded as "Zn" plus 1 modulo 32.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

if IsFeatureImplemented(FEAT_SME2) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); if ntblr == 1 && esize == 16 && VL \< 256 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let elements : integer = VL DIV esize; let tablesize : integer = if ntblr == 1 && esize == 16 then 256 else 128; let eltspertable : integer = tablesize DIV esize; let ibase : integer = elements \* part; let indexes : bits(VL) = Z{}(m); let table1 : bits(VL) = Z{}(n+0); let table2 : bits(VL) = if ntblr == 2 then [Z](shared_pseudocode.html#accessor_Z_2){VL}((n+1) MOD 32) else Zeros{VL}; var result : bits(VL); var res : bits(esize); for e = 0 to elements-1 do let index : integer = UInt(indexes\[(ibase + e)\*:isize\]); if index \< eltspertable then res = table1\[index\*:esize\]; else assert ntblr == 2; res = table2\[(index - eltspertable)\*:esize\]; end; result\[e\*:esize\] = res; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
