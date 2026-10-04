## LUTI4 (four registers, 8-bit)

Lookup table read with 4-bit indexes and 8-bit elements (four registers)

This instruction copies 8-bit elements from ZT0 to four destination vectors using packed 4-bit indices in the two source vectors.

This instruction is unpredicated.

It has encodings from 2 classes: [Consecutive](#iclass_consecutive) and [Strided](#iclass_strided)

### <span id="iclass_consecutive"></span>Consecutive<span style="font-size:smaller;">\
(FEAT_SME_LUTv2) </span>

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
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td colspan="2" class="lr">size</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="4" class="lr">Zn</td>
<td class="lr">0</td>
<td colspan="3" class="lr">Zd</td>
<td class="l">0</td>
<td class="r">0</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td colspan="2"></td>
<td></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="2"></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="4"></td>
<td></td>
<td colspan="3"></td>
<td colspan="2"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="luti4_mz4_ztmz2_1"></span>

LUTI4 { [\<Zd1\>](#Zd1__2 "For the "Consecutive" variant: is the name of the first scalable vector register of the destination multi-vector group, encoded as "Zd" times 4.").B-[\<Zd4\>](#Zd4 "For the "Consecutive" variant: is the name of the fourth scalable vector register of the destination multi-vector group, encoded as "Zd" times 4 plus 3.").B }, ZT0, { [\<Zn1\>](#Zn1__4 "Is the name of the first scalable vector register of the source multi-vector group, encoded as "Zn" times 2.")-[\<Zn2\>](#Zn2__3 "Is the name of the second scalable vector register of the source multi-vector group, encoded as "Zn" times 2 plus 1.") }

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME_LUTv2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size != '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let isize : integer{} = 4; let n : integer = UInt(Zn::'0'); let dstride : integer = 1; let d : integer = UInt(Zd::'00'); let nreg : integer{} = 4;

### <span id="iclass_strided"></span>Strided<span style="font-size:smaller;">\
(FEAT_SME2p1 && FEAT_SME_LUTv2) </span>

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
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="2" class="lr">size</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="4" class="lr">Zn</td>
<td class="lr">0</td>
<td class="lr">D</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="2" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td colspan="6"></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="4"></td>
<td></td>
<td></td>
<td colspan="2"></td>
<td colspan="2"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="luti4_mz4_ztmz2_4"></span>

LUTI4 { [\<Zd1\>](#Zd1__4 "For the "Strided" variant: is the name of the first scalable vector register Z0-Z3 or Z16-Z19 of the destination multi-vector group, encoded as "D:'00':Zd".").B, [\<Zd2\>](#Zd2__3 "Is the name of the second scalable vector register Z4-Z7 or Z20-Z23 of the destination multi-vector group, encoded as "D:'01':Zd".").B, [\<Zd3\>](#Zd3 "Is the name of the third scalable vector register Z8-Z11 or Z24-Z27 of the destination multi-vector group, encoded as "D:'10':Zd".").B, [\<Zd4\>](#Zd4__2 "For the "Strided" variant: is the name of the fourth scalable vector register Z12-Z15 or Z28-Z31 of the destination multi-vector group, encoded as "D:'11':Zd".").B }, ZT0, { [\<Zn1\>](#Zn1__4 "Is the name of the first scalable vector register of the source multi-vector group, encoded as "Zn" times 2.")-[\<Zn2\>](#Zn2__3 "Is the name of the second scalable vector register of the source multi-vector group, encoded as "Zn" times 2 plus 1.") }

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2p1) \|\| !IsFeatureImplemented(FEAT_SME_LUTv2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size != '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let isize : integer{} = 4; let n : integer = UInt(Zn::'0'); let dstride : integer = 4; let d : integer = UInt(D::'00'::Zd); let nreg : integer{} = 4;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zd1&gt;</td>
<td><span id="Zd1__2"></span>
<p>For the "Consecutive" variant: is the name of the first scalable vector register of the destination multi-vector group, encoded as "Zd" times 4.</p></td>
</tr>
<tr>
<td></td>
<td><span id="Zd1__4"></span>
<p>For the "Strided" variant: is the name of the first scalable vector register Z0-Z3 or Z16-Z19 of the destination multi-vector group, encoded as "D:'00':Zd".</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zd4&gt;</td>
<td><span id="Zd4"></span>
<p>For the "Consecutive" variant: is the name of the fourth scalable vector register of the destination multi-vector group, encoded as "Zd" times 4 plus 3.</p></td>
</tr>
<tr>
<td></td>
<td><span id="Zd4__2"></span>
<p>For the "Strided" variant: is the name of the fourth scalable vector register Z12-Z15 or Z28-Z31 of the destination multi-vector group, encoded as "D:'11':Zd".</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn1&gt;</td>
<td><span id="Zn1__4"></span>
<p>Is the name of the first scalable vector register of the source multi-vector group, encoded as "Zn" times 2.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn2&gt;</td>
<td><span id="Zn2__3"></span>
<p>Is the name of the second scalable vector register of the source multi-vector group, encoded as "Zn" times 2 plus 1.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zd2&gt;</td>
<td><span id="Zd2__3"></span>
<p>Is the name of the second scalable vector register Z4-Z7 or Z20-Z23 of the destination multi-vector group, encoded as "D:'01':Zd".</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zd3&gt;</td>
<td><span id="Zd3"></span>
<p>Is the name of the third scalable vector register Z8-Z11 or Z24-Z27 of the destination multi-vector group, encoded as "D:'10':Zd".</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); [CheckSMEZT0Enabled](shared_pseudocode.html#func_CheckSMEZT0Enabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let indexes : bits(2\*VL) = [Z](shared_pseudocode.html#accessor_Z_2){VL}(n+1) :: [Z](shared_pseudocode.html#accessor_Z_2){VL}(n+0); var dst : integer = d; let table : bits(512) = ZT0{}(); for r = 0 to nreg-1 do let base : integer = r \* elements; var result : bits(VL); for e = 0 to elements-1 do let index : integer = UInt(indexes\[(base+e)\*:isize\]); result\[e\*:esize\] = table\[index\*:32\]\[esize-1:0\]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dst) = result; dst = dst + dstride; end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
