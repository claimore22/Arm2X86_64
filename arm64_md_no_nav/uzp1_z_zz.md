## UZP1, UZP2 (vectors)

Concatenate even or odd elements from two vectors

This instruction concatenates adjacent even or odd-numbered elements from the first and second source vectors, and places the values in the destination vector. This instruction is unpredicated.

Note: UZP1 is equivalent to truncating and packing each element from two source vectors into a single destination vector with elements of half the size.

The 128-bit element variant requires that the Effective SVE vector length is at least 256 bits. ID_AA64ZFR0_EL1.F64MM indicates whether the 128-bit element variant is implemented. The 128-bit element variant is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

It has encodings from 4 classes: [Even](#iclass_even) , [Even (quadwords)](#iclass_even_quadwords_) , [Odd](#iclass_odd) and [Odd (quadwords)](#iclass_odd_quadwords_)

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
<td colspan="5" class="lr">Zm</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">0</td>
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
<td colspan="2"></td>
<td class="droppedname">H</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="uzp1_z_zz_"></span>

UZP1 [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").[\<T\>](#T__5 "Is the size specifier, "), [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__5 "Is the size specifier, "), [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").[\<T\>](#T__5 "Is the size specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd); let part : integer = 0;

### <span id="iclass_even_quadwords_"></span>Even (quadwords)<span style="font-size:smaller;">\
(FEAT_F64MM) </span>

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
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zm</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td colspan="2"></td>
<td></td>
<td></td>
<td colspan="5"></td>
<td colspan="3"></td>
<td colspan="2" class="droppedname">opc</td>
<td class="droppedname">H</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="uzp1_z_zz_q"></span>

UZP1 [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").Q, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").Q, [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").Q

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_F64MM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 128; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd); let part : integer = 0;

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
<td colspan="5" class="lr">Zm</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">1</td>
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
<td colspan="2"></td>
<td class="droppedname">H</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="uzp2_z_zz_"></span>

UZP2 [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").[\<T\>](#T__5 "Is the size specifier, "), [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__5 "Is the size specifier, "), [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").[\<T\>](#T__5 "Is the size specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd); let part : integer = 1;

### <span id="iclass_odd_quadwords_"></span>Odd (quadwords)<span style="font-size:smaller;">\
(FEAT_F64MM) </span>

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
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zm</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td colspan="2"></td>
<td></td>
<td></td>
<td colspan="5"></td>
<td colspan="3"></td>
<td colspan="2" class="droppedname">opc</td>
<td class="droppedname">H</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="uzp2_z_zz_q"></span>

UZP2 [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").Q, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").Q, [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").Q

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_F64MM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 128; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd); let part : integer = 1;

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
<td><span id="T__5"></span>
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

if esize \< 128 then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); if VL \< esize \* 2 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let pairs : integer = VL DIV (esize \* 2); let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL) = Zeros{}; for p = 0 to pairs - 1 do result\[p\*:esize\] = operand1\[(2\*p+part)\*:esize\]; end; for p = 0 to pairs - 1 do result\[(pairs+p)\*:esize\] = operand2\[(2\*p+part)\*:esize\]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
