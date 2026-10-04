## CNEG

Conditional negate

This instruction returns, in the destination register, the negated value of the source register if the condition is TRUE, and otherwise returns the value of the source register.

This is an alias of [CSNEG](csneg.html). This means:

- The encodings in this description are named to match the encodings of [CSNEG](csneg.html).
- The description of [CSNEG](csneg.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

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
<td class="lr">1</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rm</td>
<td colspan="4" class="lr">!= 111x</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td class="droppedname">op</td>
<td class="droppedname">S</td>
<td colspan="8"></td>
<td colspan="5"></td>
<td colspan="4" class="droppedname">cond</td>
<td></td>
<td class="droppedname">o2</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="CNEG_CSNEG_32_condsel"></span> Applies when <span class="bitdiff"> (sf == 0)</span>

CNEG [\<Wd\>](#WdOrWZR "Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Wn\>](#RmRn_option__3 "Is the 32-bit name of the general-purpose source register, encoded in the "Rn" and "Rm" fields."), [\<invcond\>](#cond_option__2 "Is one of the standard conditions, excluding AL and NV, encoded with its least significant bit inverted, and ")

is equivalent to

[CSNEG](csneg.html#CSNEG_32_condsel) [\<Wd\>](csneg.html#WdOrWZR), [\<Wn\>](csneg.html#WnOrWZR__3), [\<Wm\>](csneg.html#WmOrWZR__2), [\<cond\>](csneg.html#cond_option)

and is the preferred disassembly when <span class="pseudocode">Rn == Rm</span>.

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="CNEG_CSNEG_64_condsel"></span> Applies when <span class="bitdiff"> (sf == 1)</span>

CNEG [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Xn\>](#RmRn_option__4 "Is the 64-bit name of the general-purpose source register, encoded in the "Rn" and "Rm" fields."), [\<invcond\>](#cond_option__2 "Is one of the standard conditions, excluding AL and NV, encoded with its least significant bit inverted, and ")

is equivalent to

[CSNEG](csneg.html#CSNEG_64_condsel) [\<Xd\>](csneg.html#XdOrXZR__6), [\<Xn\>](csneg.html#XnOrXZR__12), [\<Xm\>](csneg.html#XmOrXZR__4), [\<cond\>](csneg.html#cond_option)

and is the preferred disassembly when <span class="pseudocode">Rn == Rm</span>.

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
<td><span id="RmRn_option__3"></span>
<p>Is the 32-bit name of the general-purpose source register, encoded in the "Rn" and "Rm" fields.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;invcond&gt;</td>
<td><span id="cond_option__2"></span>
<p>Is one of the standard conditions, excluding AL and NV, encoded with its least significant bit inverted, and encoded in “cond”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">cond</th>
<th class="symbol">&lt;invcond&gt;</th>
<th class="symbol">Description</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0000</td>
<td class="symbol">NE</td>
<td class="description"><p>Maps to &lt;cond&gt; EQ.</p></td>
</tr>
<tr>
<td class="bitfield">0001</td>
<td class="symbol">EQ</td>
<td class="description"><p>Maps to &lt;cond&gt; NE.</p></td>
</tr>
<tr>
<td class="bitfield">0010</td>
<td class="symbol">CC</td>
<td class="description"><p>Maps to &lt;cond&gt; CS.</p></td>
</tr>
<tr>
<td class="bitfield">0011</td>
<td class="symbol">CS</td>
<td class="description"><p>Maps to &lt;cond&gt; CC.</p></td>
</tr>
<tr>
<td class="bitfield">0100</td>
<td class="symbol">PL</td>
<td class="description"><p>Maps to &lt;cond&gt; MI.</p></td>
</tr>
<tr>
<td class="bitfield">0101</td>
<td class="symbol">MI</td>
<td class="description"><p>Maps to &lt;cond&gt; PL.</p></td>
</tr>
<tr>
<td class="bitfield">0110</td>
<td class="symbol">VC</td>
<td class="description"><p>Maps to &lt;cond&gt; VS.</p></td>
</tr>
<tr>
<td class="bitfield">0111</td>
<td class="symbol">VS</td>
<td class="description"><p>Maps to &lt;cond&gt; VC.</p></td>
</tr>
<tr>
<td class="bitfield">1000</td>
<td class="symbol">LS</td>
<td class="description"><p>Maps to &lt;cond&gt; HI.</p></td>
</tr>
<tr>
<td class="bitfield">1001</td>
<td class="symbol">HI</td>
<td class="description"><p>Maps to &lt;cond&gt; LS.</p></td>
</tr>
<tr>
<td class="bitfield">1010</td>
<td class="symbol">LT</td>
<td class="description"><p>Maps to &lt;cond&gt; GE.</p></td>
</tr>
<tr>
<td class="bitfield">1011</td>
<td class="symbol">GE</td>
<td class="description"><p>Maps to &lt;cond&gt; LT.</p></td>
</tr>
<tr>
<td class="bitfield">1100</td>
<td class="symbol">LE</td>
<td class="description"><p>Maps to &lt;cond&gt; GT.</p></td>
</tr>
<tr>
<td class="bitfield">1101</td>
<td class="symbol">GT</td>
<td class="description"><p>Maps to &lt;cond&gt; LE.</p></td>
</tr>
</tbody>
</table></td>
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
<td><span id="RmRn_option__4"></span>
<p>Is the 64-bit name of the general-purpose source register, encoded in the "Rn" and "Rm" fields.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [CSNEG](csneg.html) gives the operational pseudocode for this instruction.

</div>

<div class="alias_ps_section">

### Operational Information

The description of [CSNEG](csneg.html) gives the operational information for this instruction.

</div>
