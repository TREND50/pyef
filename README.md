# Python Package for Reading the Eventfile


### OMH 11/02/2019
Prerequisite: 
- python3
- rust
- Python libraries:
setuptools
setuptools-rust

./setup.py build
sudo .:setup install


## Supported python version
This lib relies on [pyo3](https://github.com/PyO3/pyo3.git), which
supports both python 2 and 3. However in order to support different
python version, some parameters must be set when compiling this (pyef)
lib. So currently I decide to only support python3.

## Installation
0. The environment of [RUST](http://www.rust-lang.org) should be
installed beforehand. Read the installation guide of
[gp_daq](https://github.com/TREND50/gp_daq) as a reference.

1. To make everything tidy and clean, we create a new directory to contain
everything needed and no other things, for example:
```
$> export BUILD_ROOT='$HOME/build' #can be set as other path
$> mkdir $BUILD_ROOT
$> cd $BUILD_ROOT
```

2. Then get necessary libs:
```
$> git clone https://github.com/PyO3/pyo3.git
$> git clone git@github.com:TREND50/gp_daq.git
$> git clone git@github.com:astrojhgu/pyef.git
```

3. Install necessary python packages:
```
$> sudo pip install setuptools-rust
$> sudo pip install setuptools
```

4. change the directory
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

# ToDo:
Update README?
