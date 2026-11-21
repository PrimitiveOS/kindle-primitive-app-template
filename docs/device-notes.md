# Device notes

Device verification is pending. Exercise launch, one interaction, sleep, wake, Done, reopen, and removal from the
launcher and Store before a device release.

Under Primitive Launcher, also check the system layer (the launcher owns these while the app is open):

- Press the power button: the lock screen shows the launcher's sleep picture. Wake: the app's screen comes back as it
  was, without a restart (the log shows no second `Opened`).
- Swipe down from the top edge: the launcher's quick settings open over the app, and the app does not react to the
  swipe. Close the panel: the app's screen comes back without a full flash.
- Switch dark mode in quick settings over the app: the screen flashes once when the theme changes, and the app
  redraws in the new theme without a second flash.
- Tap Power in quick settings, then Cancel: the power menu (Cancel, Kindle UI, Restart, Power off) is drawn over the
  app, and Cancel returns to it.
