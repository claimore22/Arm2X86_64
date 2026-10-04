## MOV

Move logical bitmask immediate to vector (unpredicated)

This instruction unconditionally broadcasts the logical bitmask immediate into each element of the destination vector. This instruction is unpredicated. The immediate is a 64-bit value consisting of a single run of ones or zeros repeating every 2, 4, 8, 16, 32 or 64 bits.

This is an alias of [DUPM](dupm_z_i.html). This means:

- The encodings in this description are named to match the encodings of [DUPM](dupm_z_i.html).
- The description of [DUPM](dupm_z_i.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

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
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="13" class="lr">imm13</td>
<td colspan="5" class="lr">Zd</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="mov_z_m__dupm_z_i_"></span>

MOV [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").[\<T\>](#T__34 "Is the size specifier, "), \#[\<const\>](#const "Is a 64, 32, 16 or 8-bit bitmask consisting of replicated 2, 4, 8, 16, 32 or 64 bit fields, each field containing a rotated run of non-zero bits, encoded in the "imm13" field.")

is equivalent to

[DUPM](dupm_z_i.html#dupm_z_i_) [\<Zd\>](dupm_z_i.html#Zd).[\<T\>](dupm_z_i.html#T__34), \#[\<const\>](dupm_z_i.html#const)

and is the preferred disassembly when <span class="pseudocode">SVEMoveMaskPreferred(imm13)</span>.

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
<td><span id="T__34"></span>
<p>Is the size specifier, encoded in “imm13”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">imm13</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0xxxxxx0xxxxx</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">0xxxxxx10xxxx</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">0xxxxxx110xxx</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">0xxxxxx1110xx</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">0xxxxxx11110x</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">0xxxxxx11111x</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">1xxxxxxxxxxxx</td>
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
<td>&lt;const&gt;</td>
<td><span id="const"></span>
<p>Is a 64, 32, 16 or 8-bit bitmask consisting of replicated 2, 4, 8, 16, 32 or 64 bit fields, each field containing a rotated run of non-zero bits, encoded in the "imm13" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [DUPM](dupm_z_i.html) gives the operational pseudocode for this instruction.

</div>
