This guide is for beginners who may have some IPS experience, but otherwise, haven't modded games much before.

Some parts of this will be inaccurate for the sake of a simpler explanations.

# What can I do with Cobalt plugins? 
Almost anything! But what you can actually achieve depends entirely on your patience...

The superpowers that Cobalt grants you, in no particular order, are
1. A way for you to write hooks and modify the game's logic directly.
2. The ability to call any original function in the game.
3. Access to the Switch SD card to load whatever you want.

# What is a hook?
A "hook" lets us intercept a function call in the game and change the behavior of the game by writing our own custom code.

To hook, you can use one of the macros Cobalt provides.

You need to define which function you want to hook, by providing the name of the function or its address.

Now when you run the game, and this function is called, it will run your hook code instead.

When your function is called, you get access to inputs to the function, and can do what you'd like with them.

This macro will also provide you the ability to call the original function. Depending on what exactly you're hooking, this may be optional. 

# Example




 