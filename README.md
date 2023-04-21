#### Modified Anycubic Kobra Neo V1.33 Firmware

I modified jokubasver's version of the kobra neo firmware and seems like got Lin_Advance working. Also I increased the acceleration. Can't say if it's safe to run, so run on own risk


## Features
- Increase probing accuracy by doing multiple probes per point
- Increased speed for the first Z-probe approach when double-probing
- Enabled quick home (X and Y homes at the same time)
- Set default mainboard fan speed to 5 instead of 255
- Enable M117 Gcode for setting messages to printer screen
- Enable M73 Gcode for setting progress bar on printer screen
- UI changes - black background, removed ugly yellow text color
- Add personal PID and E-Step values
- LIN_ADVANCE working (V1.5 Marlin!)
- Increased

## Flashing
Copy firmware.bin to your microSD card, insert the card with the printer off, turn printer on and wait until you get to the home screen. Afterwards, delete the firmware.bin file from your card.

## Before printing
Perform a PID autotune and calibrate your e-steps! This firmware contains values for my own printer, but even the stock firmware does not have great values. 

Even after doing all the hardware and mechanical touch-ups, I was not getting great prints. This was solved by calibrating E-steps, as they were quite off (instead of extruding 100mm of filament, my printer extruded 95mm before calibration), so make sure you do that as well.

Instructions can be found here:

PID Autotune: https://teachingtechyt.github.io/calibration.html#pid

E-Step calibration: https://teachingtechyt.github.io/calibration.html#esteps

If you are building the firmware yourself, you can include your personal PID and E-step values in the firmware's Configuration.h file: https://github.com/jokubasver/Kobra_Neo/commit/a33ebd921d4031b541fb395de72f8292334ff8a0

Having these values saved in the firmware itself could save a lot of headaches, as If the EEPROM for whatever reason was to clear, your values will not dissapear.


## Building
https://www.reddit.com/r/anycubic/comments/y2waxu/tutorial_how_to_build_anycubic_marlin_source_code/

## Based on
https://github.com/jokubasver/Kobra_Neo

https://github.com/sjorge/Kobra_Neo_Fw

https://github.com/jojos38/anycubic-kobra-improved-firmware

https://github.com/Auburn/Kobra_Go
