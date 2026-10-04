## MOVA (tile to vector, single)

Move ZA tile slice to Z vector

This instruction operates on individual horizontal or vertical slices within a named ZA tile of the specified element size. The slice number within the tile is selected by the sum of the slice index register and immediate offset, modulo the number of such elements in a vector. The immediate offset is in the range 0 to the number of elements in a 128-bit vector segment minus 1.

Inactive elements in the destination vector remain unmodified.

This instruction is used by the alias [MOV (tile to vector, single)](mov_mova_z_p_rza.html "Move ZA tile slice to Z vector").

It has encodings from 5 classes: [8-bit](#iclass_8_bit) , [16-bit](#iclass_16_bit) , [32-bit](#iclass_32_bit) , [64-bit](#iclass_64_bit) and [128-bit](#iclass_128_bit)

### <span id="iclass_8_bit"></span>8-bit<span style="font-size:smaller;">\
(FEAT_SME) </span>

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
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">V</td>
<td colspan="2" class="lr">Rs</td>
<td colspan="3" class="lr">Pg</td>
<td class="lr">0</td>
<td colspan="4" class="lr">off4</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td colspan="3"></td>
<td></td>
<td></td>
<td class="droppedname">Q</td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td></td>
<td colspan="4"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="mova_z_p_rza_b"></span>

MOVA [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").B, [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/M, ZA0[\<HV\>](#HV__4 "Is the horizontal or vertical slice indicator, ").B\[[\<Ws\>](#Ws__3 "Is the 32-bit name of the slice index register W12-W15, encoded in the "Rs" field."), [\<offs\>](#offs__2 "For the "8-bit" variant: is the slice index offset, in the range 0 to 15, encoded in the "off4" field.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let g : integer = UInt(Pg); let s : integer = UInt('011'::Rs); let n : integer = 0; let offset : integer = UInt(off4); let esize : integer{} = 8; let d : integer = UInt(Zd); let vertical : boolean = V == '1';

### <span id="iclass_16_bit"></span>16-bit<span style="font-size:smaller;">\
(FEAT_SME) </span>

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
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">V</td>
<td colspan="2" class="lr">Rs</td>
<td colspan="3" class="lr">Pg</td>
<td class="lr">0</td>
<td class="lr">ZAn</td>
<td colspan="3" class="lr">off3</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td colspan="3"></td>
<td></td>
<td></td>
<td class="droppedname">Q</td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td></td>
<td></td>
<td colspan="3"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="mova_z_p_rza_h"></span>

MOVA [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").H, [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/M, [\<ZAn\>](#ZAn__2 "For the "16-bit" variant: is the name of the ZA tile ZA0-ZA1 to be accessed, encoded in the "ZAn" field.")[\<HV\>](#HV__4 "Is the horizontal or vertical slice indicator, ").H\[[\<Ws\>](#Ws__3 "Is the 32-bit name of the slice index register W12-W15, encoded in the "Rs" field."), [\<offs\>](#offs__4 "For the "16-bit" variant: is the slice index offset, in the range 0 to 7, encoded in the "off3" field.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let g : integer = UInt(Pg); let s : integer = UInt('011'::Rs); let n : integer = UInt(ZAn); let offset : integer = UInt(off3); let esize : integer{} = 16; let d : integer = UInt(Zd); let vertical : boolean = V == '1';

### <span id="iclass_32_bit"></span>32-bit<span style="font-size:smaller;">\
(FEAT_SME) </span>

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
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">V</td>
<td colspan="2" class="lr">Rs</td>
<td colspan="3" class="lr">Pg</td>
<td class="lr">0</td>
<td colspan="2" class="lr">ZAn</td>
<td colspan="2" class="lr">off2</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td colspan="3"></td>
<td></td>
<td></td>
<td class="droppedname">Q</td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="mova_z_p_rza_w"></span>

MOVA [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").S, [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/M, [\<ZAn\>](#ZAn__3 "For the "32-bit" variant: is the name of the ZA tile ZA0-ZA3 to be accessed, encoded in the "ZAn" field.")[\<HV\>](#HV__4 "Is the horizontal or vertical slice indicator, ").S\[[\<Ws\>](#Ws__3 "Is the 32-bit name of the slice index register W12-W15, encoded in the "Rs" field."), [\<offs\>](#offs__6 "For the "32-bit" variant: is the slice index offset, in the range 0 to 3, encoded in the "off2" field.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let g : integer = UInt(Pg); let s : integer = UInt('011'::Rs); let n : integer = UInt(ZAn); let offset : integer = UInt(off2); let esize : integer{} = 32; let d : integer = UInt(Zd); let vertical : boolean = V == '1';

### <span id="iclass_64_bit"></span>64-bit<span style="font-size:smaller;">\
(FEAT_SME) </span>

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
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">V</td>
<td colspan="2" class="lr">Rs</td>
<td colspan="3" class="lr">Pg</td>
<td class="lr">0</td>
<td colspan="3" class="lr">ZAn</td>
<td class="lr">o1</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td colspan="3"></td>
<td></td>
<td></td>
<td class="droppedname">Q</td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td></td>
<td colspan="3"></td>
<td></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="mova_z_p_rza_d"></span>

MOVA [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").D, [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/M, [\<ZAn\>](#ZAn "For the "64-bit" variant: is the name of the ZA tile ZA0-ZA7 to be accessed, encoded in the "ZAn" field.")[\<HV\>](#HV__4 "Is the horizontal or vertical slice indicator, ").D\[[\<Ws\>](#Ws__3 "Is the 32-bit name of the slice index register W12-W15, encoded in the "Rs" field."), [\<offs\>](#offs__3 "For the "64-bit" variant: is the slice index offset, in the range 0 to 1, encoded in the "o1" field.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let g : integer = UInt(Pg); let s : integer = UInt('011'::Rs); let n : integer = UInt(ZAn); let offset : integer = UInt(o1); let esize : integer{} = 64; let d : integer = UInt(Zd); let vertical : boolean = V == '1';

### <span id="iclass_128_bit"></span>128-bit<span style="font-size:smaller;">\
(FEAT_SME) </span>

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
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">V</td>
<td colspan="2" class="lr">Rs</td>
<td colspan="3" class="lr">Pg</td>
<td class="lr">0</td>
<td colspan="4" class="lr">ZAn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td colspan="3"></td>
<td></td>
<td></td>
<td class="droppedname">Q</td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td></td>
<td colspan="4"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="mova_z_p_rza_q"></span>

MOVA [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").Q, [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/M, [\<ZAn\>](#ZAn__4 "For the "128-bit" variant: is the name of the ZA tile ZA0-ZA15 to be accessed, encoded in the "ZAn" field.")[\<HV\>](#HV__4 "Is the horizontal or vertical slice indicator, ").Q\[[\<Ws\>](#Ws__3 "Is the 32-bit name of the slice index register W12-W15, encoded in the "Rs" field."), [\<offs\>](#offs__5 "For the "128-bit" variant: is the slice index offset, with implicit value 0.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let g : integer = UInt(Pg); let s : integer = UInt('011'::Rs); let n : integer = UInt(ZAn); let offset : integer = 0; let esize : integer{} = 128; let d : integer = UInt(Zd); let vertical : boolean = V == '1';

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
<td>&lt;Pg&gt;</td>
<td><span id="Pg"></span>
<p>Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;HV&gt;</td>
<td><span id="HV__4"></span>
<p>Is the horizontal or vertical slice indicator, encoded in “V”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">V</th>
<th class="symbol">&lt;HV&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">V</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Ws&gt;</td>
<td><span id="Ws__3"></span>
<p>Is the 32-bit name of the slice index register W12-W15, encoded in the "Rs" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;offs&gt;</td>
<td><span id="offs__2"></span>
<p>For the "8-bit" variant: is the slice index offset, in the range 0 to 15, encoded in the "off4" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="offs__4"></span>
<p>For the "16-bit" variant: is the slice index offset, in the range 0 to 7, encoded in the "off3" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="offs__6"></span>
<p>For the "32-bit" variant: is the slice index offset, in the range 0 to 3, encoded in the "off2" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="offs__3"></span>
<p>For the "64-bit" variant: is the slice index offset, in the range 0 to 1, encoded in the "o1" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="offs__5"></span>
<p>For the "128-bit" variant: is the slice index offset, with implicit value 0.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;ZAn&gt;</td>
<td><span id="ZAn__2"></span>
<p>For the "16-bit" variant: is the name of the ZA tile ZA0-ZA1 to be accessed, encoded in the "ZAn" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="ZAn__3"></span>
<p>For the "32-bit" variant: is the name of the ZA tile ZA0-ZA3 to be accessed, encoded in the "ZAn" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="ZAn"></span>
<p>For the "64-bit" variant: is the name of the ZA tile ZA0-ZA7 to be accessed, encoded in the "ZAn" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="ZAn__4"></span>
<p>For the "128-bit" variant: is the name of the ZA tile ZA0-ZA15 to be accessed, encoded in the "ZAn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let dim : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let index : bits(32) = X{}(s); let slice : integer = (UInt(index) + offset) MOD dim; let operand : bits(VL) = ZAslice{}(n, esize, vertical, slice); var result : bits(VL) = Z{}(d); for e = 0 to dim-1 do let element : bits(esize) = operand\[e\*:esize\]; if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then result\[e\*:esize\] = element; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
