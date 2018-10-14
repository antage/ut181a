# UT181A protocol

## Frame format

All data (sent and received) is packed in frames.

| offset | size | description |
|-------:|-----:|-------------|
| 0      | 2    | Magic signature (0xCDAB). |
| 2      | 2    | A number of bytes in payload and checksum (N + 2). |
| 4      | N    | Payload. |
| 4 + N  | 2    | Checksum (C). |

The checksum is 16-bit unsigned integer.

C = 2 + N + (sum of bytes in payload)

## Received data

Payload format for received data:

| offset | size | description |
|-------:|-----:|-------------|
| 0      | 1    | Kind of packet (see below). |
| 1      | M    | A packet. |

Kind of packet:

| value | description |
|------:|-------------|
| 0x01  | Reply code packet. |
| 0x02  | Measurement packet. |
| 0x03  | Save packet. |
| 0x04  | Record info packet. |
| 0x05  | Record data packet. |
| 0x72  | Reply data packet (saves count, records count, etc). |

## Reply code packet

It contains 2 bytes.

| value | description |
|------:|-------------|
| 0x4B4F | OK (success). |
| 0x5245 | ER (error). |

## Measurement packet

A measurement packet has 4 formats:

1. normal measurement,
1. relative measurement,
1. min/max measurement,
1. peak measurement.

### Common format

| offset | size | description |
|-------:|-----:|-------------|
| 0      | 1    | Misc byte (see below). |
| 1      | 1    | Misc2 byte (see below). |
| 2      | 2    | Mode word (see below). |
| 4      | 1    | Range byte (see below). |

### Misc byte

| bit(s) | description |
|-------:|-------------|
| 1      | Packet has aux1 value. |
| 2      | Packet has aux2 value. |
| 3      | Packet has bargraph value. |
| 4..6   | Packet format:
|        |   0: normal |
|        |   1: relative |
|        |   2: min/max |
|        |   4: peak |
| 7      | Hold (0: off, 1: on). |

### Misc2 byte

| bit(s) | description |
|-------:|-------------|
| 0      | Auto range |
| 1      | High voltage |
| 3      | Lead error |
| 4      | Comp mode |
| 5      | Record mode |

### Mode word

| value  | mode / submode |
|-------:|----------------|
| 0x1111 | VAC/normal |
| 0x1112 | VAC/normal relative |
| 0x1121 | VAC/Hz |
| 0x1131 | VAC/peak |
| 0x1141 | VAC/low pass |
| 0x1142 | VAC/low pass relative |
| 0x1151 | VAC/dBV |
| 0x1152 | VAC/dBV relative |
| 0x1161 | VAC/dBm |
| 0x1162 | VAC/dBm relative |
| 0x2111 | mVAC/normal |
| 0x2112 | mVAC/normal relative |
| 0x2121 | mVAC/Hz |
| 0x2131 | mVAC/peak |
| 0x2141 | mVAC/AC+DC |
| 0x2142 | mVAC/AC+DC relative |
| 0x3111 | VDC/normal |
| 0x3112 | VDC/normal relative |
| 0x3121 | VDC/AC+DC |
| 0x3122 | VDC/AC+DC relative |
| 0x3131 | VDC/peak |
| 0x4111 | mVDC/normal |
| 0x4112 | mVDC/normal relative |
| 0x4121 | mVDC/peak |
| 0x4211 | TempC/T1,T2 |
| 0x4212 | TempC/T1,T2 relative |
| 0x4221 | TempC/T2,T1 |
| 0x4222 | TempC/T2,T1 relative |
| 0x4231 | TempC/T1-T2 |
| 0x4241 | TempC/T2-T1 |
| 0x4311 | TempF/T1,T2 |
| 0x4312 | TempF/T1,T2 relative |
| 0x4321 | TempF/T2,T1 |
| 0x4322 | TempF/T2,T1 relative |
| 0x4331 | TempF/T1-T2 |
| 0x4341 | TempF/T2-T1 |
| 0x5111 | Resistance |
| 0x5112 | Resistance relative |
| 0x5211 | Beeper/Short |
| 0x5212 | Beeper/Open |
| 0x5311 | Admittance |
| 0x5312 | Admittance relative |
| 0x6111 | Diode/Normal |
| 0x6112 | Diode/Alarm |
| 0x6211 | Capacitance |
| 0x6212 | Capacitance realative |
| 0x7111 | Frequency |
| 0x7112 | Frequency relative |
| 0x7211 | Duty cycle |
| 0x7212 | Duty cycle relative |
| 0x7311 | Pulse width |
| 0x7312 | Pulse width relative |
| 0x8111 | uADC/normal |
| 0x8112 | uADC/normal relative |
| 0x8121 | uADC/AC+DC |
| 0x8122 | uADC/AC+DC relative |
| 0x8131 | uADC/peak |
| 0x8211 | uAAC/normal |
| 0x8212 | uAAC/normal relative |
| 0x8221 | uAAC/Hz |
| 0x8231 | uAAC/peak |
| 0x9111 | mADC/normal |
| 0x9112 | mADC/normal relative |
| 0x9121 | mADC/AC+DC |
| 0x9122 | mADC/AC+DC relative |
| 0x9131 | mADC/peak |
| 0x9211 | mAAC/normal |
| 0x9212 | mAAC/normal relative |
| 0x9221 | mAAC/Hz |
| 0x9231 | mAAC/peak |
| 0xA111 | ADC/normal |
| 0xA112 | ADC/normal relative |
| 0xA121 | ADC/AC+DC |
| 0xA122 | ADC/AC+DC relative |
| 0xA131 | ADC/peak |
| 0xA211 | AAC/normal |
| 0xA212 | AAC/normal relative |
| 0xA221 | AAC/Hz |
| 0xA231 | AAC/peak |

