# Starting with Ghidra

To start with Ghidra, lets look at it's layout. Your's may slightly vary.

![Ghidra](assets/ghidra-layout.png)

On the left is the Symbol Tree, it shows all symbols within the current program. You can search here to find symbols by name.

![Symbol Tree](assets/symbol-tree.png)

On the bottom left is the Data Type Manager, it contains all data types within the current program. You can again search to find data types by name. This is useful when creating structs.

![Data Type Manager](assets/data-man.png)

On the right is the Decompiler Window, it shows the currently selected function decompiled to C like pseudo code. It is a good way to see what a function is actually doing.

![Decompiler](assets/decompiler.png)

Lastly in the middle is the Disassembly. It is the disassembled data of the program. This contains functions, strings and more. A basic understanding of assembly is very useful for Ghidra as pseudo code may not be entirely accurate, this [resource](https://mariokartwii.com/armv8/) is a great beginner's guide. The program has an offset of 7100000000, so any offsets from here should be reduced by that value.

![Disassembly](assets/disassembly.png)
