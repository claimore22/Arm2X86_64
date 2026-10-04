## GCSSS2

Guarded Control Stack switch stack 2

This instruction validates that the most recent entry of the Guarded Control Stack being switched to contains an In-progress GCS cap record, stores a Valid GCS cap record to the Guarded Control Stack that is being switched from, and sets Xt to the Guarded Control Stack pointer that is being switched from.

For more information, see GCSSS2, Guarded Control Stack Switch Stack 2.

This is an alias of [SYSL](sysl.html). This means:

- The encodings in this description are named to match the encodings of [SYSL](sysl.html).
- The description of [SYSL](sysl.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

### <span id="iclass_system"></span>System<span style="font-size:smaller;">\
(FEAT_GCS) </span>

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
<td class="l">1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="4"></td>
<td class="droppedname">L</td>
<td colspan="2"></td>
<td colspan="3" class="droppedname">op1</td>
<td colspan="4" class="droppedname">CRn</td>
<td colspan="4" class="droppedname">CRm</td>
<td colspan="3" class="droppedname">op2</td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="GCSSS2_SYSL_RC_systeminstrs"></span>

GCSSS2 [\<Xt\>](#XtOrXZR__4 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rt" field.")

is equivalent to

[SYSL](sysl.html#SYSL_RC_systeminstrs) [\<Xt\>](sysl.html#XtOrXZR__4), \#3, C7, C7, \#3

and is always the preferred disassembly.

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xt&gt;</td>
<td><span id="XtOrXZR__4"></span>
<p>Is the 64-bit name of the general-purpose destination register, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [SYSL](sysl.html) gives the operational pseudocode for this instruction.

</div>
