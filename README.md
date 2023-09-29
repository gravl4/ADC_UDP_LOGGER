# ADC_UDP_LOGGER

www.vauag.com - open source project www.vauag.ru

Currenly limits: NOT SUPPORTED ARCHIVE 

View and log analog signals ADC from UPD stream 

linux lite utilite for vizualise ADC data. For example from CH32V307 or STM32 via Ethernet and easy UPD stream
![изображение](https://github.com/gravl4/ADC_UDP_LOGGER/assets/64896051/4770f3fd-72f7-4c74-9f45-7e22e56a61f9)

source code for RUST, visout external grafic libraries

format data sample:
0x55 0х33 [packet id - 2 bytes] [ch0 - 2 bytes] [ch1] .. [ch8] [digital 0][digital 1] 
[packet id - 2 bytes]  - sample number. from 0 to 65565 and overload to 0 in cycle 
[chx - 2 bytes] -  H bite, L bite - for confort view in terminal
contains ADC data 12bits xxxxHHHH LLLLLLLL 
[digital x] (one bite)- digital signals. Each bit is signal
Sample date length 22 bites

One  UDP paackets contain 10..50 data samples.  
Data samples frequency 10/100/1000Hz

Software settings as samples frequency, IP addres, port, signal colors and etc. in text file cnfg.txt
Example attached with bin. 
Begin config:
[Network]
IP=192.168.0.100
//IP=192.168.56.1
Port=2001
SendFreq=1000

[GUI]
FontFamily=helvetica
FontWeight=bold
FontLen=10
GRAPH_BCK_COLOR=0x1C1C1C
GRID_LINE_COLOR=0x00AFAFAF
TIME_VALS_COLOR=0x00AFAFAF
CNTRL_BCK_COLOR=0x00FFFFFF
CNTRL_HIGHLIGHT_COLOR=0x00FFFF0F
CNTRL_TXT_COLOR=0x00000000
CNTRL_LINE_COLOR= 0x00AFAFAF

// https://tools.seo-zona.ru/color.html?ysclid=l9dtgwzj19567639097 - цвета для настроек 
// do indexing for channel name
// don't enumerate parameters in channel section
// k- float, b,c, min, max- integer
[Ch_A1]
k=0.1
b=-2047
c=0
color=0x0BFF2F
name=111111
ADC_max=0x0FFF
min=-200
max=200
visible=1
....................


