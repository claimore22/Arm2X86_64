## UBFX

Unsigned bitfield extract

This instruction copies a bitfield of \<width\> bits starting from bit position \<lsb\> in the source register to the least significant bits of the destination register, and sets destination bits above the bitfield to zero.

This is an alias of [UBFM](ubfm.html). This means:

- The encodings in this description are named to match the encodings of [UBFM](ubfm.html).
- The description of [UBFM](ubfm.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

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
<td class="lr">sf</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">N</td>
<td colspan="6" class="lr">immr</td>
<td colspan="6" class="lr">imms</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="6"></td>
<td></td>
<td colspan="6"></td>
<td colspan="6"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="UBFX_UBFM_32M_bitfield"></span> Applies when <span class="bitdiff"> (sf == 0 && N == 0)</span>

UBFX [\<Wd\>](#WdOrWZR "Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Wn\>](#WnOrWZR "Is the 32-bit name of the general-purpose source register, encoded in the "Rn" field."), \#[\<lsb\>](#lsb__2 "For the "32-bit" variant: is the bit number of the lsb of the source bitfield, in the range 0 to 31."), \#[\<width\>](#width "For the "32-bit" variant: is the width of the bitfield, in the range 1 to 32-<lsb>.")

is equivalent to

[UBFM](ubfm.html#UBFM_32M_bitfield) [\<Wd\>](ubfm.html#WdOrWZR), [\<Wn\>](ubfm.html#WnOrWZR), \#[\<lsb\>](ubfm.html#lsb__2), \#([\<lsb\>](ubfm.html#lsb__2)+[\<width\>](ubfm.html#width)-1)

and is the preferred disassembly when <span class="pseudocode">BFXPreferred(sf, opc\[1\], imms, immr)</span>.

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="UBFX_UBFM_64M_bitfield"></span> Applies when <span class="bitdiff"> (sf == 1 && N == 1)</span>

UBFX [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Xn\>](#XnOrXZR__11 "Is the 64-bit name of the general-purpose source register, encoded in the "Rn" field."), \#[\<lsb\>](#lsb__4 "For the "64-bit" variant: is the bit number of the lsb of the source bitfield, in the range 0 to 63."), \#[\<width\>](#width__2 "For the "64-bit" variant: is the width of the bitfield, in the range 1 to 64-<lsb>.")

is equivalent to

[UBFM](ubfm.html#UBFM_64M_bitfield) [\<Xd\>](ubfm.html#XdOrXZR__6), [\<Xn\>](ubfm.html#XnOrXZR__11), \#[\<lsb\>](ubfm.html#lsb__4), \#([\<lsb\>](ubfm.html#lsb__4)+[\<width\>](ubfm.html#width__2)-1)

and is the preferred disassembly when <span class="pseudocode">BFXPreferred(sf, opc\[1\], imms, immr)</span>.

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wd&gt;</td>
<td><span id="WdOrWZR"></span>
<p>Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Wn&gt;</td>
<td><span id="WnOrWZR"></span>
<p>Is the 32-bit name of the general-purpose source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;lsb&gt;</td>
<td><span id="lsb__2"></span>
<p>For the "32-bit" variant: is the bit number of the lsb of the source bitfield, in the range 0 to 31.</p></td>
</tr>
<tr>
<td></td>
<td><span id="lsb__4"></span>
<p>For the "64-bit" variant: is the bit number of the lsb of the source bitfield, in the range 0 to 63.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;width&gt;</td>
<td><span id="width"></span>
<p>For the "32-bit" variant: is the width of the bitfield, in the range 1 to 32-&lt;lsb&gt;.</p></td>
</tr>
<tr>
<td></td>
<td><span id="width__2"></span>
<p>For the "64-bit" variant: is the width of the bitfield, in the range 1 to 64-&lt;lsb&gt;.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xd&gt;</td>
<td><span id="XdOrXZR__6"></span>
<p>Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xn&gt;</td>
<td><span id="XnOrXZR__11"></span>
<p>Is the 64-bit name of the general-purpose source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [UBFM](ubfm.html) gives the operational pseudocode for this instruction.

</div>

<div class="alias_ps_section">

### Operational Information

The description of [UBFM](ubfm.html) gives the operational information for this instruction.

</div>