### Range byte

| value | description |
|------:|-------------|
| 0x00  | Auto range |
| 0x01  | 60 mV/6 V/600 uA/60 mA/600 Ohm/60 Hz/6 nF |
| 0x02  | 600 mV/60 V/6000 uA/600 mA/6 KOhm/600 Hz/60 nF |
| 0x03  | 600V/60 KOhm/6 KHz/600 nF |
| 0x04  | 1000 V/600 KOhm/60 KHz/6 uF |
| 0x05  | 6 MOhm/600 KHz/60 uF |
| 0x06  | 60 MOhm/6 MHz/600 uF |
| 0x07  | 60 MHz/6 mF |
| 0x08  | 60 mF |

### Precision byte

| bit(s) | description |
|-------:|-------------|
| 0      | Positive overload (0: no, 1: overload). |
| 1      | Negative overload (0: no, 1: overload). |
| 4..7   | A number of digits after decimal point. |

Positive and negative overloads aren't mutually exclusive and can be simultaneously.

### Normal measurement

| offset | size | description |
|-------:|-----:|-------------|
| 5      | 4    | Main value (float32). |
| 9      | 1    | Main value's precision byte (see above). |
| 10     | 8    | 0-terminated string consists a unit of measurement. |
| 18     | 4    | Aux1 value (float32). |
|        |      | It's optional (check bit 1 of misc byte). |
| 22     | 1    | Aux1's precision byte (see above). |
|        |      | It's optional (check bit 1 of misc byte). |
| 23     | 8    | 0-terminated string consists a unit of measurement. |
|        |      | It's optional (check bit 1 of misc byte). |
| 31     | 4    | Aux2 value (float32). |
|        |      | It's optional (check bit 2 of misc byte). |
| 35     | 1    | Aux2's precision byte (see above). |
|        |      | It's optional (check bit 2 of misc byte). |
| 36     | 8    | 0-terminated string consists a unit of measurement. |
|        |      | It's optional (check bit 2 of misc byte). |
| 44     | 4    | A bargraph value (float32). |
|        |      | It's optional (check bit 3 of misc byte).
| 48     | 8    | 0-terminated string consists a unit of measurement. |
|        |      | It's optional (check bit 3 of misc byte).

If the packet doesn't consist aux1 or aux2 values all offsets starting 18
should be shifted.

### Relative measurement

| offset | size | description |
|-------:|-----:|-------------|
| 5      | 4    | Relative value (float32). |
| 9      | 1    | Relative value's precision byte (see above). |
| 10     | 8    | 0-terminated string consists a unit of measurement. |
| 18     | 4    | Reference value (float32). |
| 22     | 1    | Reference value's precision byte (see above). |
| 23     | 8    | 0-terminated string consists a unit of measurement. |
| 31     | 4    | Absolute value (float32). |
| 35     | 1    | Absolute value's precision byte (see above). |
| 36     | 8    | 0-terminated string consists a unit of measurement. |

### Min/max measurement

| offset | size | description |
|-------:|-----:|-------------|
| 5      | 4    | Current value (float32). |
| 9      | 1    | Current value's precision byte (see above). |
| 10     | 4    | Max value (float32). |
| 14     | 1    | Max value's precision byte (see above). |
| 15     | 4    | A number (unsigned integer) of seconds since start of the measurement until a moment when the max value has been updated. |
| 19     | 4    | Average value (float32). |
| 23     | 1    | Average value's precision byte (see above). |
| 24     | 4    | A number (unsigned integer) of seconds since start of the measurement until a moment when the average value has been updated. |
| 28     | 4    | Min value (float32). |
| 32     | 1    | Min value's precision byte (see above). |
| 33     | 4    | A number (unsigned integer) of seconds since start of the measurement until a moment when the min value has been updated. |
| 37     | 8    | 0-terminated string consists a unit of measurement. |

### Peak measurement

| offset | size | description |
|-------:|-----:|-------------|
| 5      | 4    | Max value (float32). |
| 9      | 1    | Max value's precision byte (see above). |
| 10     | 8    | 0-terminated string consists a unit of measurement. |
| 18     | 4    | Min value (float32). |
| 22     | 1    | Min value's precision byte (see above). |
| 23     | 8    | 0-terminated string consists a unit of measurement. |

## Save packet

| offset | size | description |
|-------:|-----:|-------------|
| 0      | 4    | Date/time (see below). |
| 5      | N    | Measurement packet. |

### Date/time format

| bit(s) | description |
|-------:|-------------|
| 0..5   | Year - 2000 |
| 6..9   | Month |
| 10..14 | Day |
| 15..19 | Hour |
| 20..25 | Minute |
| 26..31 | Second |

## Record info packet

| offset | size | description |
|-------:|-----:|-------------|
| 0      | 11   | 0-terminated string, name of record. |
| 11     | 8    | 0-terminated string, unit of measurements. |
| 19     | 2    | sampling interval in seconds. |
| 21     | 4    | duration of record in seconds. |
| 25     | 4    | a number of samples. |
| 29     | 4    | max value (float32). |
| 33     | 1    | max value's precision byte. |
| 34     | 4    | average value (float32). |
| 38     | 1    | average value's precision byte. |
| 39     | 4    | min value (float32). |
| 43     | 1    | min value's precision byte. |
| 44     | 4    | record beginning date/time (see 'Date/time format'). |

## Record data packet

| offset  | size | description |
|--------:|-----:|-------------|
| 0       | 1    | a number of samples in the packet (N). |
| i*9 + 0 | 4    | sample value (float32). |
| i*9 + 4 | 1    | precision byte. |
| i*9 + 5 | 4    | sample data/time (see 'Date/time format'). |

`i` is 0..N-1.
