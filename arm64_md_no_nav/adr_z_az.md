## ADR

Calculate vector address

This instruction optionally sign-extends or zero-extends the least significant 32 bits of each element from a vector of offsets or indices in the second source vector, and scales each index by 2, 4, or 8. The scaled indices are then added to a vector of base addresses from the first source vector, and the resulting addresses are placed in the destination vector. This instruction is unpredicated.

This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

It has encodings from 3 classes: [Packed offsets](#iclass_packed_offsets) , [Unpacked 32-bit signed offsets](#iclass_unpacked_32_bit_signed_offsets) and [Unpacked 32-bit unsigned offsets](#iclass_unpacked_32_bit_unsigned_offsets)

### <span id="iclass_packed_offsets"></span>Packed offsets<span style="font-size:smaller;">\
(FEAT_SVE) </span>

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
<td class="lr">1</td>
<td class="lr">sz</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zm</td>
<td class="l">1</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td colspan="2" class="lr">msz</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="adr_z_az_sd_same_scaled"></span>

ADR [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").[\<T\>](#T__11 "Is the size specifier, "), \[[\<Zn\>](#Zn__3 "Is the name of the base scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__11 "Is the size specifier, "), [\<Zm\>](#Zm__3 "Is the name of the offset scalable vector register, encoded in the "Zm" field.").[\<T\>](#T__11 "Is the size specifier, "){, [\<mod\>](#mod_LSL "Is the index extend and shift specifier, ") [\<amount\>](#amount__12 "Is the index shift amount, ")}\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32 \<\< UInt(sz); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd); let osize : integer{} = esize; let unsigned : boolean = TRUE; let mbytes : integer{} = 1 \<\< UInt(msz);

### <span id="iclass_unpacked_32_bit_signed_offsets"></span>Unpacked 32-bit signed offsets<span style="font-size:smaller;">\
(FEAT_SVE) </span>

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
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zm</td>
<td class="l">1</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td colspan="2" class="lr">msz</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="5"></td>
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

<span id="adr_z_az_d_s32_scaled"></span>

ADR [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").D, \[[\<Zn\>](#Zn__3 "Is the name of the base scalable vector register, encoded in the "Zn" field.").D, [\<Zm\>](#Zm__3 "Is the name of the offset scalable vector register, encoded in the "Zm" field.").D, SXTW{[\<amount\>](#amount__12 "Is the index shift amount, ")}\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 64; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd); let osize : integer{} = 32; let unsigned : boolean = FALSE; let mbytes : integer{} = 1 \<\< UInt(msz);

### <span id="iclass_unpacked_32_bit_unsigned_offsets"></span>Unpacked 32-bit unsigned offsets<span style="font-size:smaller;">\
(FEAT_SVE) </span>

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
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zm</td>
<td class="l">1</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td colspan="2" class="lr">msz</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="5"></td>
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

<span id="adr_z_az_d_u32_scaled"></span>

ADR [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").D, \[[\<Zn\>](#Zn__3 "Is the name of the base scalable vector register, encoded in the "Zn" field.").D, [\<Zm\>](#Zm__3 "Is the name of the offset scalable vector register, encoded in the "Zm" field.").D, UXTW{[\<amount\>](#amount__12 "Is the index shift amount, ")}\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 64; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd); let osize : integer{} = 32; let unsigned : boolean = TRUE; let mbytes : integer{} = 1 \<\< UInt(msz);

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
<td><span id="T__11"></span>
<p>Is the size specifier, encoded in “sz”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">sz</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">1</td>
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
<td><span id="Zn__3"></span>
<p>Is the name of the base scalable vector register, encoded in the "Zn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zm&gt;</td>
<td><span id="Zm__3"></span>
<p>Is the name of the offset scalable vector register, encoded in the "Zm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;mod&gt;</td>
<td><span id="mod_LSL"></span>
<p>Is the index extend and shift specifier, encoded in “msz”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">msz</th>
<th class="symbol">&lt;mod&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">[absent]</td>
</tr>
<tr>
<td class="bitfield">x1</td>
<td class="symbol">LSL</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">LSL</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;amount&gt;</td>
<td><span id="amount__12"></span>
<p>Is the index shift amount, encoded in “msz”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">msz</th>
<th class="symbol">&lt;amount&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">[absent]</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">#1</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">#2</td>
</tr>
<tr>
<td class="bitfield">11</td>
<td class="symbol">#3</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let base : bits(VL) = Z{}(n); let offs : bits(VL) = Z{}(m); var result : bits(VL); for e = 0 to elements-1 do let addr : bits(esize) = base\[e\*:esize\]; let offselt : bits(osize) = offs\[e\*:esize\]\[osize-1:0\]; let offset : integer = if unsigned then UInt(offselt) else SInt(offselt); result\[e\*:esize\] = addr + (offset \* mbytes); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
