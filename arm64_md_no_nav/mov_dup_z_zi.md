## MOV (SIMD&FP scalar, unpredicated)

Move indexed element or SIMD&FP scalar to vector (unpredicated)

This instruction unconditionally broadcasts the SIMD&FP scalar into each element of the destination vector. This instruction is unpredicated.

This is an alias of [DUP (indexed)](dup_z_zi.html). This means:

- The encodings in this description are named to match the encodings of [DUP (indexed)](dup_z_zi.html).
- The description of [DUP (indexed)](dup_z_zi.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

### <span id="iclass_sve"></span>SVE<span style="font-size:smaller;">\
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
<td>0</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td class="r">1</td>
<td colspan="2" class="lr">imm2</td>
<td class="lr">1</td>
<td colspan="5" class="lr">tsz</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td>0</td>
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

<span id="mov_z_v__dup_z_zi_"></span>

MOV [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").[\<T\>](#T__33 "Is the size specifier, "), [\<V\>](#V__8 "Is a width specifier, ")[\<n\>](#n__6 "Is the number [0-31] of the source SIMD&FP register, encoded in the "Zn" field.")

is equivalent to

[DUP](dup_z_zi.html#dup_z_zi_) [\<Zd\>](dup_z_zi.html#Zd).[\<T\>](dup_z_zi.html#T__33), [\<Zn\>](dup_z_zi.html#Zn).[\<T\>](dup_z_zi.html#T__33)\[0\]

and is the preferred disassembly when <span class="pseudocode">BitCount(imm2 :: tsz) == 1</span>.

</div>

<div class="encoding">

#### Encoding

<span id="mov_z_zi__dup_z_zi_"></span>

MOV [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").[\<T\>](#T__33 "Is the size specifier, "), [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__33 "Is the size specifier, ")\[[\<imm\>](#imm__47 "Is the immediate index, in the range 0 to one less than the number of elements in 512 bits, encoded in "imm2:tsz".")\]

is equivalent to

[DUP](dup_z_zi.html#dup_z_zi_) [\<Zd\>](dup_z_zi.html#Zd).[\<T\>](dup_z_zi.html#T__33), [\<Zn\>](dup_z_zi.html#Zn).[\<T\>](dup_z_zi.html#T__33)\[[\<imm\>](dup_z_zi.html#imm__47)\]

and is the preferred disassembly when <span class="pseudocode">BitCount(imm2 :: tsz) \> 1</span>.

</div>

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
<td><span id="T__33"></span>
<p>Is the size specifier, encoded in “tsz”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">tsz</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00000</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">xxxx1</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">xxx10</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">xx100</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">x1000</td>
<td class="symbol">D</td>
</tr>
<tr>
<td class="bitfield">10000</td>
<td class="symbol">Q</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;V&gt;</td>
<td><span id="V__8"></span>
<p>Is a width specifier, encoded in “tsz”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">tsz</th>
<th class="symbol">&lt;V&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00000</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">xxxx1</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">xxx10</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">xx100</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">x1000</td>
<td class="symbol">D</td>
</tr>
<tr>
<td class="bitfield">10000</td>
<td class="symbol">Q</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;n&gt;</td>
<td><span id="n__6"></span>
<p>Is the number [0-31] of the source SIMD&amp;FP register, encoded in the "Zn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn&gt;</td>
<td><span id="Zn"></span>
<p>Is the name of the source scalable vector register, encoded in the "Zn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm__47"></span>
<p>Is the immediate index, in the range 0 to one less than the number of elements in 512 bits, encoded in "imm2:tsz".</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [DUP (indexed)](dup_z_zi.html) gives the operational pseudocode for this instruction.

</div>

<div class="alias_ps_section">

### Operational Information

The description of [DUP (indexed)](dup_z_zi.html) gives the operational information for this instruction.

</div>
