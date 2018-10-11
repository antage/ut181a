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
| 0      | 1    | Misc byte (see below) |
| 1      | 1    | Misc2 byte (0x00: off, 0x01: on) |
| 2      | 2    | Mode |
| 4      | 1    | Range |

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
