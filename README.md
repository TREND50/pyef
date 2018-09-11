# Python Package for Reading the Eventfile

## Supported python version
This lib relies on [pyo3](https://github.com/PyO3/pyo3.git), which
supports both python 2 and 3. However in order to support different
python version, some parameters must be set when compiling this (pyef)
lib. So currently I decide to only support python3.

## Installation

To make everything tidy and clean, we create a new directory to contain
everything needed and no other things, for example:
```
$> export BUILD_ROOT='$HOME/build'
$> mkdir $BUILD_ROOT
$> cd $BUILD_ROOT
```

Then get necessary libs:
```
$> git clone https://github.com/PyO3/pyo3.git
$> git clone git@github.com:TREND50/gp_daq.git
$> git clone git@github.com:astrojhgu/pyef.git
```

change the directory
```
$> cd pyef
```

Compile and install
```
python3 setup.py build
sudo python3 setup.py install
```

## Usage
Launch a ipython shell

```
import pyef
data=pyef.read_file('somefilename.bin')
```


Check file header values:
```
data.header.runnr
```
Here runnr can be replaced by other keys. Simply type ```data.header.```
and press <tab> twice, a list of possible keys will appear.

Check the length of event list:
```
len(data.event_list)
```

Check the event header:
```
h=data.event_list[0]
h.header.eventnr
```
eventnr can be replaced by other keys, use <tab> can list possible
keys.

Get ADC data:
```
data.event_list[0].local_station_list[0].adc_buffer
```
Currently for GRAND, the first ```[0]``` can be replaced by other ```[i]``` and
the second ```[0]``` is a fixed value (i.e., every event only
contains one local station.

