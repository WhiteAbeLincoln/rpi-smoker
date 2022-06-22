# https://docs.circuitpython.org/projects/ads1x15/en/latest/

import time
import board
import busio
import adafruit_ads1x15.ads1115 as ADS
from adafruit_ads1x15.analog_in import AnalogIn

# create the I2C bus object
i2c = busio.I2C(board.SCL, board.SDA)

# create the ADC object using I2C bus
ads = ADS.ADS1115(i2c)

# we create an object for a single-ended input on channel 0
chan0 = AnalogIn(ads, ADS.P0)

print("{:>5}\t{:>5}".format('raw', 'v'))

while True:
    print("{:>5}\t{:>5.3f}".format(chan0.value, chan0.voltage))
    time.sleep(0.5)