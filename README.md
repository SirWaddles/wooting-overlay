# wooting-overlay

This is an osu! key overlay similar to [KeyOverlay](https://github.com/Blondazz/KeyOverlay) however built for an Analog keyboard in mind. It supports the Wooting 60HE, the Wooting Two HE and the Wooting UwU.

## Config

Included in the download is a `config.json` file which allows you to set the vk_codes for k1 and k2. They will need to be converted to decimal first (e.g. the Z key is `0x5A` so the value will be `90`)

It also allows you to set the sample size. This is how many samples are shown on the display (a higher number will slow the display, a lower number will be faster). I would recommend that this be integer divisible by 500 (the size of the window.) So values like 50, 100 and 250 are recommended. Artifacting may occur with other values. 

## OBS Capture

You should be able to capture this using the 'Game Capture' element in OBS, and tick 'Allow Transparency' to show it over the top of existing elements.

## Supported Platforms

Currently only Windows is supported, but other platforms may be added in future.