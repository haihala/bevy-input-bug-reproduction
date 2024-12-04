# Bevy Analog stick bug reproduction

I was tearing my hair out tracking down a bug in my input pipeline, only to
realize it was actually a bug in Bevy.

To try it out for yourself:

- Own a "Hori Fighting commander" or another similar device
  - Important part is a dpad that can be set to act as an analog stick
- Set the dpad to act as an analog stick
- Wiggle left to right as fast as you can, occasionally dipping to a down input
- Watch as it eventually gets stuck in the down position after you let go

Tapping down will reset the situation.

Will see if it this happens on Bevy 0.15 next
