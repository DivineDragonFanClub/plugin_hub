# Introduction
We receive a request one day - would it be possible to hide the UI during combat?
(screen shot of combat UI)

# Observations
First, we check how the game handles the UI in combat.

We have this UI in the map view:
(map view)

And this UI in combat view:
(combat view)

If we watch closely, we can see that the combat UI fades in and fades out during combat.

So we can conclude that the game must have some way of showing and hiding this UI.

# Investigations
This part can take some luck.

Things are named somewhat sanely, but it can still very much feel like a looking for a needle in a haystack.

We fire up Ghidra and start searching for whatever might come to mind.

`UI`
`show`
`combat`
`fade`
`HUD`

(screenshot showing 38 hud matches)

and a few names catch our eye

Combat.CombatWorld$$FadeInHUD
Combat.CombatWorld$$FadeOutHUD

At this point, we can write a few quick hooks to confirm if this is the function we're looking for.

```
#[skyline::hook(offset = 0x29368f0)] // update with the named hooking method
pub fn fade_in_hud(this: *const u8, method_info: OptionalMethod) {
    println!("fade in hude");
    call_original!(this, method_info);
}```

(Include the hooking process for plugin users)

And play a battle!

(show the logs)

Great, it does seem like this is what the game uses.

# Implementation
We take a gamble and check if just not returning here is sufficient (and safe!).

```
#[skyline::hook(offset = 0x29368f0)] // update with the named hooking method
pub fn fade_in_hud(this: *const u8, method_info: OptionalMethod) {
    println!("skip fading in hud");
    // call_original!(this, method_info);
}```

And it works!

# Adding the configuration
(configuration stuff)

