# ADC_UDP_LOGGER
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



