# Page 1

# AXP2101 Single Cell NVDC PMU with E-gauge 

## 1. Features

- 3.9V-5.5V Input Operating Range and Support single Cell Battery
- Battery fuel gauge: Egauge 3.0
- Support TWSI(Two Wire Serial Interface) and RSB(Reduced Serial Bus)
- 100mA-1A Linear charger, CV accuracy +/-0.5\%
- Single input to support USB input
- High battery discharge efficiency with 50 mohm battery discharge MOSFET up to 4A
- High integration includes all MOSFETS, current sensing and loop compensation
- Power off current <20uA (BATFET off, RTCLDO output on)
- 4 DCDC

DCDC1:1.5 3.4V, IMAX=2A;
DCDC2: 0.5 1.2V, 1.22 1.54V, IMAX=2A
DCDC3: 0.5 1.2V, 1.22 1.54V, 1.6 3.4V, IMAX=2A
DCDC4:0.5 1.2V, 1.22 1.84V,IMAX=1.5A,for DDR;

- 11 LDOS

RTCLDO1 2: 1.8V/2.5V/3V/3.3V, 30mA; Support
RTCLDO1 supplied by backup battery(button battery)
ALDO1 4: analog LDO,0.5 3.5V, 0.1V/step, IMAX=300mA,
ALDO3 AND ALDO4 are low noise LDO
BLDO1 2: analog LDO,0.5 3.5V, 0.1V/step, IMAX=300mA, high PSRR LDO
CPUSLDO: for CPUs, 0.5 1.4V, IMAX=30mA
DLDO1 2: analog LDO or power switch, 0.5 3.5V/ 0.5 1.4V, IMAX=300mA

- startup sequence and default voltage of DCDC/LDO setting
- Protection

Input Over-Voltage Protection
Battery Thermistor Sense Hot/Cold Charge Suspend
Programmable Safety Timer for Charger
Die Thermal Balance for Charger
Thermal Shutdown
DCDC Over-Voltage/Under-Voltage
protection

## 2. Applications

- SDV, Car DVR, IPC, smart doorbell, smart speaker


## 3. Description

AXP2101 is a highly integrated power management IC(PMIC) targeting at single cell Li-battery(Li-ion or Li-polymer) applications that require multi-channel power conversion outputs. It provides an easy and flexible power management solution for multi-core processors to meet the complex and accurate requirements of power control.
AXP2101 supports Linear charge. Besides, it supports 15 channel power outputs which include 4 channel DC-DC and 11 channel LDO . To ensure the security and stability of the system, AXP2101 provides multiple channels 14-bit ADC for voltage/temperature monitor and integrates protection circuits such as over-voltage protection(OVP), over-current protection(OCP) and over-temperature protection(OTP). Moreover, AXP2101 features a unique E-Gauge ${ }^{\text {TM }}$ (Fuel Gauge) system, making power gauge easy and exact.
AXP2101 supports TWSI and RSB for system to dynamically adjust output voltages, charge current and configurate interrupt condition.

Device Information

| Part Number | Package | Body Size |
| :-- | :-- | :-- |
| AXP2101 | QFN-40 | $5 \mathrm{~mm} * 5 \mathrm{~mm}$ |

## Simplified Application Diagram

![img-0.jpeg](img-0.jpeg)

# Page 2

# 4. Revision History

|  Revision | Date | Description  |
| --- | --- | --- |
|  V 0.1 | April 28,2019 | Initial version  |
|  |   |   |

# Page 3

# Contents 

1. Features ..... 1
2. Applications ..... 1
3. Description ..... 1
4. Revision History ..... 2
5. Pin Configuration and Functions ..... 4
6. Specifications ..... 6
6.1 Absolute Maximum Ratings ${ }^{(1)}$ ..... 6
6.2 ESD Ratings ..... 6
6.3 Recommended Operating Conditions ..... 6
6.4 Thermal Information ..... 6
7. Detail Description ..... 7
7.1 Overview ..... 7
7.3 Serial Interface Communication ..... 9
7.4 Power Path ..... 9
7.5 Power On/Off and reset ..... 9
7.5.1 Power on reset(POR) ..... 9
7.5.2 Power up from BAT ..... 9
7.5.3 Power up from VBUS ..... 9
7.5.4 System power on/off management ..... 10
7.6 Multi-Power Outputs ..... 13
7.7 Charger ..... 14
7.7.1 Characteristics ..... 14
7.7.2 Charging condition ..... 14
7.7.3 Charging process ..... 14
7.7.4 Charging protection ..... 15
7.7.5 Charging indication ..... 17
7.8 BATFET ..... 18
7.9 RBFET ..... 18
7.10 ADC ..... 18
7.11 E-Gauge ..... 18
7.12 IRQ /BACKUP ..... 19
7.12.1 IRQ ..... 19
7.12.2 BACKUP ..... 19
8. Register ..... 19
8.1 Register List ..... 19
8.2 Register Description ..... 21
10 Package and Ordering Information ..... 48
10.1 Package Information ..... 48
10.2 Marking information ..... 48
10.3 Carrier ..... 48
10.4 Storage ..... 49
10.5 Baking ..... 50
9. Reflow Profile ..... 50

# Page 4

![img-1.jpeg](img-1.jpeg)

# **5. Pin Configuration and Functions**

![img-2.jpeg](img-2.jpeg)

|  Pin Description |  |  | Description  |
| --- | --- | --- | --- |
|  NO. | Pin Name | Type |   |
|  1 | CHGLED | AO | Charge status output to indicate various charger operation.  |
|  2 | VREF | P | Internal reference voltage  |
|  3 | GND | AI | Analog ground for interrupt analog and digital circuits.  |
|  4 | FB3 | P | DCDC3 feedback pin  |
|  5 | LX3 | P | Inductor pin for DCDC3  |
|  6 | VIN3 | P | DCDC3 input source  |
|  7 | VIN4 | P | DCDC4 input source  |
|  8 | LX4 | P | Inductor pin for DCDC4  |
|  9 | FB4 | P | DCDC4 feedback pin and Switch input source  |
|  10 | CPULDOS | P | Output pin of CPULDOS  |
|  11 | DLDO2/DC4S |  | Output pin of DLDO2, and can be configured as the Output pin of DC4SW  |
|   | W | DO |   |
|  12 | BLDO1 | P | Output pin of BLDO1  |
|  13 | BLDOIN | P | BLDO input source  |
|  14 | BLDO2 | P | Output pin of BLDO2  |

# Page 5

AXP2101 April,28,2019

|  15 | ALDO4 | $P$ | Output pin of ALDO4  |
| --- | --- | --- | --- |
|  16 | ALDO3 | $P$ | Output pin of ALDO3  |
|  17 | ALDOIN | $P$ | ALDO input source  |
|  18 | ALDO1 | $P$ | Output pin of ALDO1  |
|  19 | ALDO2 | $P$ | Output pin of ALDO2  |
|  20 | DLDO1/DC1S
W | $P$ | Output pin of DLDO1, and can be configured as the Output pin of DC1SW  |
|  21 | FB1 | $P$ | DCDC1 feedback pin  |
|  22 | LX1 | $P$ | Inductor pin for DCDC1  |
|  23 | VIN1 | $P$ | DCDC1 input source  |
|  24 | VIN2 | $P$ | DCDC2 input source  |
|  25 | LX2 | $P$ | Inductor pin for DCDC2  |
|  26 | FB2 | AI | DCDC2 feedback pin  |
|  27 | VBackup | $P$ | input pin of backup battery  |
|  28 | VRTC | $P$ | RTC power output  |
|  29 | PWROK | DIO | Power good indication output  |
|  30 | PWRON | DIO | Power On-Off key input, Internal 100k pull up to VINT  |
|  31 | TS | AI | Temperature qualification voltage input.
Connect a negative temperature coefficient thermistor from TS to GND.
A current source is injected to TS pin and convert TS voltage to a digital code. Charging suspends when TS pin is out of range. Besides, TS can be connected to external input signal.  |
|  32 | GPIO1 | DIO | Output pin of GPIO1 and can be configed to RTCLDO or FBS of DCDC5  |
|  33 | BAT | $P$ | Battery connection point.
The internal BATFET is connected between BAT and SYS.
Connect a 1uF capacitor closely to the BAT pin.  |
|  34 | VSYS | $P$ | System connection point.
The internal BATFET is connected between BAT and SYS. When the battery falls below the minimum system voltage, switch-mode converter keeps SYS above the minimum system voltage. Connect two 22 uF capacitors closely to the SYS pin.  |
|  35 | SW | $P$ | Inductor pin for buck  |
|  36 | VMID | $P$ | VMID Power output  |
|  37 | VBUS | $P$ | Vbus input  |
|  38 | IRQ | DIO | Open-drain interrupt Output.
Connect the IRQ to a logic rail via a $4.7 \mathrm{k} \Omega$ resistor. The IRQ pin sends a low level signal to host to report charger device status and fault.  |
|  39 | SCK | DI | Data pin for serial interface, need a $2.2 \mathrm{~K} \Omega$ Pull High.  |
|  40 | SDA | DIO | Clock pin for serial interface, need a $2.2 \mathrm{~K} \Omega$ Pull High.  |
|  EP | EP | GND | Exposed Pad, need to be connected to system ground  |

# Page 6

# 6. Specifications

### 6.1 Absolute Maximum Ratings ${ }^{(1)}$

Over operating free-air temperature range(unless otherwise noted)

|  SYMBOL | DESCRIPTION | MIN | MAX | UNIT  |
| --- | --- | --- | --- | --- |
|  VBUS | Voltage range(with respect to GND) | -0.3 | 12 | V  |
|  Others pin (exp vbus,pgnd,
gnd) |  | -0.3 | 7 | V  |
|   |  | -0.3 | 7 | V  |
|  PGND to GND |  | -0.3 | 0.3 | V  |
|  Ta | Operating Temperature Range | -40 | 85 | ${ }^{\circ} \mathrm{C}$  |
|  $T_{J}$ | Junction Temperature Range | -40 | 125 | ${ }^{\circ} \mathrm{C}$  |
|  Ts | Storage Temperature Range | -65 | 150 | ${ }^{\circ} \mathrm{C}$  |
|  $T_{\text {LEAD }}$ | Maximum Soldering Temperature (at leads.
10sec) |  | 30 | ${ }^{\circ} \mathrm{C}$  |

(1)Stresses beyond those listed under absolute maximum ratings may cause permanent damage to the device. These are stress ratings only. Functional operation of the device at these or any other conditions beyond those indicated under recommended operating conditions is not implied. Exposure to absolute maximum rated conditions for extended periods may affect device reliability.

### 6.2 ESD Ratings

|   |  | VALUE | UNIT  |
| --- | --- | --- | --- |
|  $\mathrm{V}_{\text {ESD }}$ | Human body model(HBM) ${ }^{(1)}$ | $\pm 4000$ | V  |
|   | Charged device model(CDM) ${ }^{(2)}$ | $\pm 750$ | V  |

(1) Reference:ESDA/JEDEC JS-001-2014. JEDEC document JEP155 states that 500-V HBM allows safe manufacturing with a standard ESD control process. (2) Reference:ESDA/JEDEC JS-002-2014. JEDEC document JEP157 states that 250-V CDM allows safe manufacturing with a standard ESD control process.

### 6.3 Recommended Operating Conditions

|  SYMBOL | DESCRIPTION | MIN | MAX | UNIT  |
| --- | --- | --- | --- | --- |
|  $\mathrm{V}_{\mathrm{IN}}$ | Input voltage(VBUS) | 3.9 | 5.5 | V  |
|  $\mathrm{I}_{\mathrm{IN}}$ | Input current(VBUS) |  | 2 | A  |
|  $\mathrm{I}_{\text {sys }}$ | Output current |  | 2 | A  |
|  $\mathrm{V}_{\text {BAT }}$ | Battery voltage |  | 4.4 | V  |
|  $\mathrm{I}_{\text {BAT }}$ | charging current |  | 1 | A  |

### 6.4 Thermal Information

|  Thermal Metric ${ }^{(1)}$ |  | VALUE | UNIT  |
| --- | --- | --- | --- |
|  $\theta_{\text {JA }}$ | Junction-to-ambient thermal resistance | 30 | ${ }^{\circ} \mathrm{C} / \mathrm{W}$  |
|  $\theta_{\text {JB }}$ | Junction-to-board thermal resistance | 10.8 |   |
|  $\theta_{\text {JC }}$ | Junction-to-case(top) thermal resistance | 22.8 |   |

(1)Thermal metrics are calculated refer to JEDEC document JESD51.

# Page 7

# 7. Detail Description 

### 7.1 Overview

AXP2101 is a highly integrated power management IC(PMIC) targeting at single cell Li-battery(Li-ion or Li-polymer) applications that require multi-channel power conversion outputs. It provides an easy and flexible power management solution for multi-core processors to meet the complex and accurate requirements of power control. AXP2101 supports 100mA-1A Linear charge. Besides, it supports 15 channel power outputs which include 4 channel DC-DC and 11 channel LDO . To ensure the security and stability of the system, AXP2101 provides multiple channels 16-bit ADC for voltage/temperature monitor and integrates protection circuits such as over-voltage protection(OVP), over-current protection(OCP) and over-temperature protection(OTP). Moreover, AXP2101 features a unique E-Gauge ${ }^{\text {TM }}$ (Fuel Gauge) system, making power gauge easy and exact.
AXP2101 supports TWSI and RSB for system to dynamically adjust output voltages, charge current and configurate interrupt condition.

AXP2101 is available in $5 \mathrm{~mm} \times 5 \mathrm{~mm}$ 40-pin QFN package.

# Page 8

# 7.2 Function Block Diagram 

![img-3.jpeg](img-3.jpeg)

# Page 9

# 7.3 Serial Interface Communication 

AXP2101 supports TWSI protocol and performs as a TWSI slave device with default address $0 \times 680 \times 69$. When AXP2101 powers on, SCK/SDA pin of TWSI will be pulled up to IO Power and then Host can adjust and monitor AXP2101 with rich feedback information.
Besides, AXP2101 supports RSB for Allwinner platform with address 0x01D1 or 0x0273 by customer.
Note: "Host" here refers to system processor.

### 7.4 Power Path

VBUS as the charger input, connecting to VSYS pin through a Linear charger, provides power to system and charges battery through BATFET. Charge current can be adjusted automatically according to the feedback current which is detected with an internal resistor. When system current(I $_{\text {sys }}$ ) changes, the detected current will change, and then the current change signal will feed back to charge loop to adjust the charge current to the setting value.

When battery voltage is above $\mathrm{V}_{\text {sys, }}$ BATFET is turned on and PMU enters supplement mode. When in supplement mode, if the discharge current is lower than 1A, PMU controls the voltage $\left(V_{D S}\right)$ between system and battery and keeps $\mathrm{V}_{\mathrm{DS}}$ at 30 mV to avoid entering and exiting supplement mode repeatedly. As discharge current increases, PMU adjusts BATFET to be fully on and $\mathrm{V}_{\mathrm{DS}}$ increases linearly. If an adapter is not inserted, system current is provided only by battery. At this time, BATFET is at fully on state.

### 7.5 Power On/Off and reset

### 7.5.1 Power on reset(POR)

AXP2101 is powered from the higher voltage between VBUS and BAT. When VBUS voltage $\left(V_{\text {vbus }}\right)$ is higher than $\mathrm{V}_{\text {vbus_uvloz }}$ or BAT voltage $\left(V_{B A T}\right)$ is higher than $\mathrm{V}_{\text {BAT_uvloz, }}$, the sleep comparator, battery depletion comparator and BATFET driver are active. All registers are reset to the default value. TWSI communication is active and Host can communicate with PMU.

### 7.5.2 Power up from BAT

If only battery is present and $\mathrm{V}_{\text {BAT }}$ is higher than depletion threshold( $\mathrm{V}_{\mathrm{BAT} \text { _DPL2 }}$ ), BATFET, connecting battery to system, is off by default and need to be turned on by pressing the PWRON key or inserting an adapter.

### 7.5.3 Power up from VBUS

When VBUS is inserted, PMU detects the input voltage to start up the reference voltage and the bias circuit. When $\mathrm{V}_{\text {vbus }}$ is higher than $\mathrm{V}_{\text {vbus_uvloz, }}$, the VBUS insertion IRQ is sent and the register bit reg49H[7] is set to 1 to indicate VBUS is inserted. Then PMU detects the input source whether it is good or not. If Vbus is good, the RBFET is open and Vsys is working.

# Page 10

# 7.5.3.1 Good source condition 

PMU needs to check the current capability of the input source. Only when the input source meets the following requirements can it start the buck converter.
a. VBUS voltage lower than $\mathrm{V}_{\text {ACOV }}$
b. VBUS voltage higher than $\mathrm{V}_{\text {VbUSMIN }}$ when pulling $\mathrm{I}_{\text {BADBUS }}$ (typical 30mA )

Once the input source meets the requirements above,the register bit reg00H[5](VBUS_GD) is set to 1 to indicate the input source is good.

### 7.5.3.3 Set input voltage limit $\left(V_{\text {INDPM }}\right)$

AXP2101 supports wide range of input voltage( $3.9 \mathrm{~V} \sim 5.5 \mathrm{~V}$ ). $\mathrm{V}_{\text {INDPM }}$ can be set through reg15H[3:0]. The range of $\mathrm{V}_{\text {INDPM }}$ is from 3.88 V to 5.08 V and the step is 80 mV .

When VBUS voltage reaches $\mathrm{V}_{\text {INDPM }}$, the charge current will decrease automatically until the current is zero. If $\mathrm{I}_{\text {SYS }}$ is over the input power supply capability, $\mathrm{V}_{\text {SYS }}$ will drop. If $\mathrm{V}_{\mathrm{BAT}}$ is above $\mathrm{V}_{\text {SYS }}$, PMU will enter the supplement mode.

### 7.5.4 System power on/off management

PMU has power off and power on status. When at off state, all voltage outputs are turned off except RTCLDO . At this time, the total power consumption is typically 25 uA .

### 7.5.4.1.Power on-off Key (POK)

EN/PWRON pin can be configured as PWRON pin or EN pin by customization. The default is PWRON pin. The Power on-off Key (POK) can be connected between PWRON pin and GND of AXP2101. AXP2101 can automatically identify the four status(Long-press ,Short-press ,Negative edge, Positive edge) and then correspond respectively.

### 7.5.4.2.Power on

1.When EN/PWRON pin is configured as PWRON pin, power on sources include:
(1).POK. AXP2101 can be powered on by pressing and holding POK for a period of time that longer than "ONLEVEL".
(2).VBUS low go high. The function can be configured by customization.
(3).VBAT low go high. The function can be configured by customization.
(4).IRQ Low level. IRQ pin is low level for more than 16 ms , AXP2101 will be powered on.The function can be configured by customization
(4).Battery is charged to normal ( Vbat>3.3V and is charging).The function can be configured by customization
2.When EN/PWRON pin is used as EN pin, AXP2101 can be powered on by EN pin from low go high(0.6V).

After power on, DC-DC and LDO will be soft booted in preset timing sequence. When IRQ low level power on, AXP2101 can be configured for fast power on by REG2B, and the DCDC/LDOS start sequence can be configured by REG28"REG2B.

# Page 11

# 7.5.4.3.Power Off 

1.When EN/PWRON pin is configured as PWRON pin, power off sources include:
(1).POK. AXP2101 can be powered off by pressing and holding POK for a period of time that longer than "OFFLEVEL". The function can be configured by REG22H[1] and REG22H[3:2] decides whether the PMU auto turns on or not when it shuts down after OFFLEVEL POK.
(2). Write " 1 " to REG10H[0] .
(3).VSYSGOOD high go low. When VSYS<VOFF or VBUS>7V, AXP2101 will be powered off. The default of VOFF is 2.6 V which can be configured by REG24H[2:0].
(4).The output voltage of DCDC is $15 \%$ lower than the setting value. The function can be configured by REG23H[4:0].
(5).The output voltage of DCDC is much larger than their setting. The function can be configured by REG23H[5].
(6).Die temperature is over the warning level2( $145^{\circ} \mathrm{C}$ ). The function can be configured by REG22H[2].

### 7.5.4.4.Sleep and wakeup

When the running system needs to enter Sleep mode, Maybe one or several power outputs should be disabled or changed to other voltage. Wakeup can be initiated by the following sources:
1.Software wakeup (REG26H[1] is set to 1)
2.IRQ pin wakeup(REG 26H[4]=1 and IRQ pin is low level for more than 16ms)

These sources will make the all PMU power outputs resume to the default voltage or the setting voltage, which is configured by REG26H[2], and all shutdown powers will resume by the startup sequence.

See the control process under sleep and wakeup modes as below:

# Page 12

![img-4.jpeg](img-4.jpeg)

Write "1" to REG26H[0] to start the Wakeup, and PMU will record the configuration of REG80H, REG90H,REG91H

Write REG80H, REG90H, REG91H, Disable corresponding power or modify the voltage

Sleep and wait to be Wakeup

All power resume to the default or setting voltage

# 7.5.4.5.Reset 

The PMU has system reset and power on reset.

- System reset

System reset means the registers will be reset when PMU is powered on. When at system reset state, all voltage outputs are turned off except RTCLDO and VREF. There are three ways of system reset.
(1).PWROK drive low.

The PWROK pin can be used as the reset signal of application system. During AXP2101 startup, PWROK outputs low level, which will be pulled up to startup the system after output voltage reaches the regulated value.

When application system works normally, If the PWROK pin is driven low by external key or other reasons, the PMU will be restarted. The function can be configured by REG10H[3].
(2).Write "1" to REG10H[1] to restart the PMU.
(3).Watchdog timeout . The function can be configured by REG18[0] and REG19[5:4]

- Power on reset

Power on reset means the registers will be reset when PMU is powered up. When at power on reset state, all voltage outputs are turned off including RTCLDO and VREF.

# Page 13

# 7.6 Multi-Power Outputs 

The following table has listed the multi-power outputs and their functions of AXP2101.

| Output Path | Type | Default Voltage | Startup <br> Sequence | Application Suggestion | Load <br> Capacity(Max) |
| :-- | :-- | :-- | :-- | :-- | :-- |
| DCDC1 | BUCK | 3.3 V | 3 | IO/USB | 2000 mA |
| DCDC2 | BUCK | 0.9 V | 3 | CPU | 2000 mA |
| DCDC3 | BUCK | 0.9 V | 2 | VSYS | 2000 mA |
| DCDC4 | BUCK | 1.1 V | 1 | DDR | 1500 mA |
| ALDO1 | LDO | 1.8 V | 3 | N/A | 300 mA |
| ALDO2 | LDO | 2.8 V | OFF | N/A | 300 mA |
| ALDO3 | LDO | 3.3 V | 3 | N/A | 300 mA |
| ALDO4 | LDO | 2.9 V | OFF | N/A | 300 mA |
| BLDO1 | LDO | 1.8 V | OFF | N/A | 300 mA |
| BLDO2 | LDO | 2.8 V | OFF | N/A | 300 mA |
| DLDO1 | LDO | 3.3 V | OFF | N/A | 300 mA |
| DLDO2 | LDO | 1.2 V | OFF | N/A | 300 mA |
| VCPUS | LDO | 0.9 V | 1 | CPUs/Reference of DDR | 30 mA |
| RTC-LDO1 | LDO | 1.8 V | Always on | RTC | 30 mA |
| RTC-LDO2 | LDO | OFF | OFF | N/A | 30 mA |

AXP2101 includes 4 synchronous step-down DCDCs, 11 LDOs and one switch. The work frequency of DC-DC 1/4 is 3 MHz and DCDC2/3 is 1.5 MHz . External small inductors and capacitors can be connected. In addition, 4-ch DCDCs can be set in fixed PWM mode or auto mode (automatically switchable according to the load). See register REG81H.

DCDC2/3 has DVM enable option. In DVM mode, when there is a change in the output voltage, DCDC will change to the new targeted value step by step. It supports two kinds of DVM slope:1step/15.625us and 1step/31.250us. The slope can be chosen by REG80H[5].

AXP2101 can configure the default voltage, the startup sequence and other control of all power output.
Startup sequence:The startup sequence has eight levels from 0 to 7 . When the sequence is 0 , it means the output is booted at the first step. When the sequence code is 1 , it means the output is booted at the second step. When the sequence is 7 , it means the output is not booted.
Default voltage setting: The default voltage of each channel can be set to each step within the output range.

# Page 14

# 7.7 Charger 

### 7.7.1 Characteristics

- Range of input voltage:3.9V 5.5V, PWM charger, supports single cell Li-battery
- Pre-charge current settable(I PRE-CHG, reg61[3:0]), default:125mA, range: 0mA 200mA,step:25mA
- Fast charge current settable(I CHG, reg62[4:0]), default:1024mA, range: 0mA 200mA,step:64mA, $200 \sim 1000 \mathrm{~mA}$, step: 100 mA ,
- Target charge voltage settable( $\mathrm{V}_{\text {REG }}$, reg64[2:0]), default:4.2V, range: $4.0 \mathrm{v} / 4.1 \mathrm{v} / 4.2 \mathrm{v} / 4.35 \mathrm{v} / 4.4 \mathrm{v} / 4.6 \mathrm{v}$
- Accuracy of target voltage: $\pm 0.5 \%$ (testing ambient temperature: $25^{\circ} \mathrm{C}$, target voltage:4.2V)


### 7.7.2 Charging condition

- VBUS is present and available, $\mathrm{V}_{\text {VbUS }}>\mathrm{V}_{\text {BAT }}+\mathrm{V}_{\text {SLEEPZ }}$
- Input source detection finishes(reg00H[5]=1)
- Charging is enabled(reg18H[1]=1)
- Die temperature is lower than $\mathrm{T}_{\text {SHUT }}$
- When TS pin is used to detect battery temperature, battery temperature is within the chargeable range
- $\mathrm{V}_{\text {BAT }}$ is lower than $\mathrm{V}_{\text {BAT_OVP }}$
- No charger safety timer fault


### 7.7.3 Charging process

When PMU meets all charging conditions, it can complete the whole charging process without the participation of Host. The charging status can be known from the register bits reg01H[2:0]. The default values of charging parameters are shown as following. Host can modify registers to optimize the values through TWSI.

Table 7-1

| Parameter | Default value |
| :--: | :--: |
| Charging voltage | 4.208 V |
| Charging current | 1.024 A |
| Pre-charging current | 125 mA |
| Termination current | 125 mA |
| Temperature profile | Cold/hot |
| Safety timer | 12 hours |

### 7.7.3.1. Pre-charge

When $\mathrm{V}_{\text {BAT }}$ is lower than $\mathrm{V}_{\text {BATLOWV }}(3 \mathrm{~V})$, the charger is under pre-charge mode where charging current is limited to a value of $I_{\text {PRE-CHG }}$. Safety time is set through reg67H[1:0] and its default value is 50 minutes. If pre-charge process times out, PMU will stop charging and send a corresponding IRQ to Host. The function of safety timer can be disabled through reg67H[2].

### 7.7.3.2. Constant current charge

Once $\mathrm{V}_{\text {BAT }}$ is higher than $\mathrm{V}_{\text {BATLOWV }}$ and lower than $\mathrm{V}_{\text {REG }}$, the charger is under constant current charge mode. It will charge with constant current $\mathrm{I}_{\mathrm{CHG}}$.

# Page 15

# 7.7.3.3.Constant voltage charge 

When $\mathrm{V}_{\text {BAT }}$ reaches target voltage $\left(\mathrm{V}_{\text {REG }}\right)$, the charger enters constant voltage charge mode. In this stage, the charger keeps the output voltage constant and step down charging current gradually, in order to fully charge battery.

When $\mathrm{V}_{\text {BAT }}$ is above $\mathrm{V}_{\text {RECHG }}$ and the charging current reduces under termination current( $\mathrm{I}_{\text {TERM }}$ ), AXP2101 reports charger done, stops charging(charger enable bit is still 1) and turns off BATFET. Meanwhile, IRQ is sent to Host.

When AXP2101 is in regulation of input current, input voltage or temperature, the function of charging termination configured through reg63[4] is temporarily disabled and the speed of safety timer slows down. Whether to set safety timer during DPM or thermal regulation depends on reg67H[7].

### 7.7.3.4.Re-charge

After charge done, if $\mathrm{V}_{\text {BAT }}$ falls below $\mathrm{V}_{\text {RECHG }}$, PMU will automatically enable charger without reinserting adapter. No matter whether $\mathrm{V}_{\text {BAT }}$ is above $\mathrm{V}_{\text {RECHG }}$ or not, the charger is enabled when an adapter is inserted.

### 7.7.3.5.Battery detection

As long as an AC adapter is present and usable, battery detection will be enabled to detect whether battery is connected. Battery detection function is enabled by default and can be disabled through reg68H[0]. If the function is disabled, PMU considers that battery is always present. The detection result is saved in reg00H[3]

### 7.7.4 Charging protection

### 7.7.4.1. charger safety timer

Once starting pre-charge mode, PMU will enable timer1. If PMU can not enter constant current charge mode from pre-charge within 50min(set through reg67H[1:0]), PMU will enter battery safe mode and send IRQ to indicate the battery may be damaged.

When the charger enters into constant current charge mode, PMU will enable timer2. If PMU can not finish the whole charge cycle within 12 hours(set through reg67H[5:4]), PMU will enter battery safe mode and send IRQ to indicate the battery may be damaged.

Timing speed of timer1 or timer2 is relevant with actual charge current. The smaller the actual charge current, the slower timing speed is.

### 7.7.4.2. Battery safe mode

In battery safe mode, the charger always charges with 10 mA current. PMU can quit battery safe mode with one of the following methods:

- $\mathrm{V}_{\text {BAT }}>\mathrm{V}_{\text {RECHG }}$
- Adapter removal
- Charger enable bit(reg18H[1]) is reset to 1
- Safety timer1 enable bit or safety timer2 enable bit is reset to 1

# Page 16

# 7.7.4.3. PMU die temperature protection 

AXP2101 has built-in temperature protection function through ADC to monitor internal temperature.

Under charging mode, the temperature point of thermal regulation can be set through reg65H[1:0]. When die temperature rises up to the setting point, the charging current will be decreased to decrease heat. When thermal regulation works, actual charge current is lower than the setting value and thermal regulation status(reg00H[1]) is set to 1 . If die temperature rises up to $\mathrm{T}_{\text {SHUT }}\left(145^{\circ} \mathrm{C}\right)$, IRQ is sent,PMU is poweroff. When die temperature falls below hysteretic threshold $\left(120^{\circ} \mathrm{C}\right)$, PMU is not poweron automatically.

### 7.7.4.4. Battery temperature protection

AXP2101 can monitor battery temperature, when TS pin is used to detect battery temperature and parallel with charger(reg50H[4]=0). The battery temperature sensitive resistor is connected between TS pin and GND. The suggestion resistance should be 10 Kohm at $25^{\circ} \mathrm{C}$ ambient temperature. Through TS pin, PMU outputs constant current which can set through reg50H[1:0] to adapt different resistance. When the resistance is 10Kohm, the current should be set to 50 uA . The enable bit of TS current source is configured through reg50H[3:2]. When current passes through the temperature sensitive resistor, PMU gets a detected voltage and calculates its value through ADC circuit. Take for example, TH11-3H103F temperature sensitive resistor of Mitsubishi Company. Using 50uA current source, the relationship among temperature, equivalent resistance, detected voltage and ADC data is as following.

Table 7-2

| Temperature | equivalent resistance | detected voltage | ADC DATA |
| :--: | :--: | :--: | :--: |
| $-20^{\circ} \mathrm{C}$ | 63.00Kohm | 3.150 V | 189 Ch |
| $-15^{\circ} \mathrm{C}$ | 50.15 Hohm | 2.508 V | 1398h |
| $-10^{\circ} \mathrm{C}$ | 40.26Kohm | 2.013 V | FBAh |
| $-5^{\circ} \mathrm{C}$ | 32.55Kohm | 1.628 V | CB8h |
| $0^{\circ} \mathrm{C}$ | 26.49Kohm | 1.325 V | A5Ah |
| $5^{\circ} \mathrm{C}$ | 21.68Kohm | 1.084 V | 878h |
| $10^{\circ} \mathrm{C}$ | 17.78Kohm | 0.889 V | 6F2h |
| $15^{\circ} \mathrm{C}$ | 14.63Kohm | 0.732 V | 5B8h |
| $20^{\circ} \mathrm{C}$ | 12.07Kohm | 0.604 V | 4B8h |
| $25^{\circ} \mathrm{C}$ | 10.00Kohm | 0.500 V | 3E8h |
| $30^{\circ} \mathrm{C}$ | 8.320Kohm | 0.416 V | 340h |
| $35^{\circ} \mathrm{C}$ | 6.954Kohm | 0.348 V | 2B8h |
| $40^{\circ} \mathrm{C}$ | 5.839Kohm | 0.292 V | 248h |
| $45^{\circ} \mathrm{C}$ | 4.924Kohm | 0.246 V | 1ECh |
| $50^{\circ} \mathrm{C}$ | 4.171Kohm | 0.209 V | 1A2h |
| $55^{\circ} \mathrm{C}$ | 3.549Kohm | 0.177 V | 162h |
| $60^{\circ} \mathrm{C}$ | 3.032Kohm | 0.152 V | 130h |

# Page 17

During battery charging process, if TS pin voltage is lower than VHTF-CHG or higher than VLTF-CHG ( VHTF-CHG and VLTF-CHG can be set through reg55H and reg54H. The default value of VLTF-CHG is set around $0^{\circ} \mathrm{C}$ and VHTF-CHG around $45^{\circ} \mathrm{C}$ ), which indicates battery temperature is too high or too low, then the charger is paused and IRQ is sent to notify Host. When battery temperature is back to the normal range, the charger will recovery automatically.

During battery discharging mode, if TS pin voltage is lower than VHTF-WORK or higher than VLTF-WORK( VHTF-WORK and VLTF-WORK can be set through reg57H and reg56H. The default value of VLTF-WORK is set around $-10^{\circ} \mathrm{C}$ and VHTF-WORK around $55^{\circ} \mathrm{C}$ ), which indicates battery temperature is too high or too low, then the boost is paused and IRQ is sent to notify Host. When battery temperature is back to the normal range, the boost will recovery automatically.

High temperature protection threshold hysteresis for VHTF-CHG and VHTF-WORK can be set through reg53H. Low temperature protection threshold hysteresis for VLTF-CHG and VLTF-WORK can be set through reg52H. The range of temperature detection can be expanded by adding more resistors.

Some battery may have no temperature sensitive resistor. Under this situation, TS pin can be pulled down to GND with a 10Kohm resistor externally or set as external input of ADC through register.

Use TS pin current source and obtain TS pin data according to the following table:

# 7.7.5 Charging indication 

CHGLED pin uses open-drain/push-pull output method. It is internally pulled up to LDO. Its output drive capability is above 10 mA . Detail function control is shown as the following table.

Table 7-4

| REG69H[2:1]= 00 <br> (Type A CHGLED) <br> Open Drain | Hi-Z | No charging(conditions are not met or battery charged) |
| :--: | :--: | :--: |
|  | 25\% 1Hz pull low/Hi-Z jump | Charger internal abnormal alarm(including timer out、 die temperature over temperature、battery temperature out of charging range) |
|  | 25\% 4Hz pull low/Hi-Z jump | Input source or battery over voltage |
|  | Pull low | Charging |
| REG69H[2:1]= 01 <br> (Type B CHGLED) <br> Open Drain | Hi-Z | No VBUS, and power supply by battery |
|  | 25\% 1Hz pull low/Hi-Z jump | Charging |
|  | 25\% 4Hz pull low/Hi-Z jump | Alarm, including input source or battery over voltage, battery temperature out of charging range, timer out,die temperature over temperature |
|  | Pull low | No battery or charge finished, and power supply by VBUS |
| REG90H[2:0]=10 <br> Cfg chgled | The output status is controlled by REG69H[5:4] |  |

Note: LED is on when CHGLED is low.

# Page 18

# 7.8 BATFET 

BATFET connects system and battery. The on-resistance is low to 50mohm(point to point).

### 7.9 RBFET

RBFET connects VMID and VBUS. The on-resistance is low to 100mohm(point to point). It supports input and output current limit function. In charger mode, the input current limit value of RBFET is set through reg16H[2:0].

### 7.10 ADC

AXP2101 has a low speed 14Bit SAR ADC for measuring BAT voltage, Vbus voltage, Vsys voltage, TS voltage and die temperature.

Table 7-5

| No. | Channel function | 000 H | 001 H | 002 H | $\ldots$ | FFFH |
| :--: | :-- | :--: | :--: | :--: | :--: | :--: |
| 0 | BAT voltage | 0 mV | 1 mV | 2 mV | $\ldots$ | 8.192 V |
| 1 | Vbus voltage | 0 mV | 1 mV | 2 mV | $\ldots$ | 8.192 V |
| 2 | Vsys voltage | 0 mV | 1 mV | 2 mV | $\ldots$ | 8.192 V |
| 3 | TS voltage | 0 mV | 0.5 mV | 1 mV | $\ldots$ | 4.096 V |
| 4 | die temperature | 0 mV | 0.1 mV | 2 mV | $\ldots$ | 0.8192 V |

Note: ADC data is 14 bits. In order to get the complete data, TWSI must read the high 6 bits firstly and then the low 8 bits.

### 7.11 E-Gauge

The Fuel Gauge comprises of 3 modules: Rdc calculation module; OCV (Open Circuit Voltage) and Coulomb counter module; and calibration module. The Fuel Gauge system is able to export information about battery capacity percentage (regA4H), Battery Voltage (reg34H, reg35H). The Fuel Gauge can be enabled or disabled through reg18H[3]. The Battery low warning level can be set in reg1AH, and IRQ will be sent out to alert the platform when the battery capacity percentage is lower than the warning level set in reg1AH.

Once a default battery is selected for a particular design, it is highly recommended to calibrate the battery to achieve better Fuel Gauge accuracy. Once the calibration data are available, user can write the calibration information to battery parameter (REGA1) on each boot. Additionally, the Fuel Gauge system is capable to learn the battery characteristic on each full charge cycle. Information such as battery maximum capacity and Rdc will be updated automatically over time.

# Page 19

# 7.12 IRQ /BACKUP 

### 7.12.1 IRQ

AXP2101 has an IRQ pin which is used to indicate whether there interrupt events occur.
PMU Interrupt Controller monitors the trigger events such as over voltage, over current, PWRON pin signal, over temperature and so on. When the events occur and their IRQ enabled bits are set to 1 (Refer to registers reg40H/41H/42H), corresponding IRQ status will be set to 1 (Refer to registers reg48H/49H/AAH), and IRQ pin will be pulled down. When Host detects triggered IRQ signal, Host will scan through the IRQ Status registers and respond accordingly. Meanwhile, Host will reset the IRQ status by writing " 1 " to status bit.

### 7.12.2 BACKUP

AXP2101 has an backup pin which is used to connect backup battery. It is the source of RTCLDO1 when pmu only has backup battery.

When PMU is poweron, the backup battery also cancan be charged by configuring reg18H[2]. The charger is working in linear mode with 100 uA charge current and the termination voltage can be configured by reg6AH in range from 2.6 V to $3.3 \mathrm{~V}($ default 2.9 V$)$.

The backup pin also can be configure for the RTCLDO2 by customization.

## 8. Register

### 8.1 Register List

| Address | Description | R/W |
| :-- | :-- | :-- |
| 0X00 | PMU status1 | R |
| 0X01 | PMU status2 | R |
| 0X03 | PMU CHIP ID | R |
| 0X04-0X08 | DATA_BUFFER | RW |
| 0X10 | PMU common config | RW |
| 0X12 | BATFET control | RW |
| 0X13 | Die temperature control | RW |
| 0X14 | Minimum system voltage control | RW |
| 0X15 | Input voltage limit control | RW |
| 0X16 | Input current limit control | RW |
| 0X17 | Reset the fulegauge | RW |
| 0X18 | Charger, fuleguage, watchdog on/off control | RW |
| 0X19 | Wathdog control | RW |
| 0X1A | Low Battery warning threshold setting | RW |
| 0X20 | PWRON status | R |
| 0X21 | PWROFF status | R |
| 0X22 | PWROFF_EN | RW |
| 0X23 | PWROFF of DCDC OVP/UVP control | RW |

# Page 20

| Address | Description | R/W |
| :--: | :--: | :--: |
| 0X24 | Vsys voltage for PWROFF threshold setting | RW |
| 0X25 | PWROK setting and PWROFF sequence control | RW |
| 0X26 | Sleep and wakeup control | Rw |
| 0X27 | IRQLEVEL/OFFLEVEL/ONLEVEL setting | RW |
| 0X28 | Fast pwron setting | Rw |
| 0X29 | Fast pwron setting | RW |
| 0X2A | Fast pwron setting | RW |
| 0X2B | Fast pwron setting and control | RW |
| 0X30-0X33 | ADC Channel enable control | RW |
| 0X34-0X3F | ADC data | RW |
| 0X40-0X42 | IRQ Enable | RW |
| 0X48-0X4A | IRQ Status | RW |
| 0X50 | TS pin CTRL \& GPADC mode CTRL | RW |
| 0X52 | TS/GPADC_HYSL2H setting | RW |
| 0X53 | TS/GPAC_HYSH2L setting | RW |
| 0X54 | VLTF_CHG setting | RW |
| 0X55 | VHTF_CHG setting | RW |
| 0X56 | VLTF_WORK setting | Rw |
| 0X57 | VHTF_WORK setting | Rw |
| 0X58 | JIETA standard Enable control | Rw |
| 0x59-0X5B | JIETA standard setting | Rw |
| 0X61 | Iprechg charger setting | RW |
| 0X62 | ICC charger setting | RW |
| 0X63 | Item charger setting and control | RW |
| 0X64 | CV charger voltage setting | RW |
| 0X65 | Thermal regulation threshold setting | RW |
| 0X67 | Charger timeout setting and control | RW |
| 0X68 | Battery detection control | RW |
| 0X69 | CHGLED setting and control | RW |
| 0X6A | Button battery charge termination voltage setting | RW |
| 0X80 | DCDCS ON/OFF and DVM control | RW |
| 0X81 | DCDCS force PWM control | RW |
| 0X82-0X86 | DCDCs voltage setting | RW |
| 0X90-0X91 | LDOS ON/OFF control | RW |
| 0X92-0X9A | LDOS voltage setting | RW |
| 0XA1 | Battery parameter | RW |
| 0XA2 | Fule guage control | RW |
| 0XA4 | Battery percentage data | R |

# Page 21

# 8.2 Register Description 

| Reg_Name | Addr | Type | Default | Reset Type | Description |
| :--: | :--: | :--: | :--: | :--: | :--: |
| comm_stat0 | 0x00 |  |  |  |  |
| reserved | 7:6 | R0 | 0 | / |  |
| vbus_good | 5 | R0 | 0 | POR | VBUS good indication <br> $0_{1}$ not good <br> $1_{1}$ good |
| batfet_stat | 4 | R0 | 0 | POR | BATFET state <br> 0 : close <br> 1: open |
| bat_prst_stat | 3 | R0 | 0 | POR | Battery present state <br> 0 : absent <br> 1: present |
| bat_active_mode | 2 | R0 | 0 | POR | Battery in Active Mode <br> 0 : in Normal <br> 1: in Active Mode |
| therm_regu_stat | 1 | R0 | 0 | POR | Thermal regulation status <br> 0 : normal <br> 1: in thermal regulation |
| ilim_stat | 0 | R0 | 0 | POR | Current Limit state <br> 0 : not in current limit state <br> 1: in current limit state |
| comm_stat1 | 0x01 |  |  |  |  |
| reserved | 7 | R0 | 0 | / |  |
| bat_curr_dir | $6: 5$ | R0 | 0 | POR | Batery Current Direction <br> 00: Standby <br> 01: charge <br> 10: discharge <br> 11: reserved |
| sys_stat | 4 | R0 | 0 | POR | System status indication <br> 0 : System is power off. <br> 1: System is power on. |
| vindpm_stat | 3 | R0 | 0 | POR | VINDPM status <br> 0 : not in VINDPM <br> 1: VINDPM |

# Page 22

| chg_stat | 2:0 | RO | 0 | POR | charging status <br> 000: tri_charge <br> 001: pre_charge <br> 010: constant charge (CC) <br> 011: constant voltage (CV) <br> 100: charge done <br> 101: not charging <br> 11X: reserved |
| :--: | :--: | :--: | :--: | :--: | :--: |
| chip_id | 0x03 |  |  |  |  |
| chip_id_h | 7:6 | RO | 0 h | POR |  |
| chip_version | $5: 4$ | RO | 0 h | POR | 00: A version <br> 01: B version |
| chip_id_1 | $3: 0$ | RO | 0 h | POR | \{chip_id_h, chip_id_1\} <br> 01_0111: axp2101 |
| data_buff0 | 0x04 |  |  |  |  |
| data_buff0 | 7:0 | RW | 00h | POR | data buffer |
| data_buff1 | 0x05 |  |  |  |  |
| data_buff1 | 7:0 | RW | 00h | POR | data buffer |
| data_buff2 | 0x06 |  |  |  |  |
| data_buff2 | 7:0 | RW | 00h | POR | data buffer |
| data_buff3 | 0x07 |  |  |  |  |
| data_buff3 | 7:0 | RW | 00h | POR | data buffer |
| comm_cfg | 0x10 |  |  |  |  |
| reserved | 7:6 | RW | 0b | 1 |  |
| dchg_off_en | 5 | RW | $1 b$ | POR | Internal off-discharge enable for DCDC \& <br> LDO \& SWITCH <br> 0 : disable <br> 1: enable |
| reserved | 4 | RW | $1 b$ | 1 |  |
| pwrok_restart_e <br> n | 3 | RW | $0 b$ | POR | PWROK PIN pull low to Restart the System <br> 0 : disable <br> 1: enable |
| pon_shut_en | 2 | RW | $0 b$ | POR | PWRON 16s to shut the PMIC enable <br> 0 : disable <br> 1: enable |
| soft_sys_restar <br> t | 1 | RWAC | $0 b$ | POR | Restart the SoC System, POWOFF/POWON and reset the related regsiters <br> 0 : normal <br> 1: reset |
| soft_pwroff | 0 | RWAC | $0 b$ | POR | Soft PWROFF <br> 0 : Normal <br> 1: PWROFF Config |
| batfet_ctr1 | $0 \times 12$ |  |  |  |  |

# Page 23

AXP2101
April, 28, 2019

| reserved | $7: 4$ | RO | 0 | / |  |
| :--: | :--: | :--: | :--: | :--: | :--: |
| batfet_pwroff_e <br> n | 3 | RW | EFUSE | POR | BATFET enable when POWEROFF and Battery only <br> 0 : disable <br> 1: enable |
| reserved | 2 | RO | 0 | / |  |
| batfet_ocp_en | 1 | RW | EFUSE | POR | BATFET Close when OCP ( 6 A) for 100us <br> 0 : disable <br> 1: enable |
| reserved | 0 | RO | 0 | / |  |
| die_temp_cfg | 0x13 |  |  |  |  |
| reserved | $7: 3$ | RO | 0 | / |  |
| die_otp_thld | $2: 1$ | RW | 01b | POR | DIE Over Temperature Protection Level1 <br> Config <br> 00: 115deg <br> 01: 125 deg <br> 10: 135 deg <br> 11: reserved |
| die_temp_det | 0 | RW | $1 b$ | POR | DIE Temperature Detect Enable <br> 0 : disable <br> 1: enable |
| vsys_min | 0x14 |  |  |  |  |
| reserved | 7 | RO | 0 | / |  |
| In_vsys_dpm | $6: 4$ | RW | 110b | POR | Linear Charger Vsys voltage dpm <br> $4,1+\mathrm{N} \equiv 0,1 \mathrm{~V}$ <br> 000: 4, 1V <br> 001: 4, 2V <br> 010: 4, 3V <br> 011: 4, 4V <br> 100: 4, 5V <br> 101: 4, 6V <br> 110: 4, 7V <br> 111: 4, 8V |
| reserved | $3: 0$ | RO | 0 | / |  |
| vimdpm_cfg | 0x15 |  |  |  |  |
| reserved | $7: 4$ | RO |  | / |  |
| vindpm_cfg | $3: 0$ | RW | 0110b | POR | VINDPM config: <br> $3,88+\mathrm{N} \equiv 0,08 \mathrm{~V}$ <br> 0000: 3, 88 V <br> 0001: 3, 96V <br> 0010: 4, 04V <br> 0011: 4, 12V <br> 0100: 4, 20V <br> 0101: 4, 28V |

# Page 24

|  |  |  |  |  | $\begin{aligned} & 0110: 4.36 \mathrm{~V} \\ & 0111: 4.44 \mathrm{~V} \\ & 1000: 4.52 \mathrm{~V} \\ & 1001: 4.60 \mathrm{~V} \\ & 1010: 4.68 \mathrm{~V} \\ & 1011: 4.76 \mathrm{~V} \\ & 1100: 4.84 \mathrm{~V} \\ & 1101: 4.92 \mathrm{~V} \\ & 1110: 5.00 \mathrm{~V} \\ & 1111: 5.08 \mathrm{~V} \end{aligned}$ |
| :--: | :--: | :--: | :--: | :--: | :--: |
| iin_1im | 0x16 |  |  |  |  |
| reserved | $7: 3$ | RO | 0 | / |  |
| iin_1im | $2: 0$ | RW | 001b | POR | Input current 1imit <br> 000: 100 mA <br> 001: 500 mA <br> 010: 900 mA <br> 011: 1000mA <br> 100: 1500mA <br> 101: 2000mA <br> 110-111: reserved |
| reset_cfg | 0x17 |  |  |  |  |
| reserved | $7: 4$ | RO | 0 | / |  |
| reset_guage | 3 | RWAC | 0b | POR | reset the gauge <br> 0: normal <br> 1: reset |
| reset_lgc_gauge | 2 | RW | 0b | POR | reset the gauge besides registers <br> 0: normal <br> 1: reset |
| reserved | $1: 0$ | RO | 0 | / |  |
| module_en | 0x18 |  |  |  |  |
| reserved | $7: 4$ | RO | 0 | / |  |
| gauge_en | 3 | RW | 1b | POR | Gauge Module enable <br> 0: disable <br> 1: enalbe |
| btn_chg_en | 2 | RW | 0b | System Reset | Button Battery charge enable <br> 0: disable <br> 1: enable |
| chg_en | 1 | RW | 1b | System Reset | Cell Battery charge enable <br> 0: disable <br> 1: enable |
| watchdog_en | 0 | RW | 0b | System Reset | Watchdog Module enable <br> 0: disable <br> 1: enalbe |

# Page 25

AXP2101
April, 28, 2019

| watchdog_cfg | 0x19 |  |  |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: |
| reserved | 7:6 | RO | 0 | / |  |
| wd_rst_cfg | $5: 4$ | RW | 0 b | POR | Watchdog Reset Config <br> 00: IRQ only <br> 01: IRQ and System Reset <br> 10: IRQ, System Reset and Pull down PWROK <br> 1s <br> 11: IRQ, System Reset, DCDC/LDO PWROFF \& <br> PWRON |
| watchdog_clr | 3 | RWAC | 0 b | POR | watchdog clear signal <br> 0: normal <br> 1: clear |
| watchdog_cfg | $2: 0$ | RW | 110 b | POR | TWSI watchdog timer config <br> 000: 1s <br> 001: 2s <br> 010: 4s <br> 011: 8s <br> 100: 16s <br> 101: 32s <br> 110: 64s <br> 111: 128s |
| gauge_thld | 0x1A |  |  |  |  |
| warn_thld | $7: 4$ | RW | 1010 b | POR | low battery warning threshold $5-20 \%, 1 \%$ per step <br> 0000: $5 \%$ <br> 0001: $6 \%$ <br> $\qquad$ <br> 1111: $20 \%$ |
| shutdown_thld | $3: 0$ | RW | 0001 b | POR | low battery shutdown threshold $0-15 \%, 1 \%$ per step <br> 0000: $0 \%$ <br> 0001: $1 \%$ <br> $\qquad$ <br> 1111: $15 \%$ |
| pwron_stat | 0x20 |  |  |  |  |
| reserved | 7:6 | RO | 0 | / |  |
| en_pwron_stat | 5 | RO | 0 b | System Reset | POWERON always high when EN Mode as POWERON Source <br> 0: no <br> 1: yes |
| bat_pwron_stat | 4 | RO | 0 b | System Reset | Battery Insert and Good as POWERON Source <br> 0: no <br> 1: yes |
| chg_pwron_stat | 3 | RO | 0 b | System Reset | Battery Voltage > 3.3V when Charged as |

# Page 26

AXP2101
April, 28, 2019

|  |  |  |  |  | Source <br> 0: no <br> 1: yes |
| :--: | :--: | :--: | :--: | :--: | :--: |
| vbus_pwron_stat | 2 | R0 | 0b | System Reset | Vbus Insert and Good as POWERON Source <br> 0: no <br> 1: yes |
| irq_pwron_stat | 1 | R0 | 0b | System Reset | IRQ PIN Pull-down as POWERON Source <br> 0: no <br> 1: yes |
| btn_pwron_stat | 0 | R0 | 0b | System Reset | POWERON low for onlevel when POWERON Mode <br> as POWERON Source <br> 0: no <br> 1: yes |
| pwroff_stat | $0 \times 21$ |  |  |  |  |
| dot_pwroff_stat | 7 | R0 | 0b | POR | Die Over Temperature as POWEROFF Source <br> 0: no <br> 1: yes |
| dcov_pwroff_sta <br> t | 6 | R0 | 0b | POR | DCDC Over Voltage as POWEROFF Source <br> 0: no <br> 1: yes |
| dcuv_pwroff_sta <br> t | 5 | R0 | 0b | POR | DCDC Under Voltage as POWEROFF Source <br> 0: no <br> 1: yes |
| vbov_pwroff_sta <br> t | 4 | R0 | 0b | POR | VBUS Over Voltage as POWEROFF Source <br> 0: no <br> 1: yes |
| vsuv_pwroff_sta <br> t | 3 | R0 | 0b | POR | Vsys Under Voltage as POWEROFF Source <br> 0: no <br> 1: yes |
| en_pwroff_stat | 2 | R0 | 0b | POR | POWERON always low when EN Mode as <br> POWEROFF Source <br> 0: no <br> 1: yes |
| sw_pwroff_stat | 1 | R0 | 0b | POR | Software config as POWEROFF Source <br> 0: no <br> 1: yes |
| btn_pwroff_stat | 0 | R0 | 0b | POR | POWERON Pull down for offlevel when <br> POWERON Mode as POWEROFF Source <br> 0: no <br> 1: yes |
| pwroff_en | $0 \times 22$ |  |  |  |  |
| reserved | 7:3 | R0 | 0 | $/$ |  |
| dot_pwroff_en | 2 | RW | 1b | POR | DIE Over-Temperature (LEVEL2) as POWEROFF <br> Source enable |

# Page 27

AXP2101
April 28, 2019

|  |  |  |  |  | 0 : disable <br> 1: enable |
| :--: | :--: | :--: | :--: | :--: | :--: |
| btn_pwroff_en | 1 | RW | EFUSE | POR | PwRON > OFFLEVEL as POWEROFF Source enable <br> 0 : disable <br> 1: enable |
| btn_pwroff_mode | 0 | RW | EFUSE | POR | Function Select when btn_pwroff_en=1 and button power-off occur <br> 0 : Power-off <br> 1: Restart |
| dcdc_pwroff_ en | $0 \times 23$ |  |  |  |  |
| reserved | 7:6 | RO | 0 | / |  |
| dcdc_ovp_en | 5 | RW | $1 b$ | POR | DCDC 120\%(130\%) high voltage turn off PMIC function <br> 0: disable <br> 1: enable |
| dcdc5_uvp_en | 4 | RW | $1 b$ | POR | DCDC5 85\% low voltage turn off PMIC function <br> 0: disable <br> 1: enable |
| dcdc4_uvp_en | 3 | RW | $1 b$ | POR | DCDC4 85\% low voltage turn off PMIC function <br> 0: disable <br> 1: enable |
| dcdc3_uvp_en | 2 | RW | $1 b$ | POR | DCDC3 85\% low voltage turn off PMIC function <br> 0: disable <br> 1: enable |
| dcdc2_uvp_en | 1 | RW | $1 b$ | POR | DCDC2 85\% low voltage turn off PMIC function <br> 0: disable <br> 1: enable |
| dcdc1_uvp_en | 0 | RW | $1 b$ | POR | DCDC1 85\% low voltage turn off PMIC function <br> 0: disable <br> 1: enable |
| voff_thld | $0 \times 24$ |  |  |  |  |
| reserved | 7:3 | RO | 0 | / |  |
| voff_thld | 2:0 | RW | EFUSE | POR | Battery Voltage for POWEROFF <br> $2.6^{\circ} 3,3 \mathrm{~V}, 0.1 \mathrm{~V} /$ step, 8 steps <br> 000: 2.6 V <br> 001: 2.7 V <br> $\cdots \cdots$ |

# Page 28

AXP2101
April 28, 2019

| pwr_time_ctr <br> 1 | 0x25 |  |  |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: |
| reserved | 7:5 | RO | 0 | $/$ |  |
| pwrok_chk_en | 4 | RW | 1b | POR | Check the PWROK Pin enable after all dcdc/1do output valid 128ms <br> 0 : disable <br> 1: enable |
| pwroff_dly_en | 3 | RW | 1b | POR | POWEROFF Delay 4ms after PWROK disable <br> 0 : disable <br> 1: enable |
| pwroff_seq_ctr1 | 2 | RW | 0b | POR | POWEROFF Sequence Control <br> 0 : At the same time <br> 1: the reverse of the Startup |
| pwrok_dly | $1: 0$ | RW | EFUSE | POR | Delay of PWROK after all power output good 00: 8ms <br> 01: 16ms <br> 10: 32ms <br> 11: 64ms |
| sleep_cfg | $0 \times 26$ |  |  |  |  |
| reserved | 7:5 | RO | 0 | $/$ |  |
| irq_wakup_en | 4 | RW | 0b | POR | IRQ Pin low to Wakeup <br> 0 : disable <br> 1: enable |
| pwrok_wakup_ind | 3 | RW | 1b | POR | PWROK be low-level enable when Wakup <br> 0 : disable <br> 1: enable |
| wakup_cfg_sel | 2 | RW | 0b | POR | DCDC/LDO Voltage Select when Wakup <br> 0 : The Default <br> 1: The voltage before wakup |
| wakup_en | 1 | RWLC | 0b | System Reset | Wake Up enable <br> 0 : disable <br> 1: enable |
| sleep_en | 0 | RWLC | 0b | System Reset | SLEEP enable <br> 0 : disable <br> 1: enalbe |
| ponlevel | $0 \times 27$ |  |  |  |  |
| reserved | 7:6 | RO | 0 | $/$ |  |
| irqleve1 | $5: 4$ | RW | 01b | POR | IRQLEVEL config <br> 00: 1s <br> 01: 1.5 s <br> 10: 2 s <br> 11: 2.5 s |
| offleve1 | $3: 2$ | RW | 01b | POR | OFFLEVEL config |

# Page 29

|  |  |  |  |  | $\begin{aligned} & 00: 4 \mathrm{~s} \\ & 01: 6 \mathrm{~s} \\ & 10: 8 \mathrm{~s} \\ & 11: 10 \mathrm{~s} \end{aligned}$ |
| :--: | :--: | :--: | :--: | :--: | :--: |
| on1eve1 | $1: 0$ | HW | EFUSE | POR | OSLEVEL config <br> 00: 128 ms <br> 01: 512 ms <br> 10: 1 s <br> $11: 2 \mathrm{~s}$ |
| fast_pwron_c fg0 | $0 \times 28$ |  |  |  |  |
| dcdc4_fstart_se q | $7: 6$ | HW | 0b | POR | DCDC4 Fast Power On Start Sequence 00 10: Start Sequence Code 11: disable |
| dcdc3_fstart_se <br> q | $5: 4$ | HW | 0b | POR | DCDC3 Fast Power On Start Sequence 00 10: Start Sequence Code 11: disable |
| dcdc2_fstart_se <br> q | $3: 2$ | HW | 0b | POR | DCDC2 Fast Power On Start Sequence 00 10: Start Sequence Code 11: disable |
| dcdc1_fstart_se <br> q | $1: 0$ | HW | 0b | POR | DCDC1 Fast Power On Start Sequence 00 10: Start Sequence Code 11: disable |
| fast_pwron_c fg1 | $0 \times 29$ |  |  |  |  |
| aldo3_fstart_se <br> q | $7: 6$ | HW | 0b | POR | ALDO3 Fast Power On Start Sequence 00 10: Start Sequence Code 11: disable |
| aldo2_fstart_se <br> q | $5: 4$ | HW | 0b | POR | ALDO2 Fast Power On Start Sequence 00 10: Start Sequence Code 11: disable |
| aldo1_fstart_se <br> q | $3: 2$ | HW | 0b | POR | ALDO1 Fast Power On Start Sequence 00 10: Start Sequence Code 11: disable |
| dcdc5_fstart_se <br> q | $1: 0$ | HW | 0b | POR | DCDC5 Fast Power On Start Sequence 00 10: Start Sequence Code 11: disable |
| fast_pwron_c fg2 | $0 \times 2 \mathrm{~A}$ |  |  |  |  |
| cpus1do_fstart_ seq | $7: 6$ | HW | 0b | POR | CPUSLDO Fast Power On Start Sequence 00 10: Start Sequence Code 11: disable |
| b1do2_fstart_se <br> q | $5: 4$ | HW | 0b | POR | BLDO2 Fast Power On Start Sequence 00 10: Start Sequence Code |

# Page 30

AXP2101
April, 28, 2019

|  |  |  |  |  | 11: disable |
| :--: | :--: | :--: | :--: | :--: | :--: |
| b1do1_fstart_se q | $3: 2$ | RW | 0b | POR | BLDO1 Fast Power On Start Sequence 00 10: Start Sequence Code 11: disable |
| aldo4_fstart_se <br> q | $1: 0$ | RW | 0b | POR | ALDO4 Fast Power On Start Sequence 00 10: Start Sequence Code 11: disable |
| fast_pwron_c fg3 | $0 \times 2 B$ |  |  |  |  |
| fast_pwron_en | 7 | RW | 0b | POR | Fast Power On Enable <br> 0: disable <br> 1: enable |
| fast_wakup_en | 6 | RW | 0b | POR | Fast Wake up Enable <br> 0: disable <br> 1: enable |
| reserved | $5: 4$ | RO | 0b | / |  |
| d1do2_fstart_se <br> q | $3: 2$ | RW | 0b | POR | DLDO2 Fast Power On Start Sequence 00 10: Start Sequence Code 11: disable |
| d1do1_fstart_se <br> q | $1: 0$ | RW | 0b | POR | DLDO1 Fast Power On Start Sequence 00 10: Start Sequence Code 11: disable |
| adc_ch_en0 | $0 \times 30$ |  |  |  |  |
| reserved | $7: 6$ | RO | 0 | / |  |
| gpadc_ch_en | 5 | RW | 0b | POR | general purpose ADC channel enable <br> 0: disable <br> 1: enable |
| tdie_ch_en | 4 | RW | 0b | POR | die temperature measure ADC channel enable <br> 0: disable <br> 1: enable |
| vsys_ch_en | 3 | RW | 0b | POR | system voltage voltage measure ADC channel enable <br> 0: disable <br> 1: enable |
| vbus_ch_en | 2 | RW | 0b | POR | vbus voltage measure ADC channel enable <br> 0: disable <br> 1: enable |
| ts_ch_en | 1 | RW | $1 b$ | POR | TS pin measure ADC channel enable <br> 0: disable <br> 1: enable |
| vbat_ch_en | 0 | RW | $1 b$ | POR | battery voltage measure ADC channel enable <br> 0: disable |

# Page 31

AXP2101
April, 28, 2019

|  |  |  |  |  | 1: enable |
| :--: | :--: | :--: | :--: | :--: | :--: |
| vbat_h | $0 \times 34$ |  |  |  |  |
| ch_dbg_en_1 | $7: 6$ | RW | 0b | POR | ch_dbg_en_1 is ch_dbg_en[1:0] ch_dbg_en: <br> 000: disable <br> 001: vbat use all channels <br> 010: ts use all channels <br> 011: vbus use all channels <br> 100: vsys use all channels <br> 101: tdie use all channels <br> 110: gpadc use all channels <br> 111: reserved |
| vbat_h | $5: 0$ | RO | 0b | POR | vbat $[13: 8]$ |
| vbat_1 | $0 \times 35$ |  |  |  |  |
| vbat_1 | $7: 0$ | RO | 0b | POR | vbat $[7: 0]$ |
| ts_h | $0 \times 36$ |  |  |  |  |
| adc_1f_en | 7 | RW | 1b | POR | ADC in low frequence sample mode when <br> PWROFF and Battery only enable(64s) <br> 0: disable <br> 1: enable |
| ch_dbg_en_h | 6 | RW | 0b | POR | ch_dbg_en_h is ch_dbg_en[2] |
| ts_h | $5: 0$ | RO | 0b | POR | ts $[13: 8]$ |
| ts_1 | $0 \times 37$ |  |  |  |  |
| ts_1 | $7: 0$ | RO | 0b | POR | ts $[7: 0]$ |
| vbus_h | $0 \times 38$ |  |  |  |  |
| reserved | $7: 6$ | RO | 0 | / |  |
| vbus_h | $5: 0$ | RO | 0b | POR | vbus $[13: 8]$ |
| vbus_1 | $0 \times 39$ |  |  |  |  |
| vbus_1 | $7: 0$ | RO | 0b | POR | vbus $[7: 0]$ |
| vsys_h | $0 \times 3 A$ |  |  |  |  |
| reserved | $7: 6$ | RO | 0 | / |  |
| vsys_h | $5: 0$ | RO | 0b | POR | vsys $[13: 8]$ |
| vsys_1 | $0 \times 3 B$ |  |  |  |  |
| vsys_1 | $7: 0$ | RO | 0b | POR | vsys $[7: 0]$ |
| tdie_h | $0 \times 3 C$ |  |  |  |  |
| reserved | $7: 6$ | RO | 0 | / |  |
| tdie_h | $5: 0$ | RO | 0b | POR | tdie $[13: 8]$ |
| tdie_1 | $0 \times 3 D$ |  |  |  |  |
| tdie_1 | $7: 0$ | RO | 0b | POR | tdie $[7: 0]$ |
| gpadc_h | $0 \times 3 E$ |  |  |  |  |
| reserved | $7: 6$ | RO | 0 | / |  |
| gpadc_h | $5: 0$ | RO | 0b | POR | gpadc $[13: 8]$ |
| gpadc_1 | $0 \times 3 F$ |  |  |  |  |

# Page 32

AXP2101
April, 28, 2019

| gpadc_1 | 7:0 | H0 | 0b | POR | gpadc[7:0] |
| :--: | :--: | :--: | :--: | :--: | :--: |
| irq_en0 | 0x40 |  |  |  |  |
| socw12_irq_en | 7 | HW | 1b | System Reset | SOC drop to Warning Level2 <br> IRQ (socw12_irq) enable <br> 0 : disable <br> 1: enable |
| socw11_irq_en | 6 | HW | 1b | System Reset | SOC drop to Warning Level1 <br> IRQ (socw11_irq) enable <br> 0 : disable <br> 1: enable |
| gwdt_irq_en | 5 | HW | 1b | System Reset | Gauge Watchdog Timeout IRQ(gwdt_irq) enable <br> 0 : disable <br> 1: enable |
| newsoc_irq_en | 4 | HW | 1b | System Reset | Gauge New SOC IRQ (lowsoc_irq) enable <br> 0 : disable <br> 1: enable |
| bcot_irq_en | 3 | HW | 1b | System Reset | Battery Over Temperature in Charge mode IRQ(bcot_irq) enable <br> 0 : disable <br> 1: enable |
| bcut_irq_en | 2 | HW | 1b | System Reset | Battery Under Temperature in Charge mode IRQ(bcut_irq) enable <br> 0 : disable <br> 1: enable |
| bwot_irq_en | 1 | HW | 1b | System Reset | Battery Over Temperature in Work mode IRQ(bwot_irq) enable <br> 0 : disable <br> 1: enable |
| bwut_irq_en | 0 | HW | 1b | System Reset | Battery Under Temperature in Work mode IRQ(bwut_irq) enable <br> 0 : disable <br> 1: enable |
| irq_en1 | 0x41 |  |  |  |  |
| vinsert_irq_en | 7 | HW | 1b | System Reset | VBUS Insert IRQ(vinsert_irq) enable <br> 0 : disable <br> 1: enable |
| vremove_irq_en | 6 | HW | 1b | System Reset | VBUS Remove IRQ(vremove_irq) enable <br> 0 : disable <br> 1: enable |
| binsert_irq_en | 5 | HW | 1b | System Reset | Battery Insert IRQ (binsert_irq) enable <br> 0 : disable <br> 1: enable |
| bremove_irq_en | 4 | HW | 1b | System Reset | Battery Remove IRQ (bremove_irq) enable |

# Page 33

AXP2101
April 28, 2019

|  |  |  |  |  | 0 : disable <br> 1 : enable |
| :--: | :--: | :--: | :--: | :--: | :--: |
| pons_irq_en | 3 | RW | 1b | System Reset | POWERON Short PRESS IRQ(ponsp_irq_en) enable <br> 0 : disable <br> 1 : enable |
| ponl_irq_en | 2 | RW | 1b | System Reset | POWERON Long PRESS IRQ(ponlp_irq) enable 0 : disable <br> 1 : enable |
| ponn_irq_en | 1 | RW | 0b | System Reset | POWERON Negative Edge IRQ(ponne_irq_en) enable <br> 0 : disable <br> 1 : enable |
| ponp_irq_en | 0 | RW | 0b | System Reset | POWERON Positive Edge IRQ(ponpe_irq_en) enable <br> 0 : disable <br> 1 : enable |
| irq_en2 | $0 \times 42$ |  |  |  |  |
| wdexp_irq_en | 7 | RW | 0b | System Reset | Watchdog Expire IRQ(wdexp_irq) enable 0 : disable <br> 1: enable |
| 1dooc_irq_en | 6 | RW | 1b | System Reset | LDO Over Current IRQ(1dooc_irq) enable <br> 0 : disable <br> 1 : enable |
| bocp_irq_en | 5 | RW | 0b | System Reset | BATFET Over Current Protection <br> IRQ(bocp_irq) enable <br> 0 : disable <br> 1 : enable |
| chgdn_irq_en | 4 | RW | 1b | System Reset | Battery charge done IRQ(chgdn_irq) enable <br> 0 : disable <br> 1 : enable |
| chgst_irq_en | 3 | RW | 1b | System Reset | Charger start IRQ(chgst_irq) enable <br> 0 : disable <br> 1 : enable |
| dot11_irq_en | 2 | RW | 1b | System Reset | DIE Over Temperature level1 <br> IRQ (dot11_irq) enable <br> 0 : disable <br> 1 : enable |
| chgte_irq_en | 1 | RW | 1b | System Reset | Charger Safety Timer1/2 expire <br> IRQ(chgte_irq) enable <br> 0 : disable <br> 1 : enable |
| bovp_irq_en | 0 | RW | 1b | System Reset | Battery Over Voltage Protection |

# Page 34

|  |  |  |  |  | IRQ (bovp_irq) enable <br> 0 : disable <br> 1: enable |
| :--: | :--: | :--: | :--: | :--: | :--: |
| irq0 | $0 \times 48$ |  |  |  |  |
| socw12_irq | 7 | RW1C | 0b | POR | SOC drop to Warning Level IRQ <br> 0 : no irq <br> 1: irq <br> when SOC $>$ = Warning Level or SOC <br> Shundown Level to clear it |
| socw11_irq | 6 | RW1C | 0b | POR | SOC drop to Shutdown Level IRQ <br> 0 : no irq <br> 1: irq <br> when SOC $>$ = Shutdown Level to clear it |
| gwdt_irq | 5 | RW1C | 0b | POR | Gauge Watchdog Timeout IRQ <br> 0 : no irq <br> 1: irq |
| newsoc_irq | 4 | RW1C | 0b | POR | Gauge New SOC IRQ <br> 0 : no irq <br> 1: irq |
| bcot_irq | 3 | RW1C | 0b | POR | Battery Over Temperature in Charge mode <br> IRQ <br> 0 : no irq <br> 1: irq <br> Battery Temperature to normal to clear it |
| bcut_irq | 2 | RW1C | 0b | POR | Battery Under Temperature in Charge mode <br> IRQ <br> 0 : no irq <br> 1: irq <br> Battery Temperature to normal to clear it |
| bwot_irq | 1 | RW1C | 0b | System Reset | Battery Over Temperature in Work mode IRQ <br> 0 : no irq <br> 1: irq <br> Battery Temperature to normal to clear it |
| bwut_irq | 0 | RW1C | 0b | System Reset | Battery Under Temperature in Work mode <br> IRQ <br> 0 : no irq <br> 1: irq <br> Battery Temperature to normal to clear it |
| irq1 | $0 \times 49$ |  |  |  |  |
| vinsert_irq | 7 | RW1C | 0b | POR | VBUS Insert IRQ <br> 0 : no irq <br> 1: irq <br> VBUS Remove to clear it |
| vremove_irq | 6 | RW1C | 0b | POR | VBUS Remove IRQ |

# Page 35

AXP2101
April 28, 2019

|  |  |  |  |  | 0: no irq <br> 1: irq <br> VBUS Insert to clear it |
| :--: | :--: | :--: | :--: | :--: | :--: |
| binsert_irq | 5 | RW1C | 0b | POR | Battery Insert IBQ <br> 0: no irq <br> 1: irq <br> Battery Remove to clear it |
| bremove_irq | 4 | RW1C | 0b | POR | Battery Remove IBQ <br> 0: no irq <br> 1: irq <br> Battery Insert to clear it |
| pons_irq | 3 | RW1C | 0b | System Reset | POWERON Short PRESS IBQ <br> 0: no irq <br> 1: irq |
| ponl_irq | 2 | RW1C | 0b | System Reset | POWERON Long PRESS IBQ <br> 0: no irq <br> 1: irq |
| ponn_irq | 1 | RW1C | 0b | System Reset | POWERON Negative Edge IBQ <br> 0: no irq <br> 1: irq |
| ponp_irq | 0 | RW1C | 0b | System Reset | POWERON Positive Edge IBQ <br> 0: no irq <br> 1: irq |
| irq2 | $0 \times 4 A$ |  |  |  |  |
| wdexp_irq | 7 | RW1C | 0b | POR | Watchdog Expire IBQ <br> 0: no irq <br> 1: irq |
| 1dooc_irq | 6 | RW1C | 0b | System Reset | LDO Over Current IBQ <br> 0: no irq <br> 1: irq <br> LDO Current to normal to clear it |
| bocp_irq | 5 | RW1C | 0b | POR | BATFET Over Current Protection IBQ <br> 0: no irq <br> 1: irq |
| chgdn_irq | 4 | RW1C | 0b | POR | Battery charge done IBQ <br> 0: no irq <br> 1: irq <br> Battery charge start to clear it |
| chgst_irq | 3 | RW1C | 0b | POR | Battery charge start IBQ <br> 0: no irq <br> 1: irq <br> Battery charge done to clear it |
| dot11_irq | 2 | RW1C | 0b | POR | DIE Over Temperature level1 IBQ <br> 0: no irq |

# Page 36

AXP2101
April 28, 2019

| chgte_irq | chgte_irq | 1 | RW1C | 0b | POR | 1: irq <br> DIE Temperature to normal to clear it <br> Charger Safety Timer1/2 expire IRQ <br> 0: no irq <br> 1: irq |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| bovp_irq |  | 0 | RW1C | 0b | POR | Battery Over Voltage Protection IRQ <br> 0: no irq <br> 1: irq <br> Battery Voltage to normal to clear it |
| ts_cfg |  | $0 \times 50$ |  |  |  |  |
| reserved |  | $7: 5$ | RO | 0 | / |  |
| ts_func |  | 4 | RW | EFUSE | POR | TS PIN function select: <br> 0: TS pin is the battery temperature sensor input and will affect the charger <br> 1: TS pin is the external fixed input and doesn't affect the charger |
| ts_src_en |  | $3: 2$ | RW | EFUSE | POR | TS current source on/off enable 00: off <br> 01: on when TS channel of ADC is enabled 10: on only when TS channel is working and off when others channel is working 11: always on |
| ts_curr |  | $1: 0$ | RW | 10b | POR | current source to TS pin config <br> 00: 20uA <br> 01: 40uA <br> 10: 50uA <br> 11: 60uA |
| ts_hys12h |  | $0 \times 52$ |  |  |  |  |
| ts_hys12h |  | $7: 0$ | RW | 2 h | POR | hysteresis for TS from low go to normal Thys $=$ N $* 16 \mathrm{mV}$ (default 32 mV ) |
| ts_hysh21 |  | $0 \times 53$ |  |  |  |  |
| ts_hysh21 |  | $7: 0$ | RW | 1 h | POR | hysteresis for TS from high go to normal Thys $=$ N $* 4 \mathrm{mV}$ (default 4 mV ) |
| v1tf_chg |  | $0 \times 54$ |  |  |  |  |
| v1tf_chg |  | $7: 0$ | RW | 29h | POR | VLTF in voltage of charge config <br> VLTF $=$ N $* 32 \mathrm{mV}$ (default is about 0deg) <br> This is also T1 of JEITA |
| vhtf_chg |  | $0 \times 55$ |  |  |  |  |
| vhtf_chg |  | $7: 0$ | RW | 58h | POR | VHTF in voltage of charge config <br> VHTF $=$ N $* 2 \mathrm{mV}$ (default is about 55deg) <br> This is also T4 of JEITA |
| v1tf_work |  | $0 \times 56$ |  |  |  |  |
| v1tf_work |  | $7: 0$ | RW | 3Eh | POR | VLTF in voltage of work config <br> VLTF $=$ N $* 32 \mathrm{mV}$ (default is about -10 deg ) |

# Page 37

AXP2101
April 28, 2019

| vhtf_work | 0x57 |  |  |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: |
| vhtf_work | 7:0 | RW | 4 Ch | POR | VHTF in voltage of work config <br> VHTF $=$ N*2 mV (default is about 60deg) |
| jeita_cfg | 0x58 |  |  |  |  |
| reserved | 7:6 | RO | 0 | / |  |
| jeita_en | 0 | RW | EFUSE | POR | JEITA Standard Enable <br> 0: disable <br> 1: enable |
| jeita_cv_cfg | 0x59 |  |  |  |  |
| reserved | 7 | RO | 0 | / |  |
| jwarm_ifall | 6 | RW | 0 b | POR | Current fall of Warm in JEITA Standard <br> 0: $100 \%$ <br> 1: $50 \%$ |
| reserved | 5 | RO | 0 | / |  |
| jcool_ifall | 4 | RW | $1 b$ | POR | Current fall of Cool in JEITA Standard <br> 0: $100 \%$ <br> 1: $50 \%$ |
| jwarm_vfall | $3: 2$ | RW | $01 b$ | POR | Voltage fall of Warm in JEITA Standard <br> 00: 0 mV <br> $01: 4.1 / 4.2 / 4.35 / 4.4 \mathrm{~V}$ 的 1 档位 <br> $10: 4.1 / 4.2 / 4.35 / 4.4 \mathrm{~V}$ 的 2 档位 <br> 11: reserved |
| jcool_vfall | $1: 0$ | RW | $00 b$ | POR | Voltage fall of Cool in JEITA Standard <br> 00: 0 mV <br> $01: 4.1 / 4.2 / 4.35 / 4.4 \mathrm{~V}$ 的 1 档位 <br> $10: 4.1 / 4.2 / 4.35 / 4.4 \mathrm{~V}$ 的 2 档位 <br> 11: reserved |
| jeita_cool | 0x5A |  |  |  |  |
| jeita_cool | 7:0 | RW | 37 h | POR | Cool Temprature (T2) in voltage of charge <br> config <br> VHTF $=$ N*16 mV (default is about 10deg) |
| jeita_warm | 0x5B |  |  |  |  |
| jeita_warm | 7:0 | RW | 1Eh | POR | Warm Temprature (T3) in voltage of charge <br> config <br> VHTF $=$ N*8 mV (default is about 45deg) |
| ts_cfg_data_ <br> h | 0x5C |  |  |  |  |
| reserved | 7:6 | RO | 0 | / |  |
| ts_cfg_data_h | 5:0 | RW | 2 h | POR | ts_cfg_data[13:8] |
| ts_cfg_data_ <br> 1 | 0x5D |  |  |  |  |
| ts_cfg_data_1 | 7:0 | RW | 58 h | POR | ts_cfg_data[7:0], ts_cfg_data is TS <br> Voltage configured by MCU when ts_ch_en <br> $=0 b$ |

# Page 38

AXP2101
April, 28, 2019

| chg_cfg | 0x60 |  |  |  |  |
| :--: | :--: | :--: | :--: | :--: | :--: |
| reserved | 7:2 | RO | 0 | / |  |
| vrechg_rechg_en | 1 | RW | 1b | POR | Recharge with Battery Voltage below <br> Vrechg enable <br> 0 : disable <br> 1: enable |
| gauge_rechg_en | 0 | RW | 0b | POR | Recharge with Egauge SOC Level enable <br> 0 : disable <br> 1: enable |
| iprechg_cfg | 0x61 |  |  |  |  |
| reserved | $7: 4$ | RO | 0 | / |  |
| iprechg_cfg | 3:0 | RW | 0101b | POR | Precharge current 1imit: <br> $25 * \mathrm{~N} \mathrm{~mA}$ <br> 0000: 0 mA <br> 0001: 25 mA <br> 0010: 50 mA <br> 0011: 75 mA <br> 0100: 100mA <br> 0101: 125 mA <br> 0110: 150 mA <br> 0111: 175 mA <br> 1000: 200mA <br> $1001^{\wedge} 1111$ : reserved |
| icc_cfg | 0x62 |  |  |  |  |
| reserved | 7:5 | RO | 0 | / |  |
| icchg_cfg | 4:0 | RW | {EFUSE, 0b <br> ,EFUSE} | POR | constant current charge current 1imit: <br> $25 * \mathrm{~N} \mathrm{~mA}$ if $\mathrm{N}<\varnothing 8$ <br> $200+100 *(N-8)$ mA if $N>8$ <br> 00000: 0mA <br> 00100: 100mA <br> 00101: 125 mA <br> 00110: 150mA <br> 00111: 175 mA <br> 01000: 200mA <br> 01001: 300mA <br> 01010: 400mA <br> 01011: 500mA <br> 01100: 600mA <br> 01101: 700mA <br> 01110: 800mA <br> 01111: 900mA <br> 10000: 1000mA <br> others: reserved |
| iterm_cfg | 0x63 |  |  |  |  |

# Page 39

AXP2101
April, 28, 2019

| reserved | 7:5 | RO | 0 b | $/$ |  |
| :--: | :--: | :--: | :--: | :--: | :--: |
| iterm_en | 4 | RW | 1b | System Reset | Charging termination of current enable <br> 0 : disable <br> 1: enable |
| iterm_cfg | 3:0 | RW | 0101b | POR | Termination current limit: <br> $25 * N \mathrm{~mA}$ <br> 0000: 0 mA <br> 0001: 25 mA <br> 0010: 50 mA <br> 0011: 75 mA <br> 0100: 100mA <br> 0101: 125 mA <br> 0110: 150 mA <br> 0111: 175 mA <br> 1000: 200mA <br> $1001^{\sim} 1111$ : reserved |
| chg_v_cfg | 0x64 |  |  |  |  |
| reserved | $7: 3$ | RO | 0 | $/$ |  |
| vterm_cfg | $2: 0$ | RW | 011b | POR | Charge voltage limit <br> $000: 4,6 \mathrm{~V}$ <br> $001: 4,0 \mathrm{~V}$ <br> $010: 4,1 \mathrm{~V}$ <br> $011: 4,2 \mathrm{~V}$ <br> $100: 4,35 \mathrm{~V}$ <br> $101: 4,4 \mathrm{~V}$ <br> $11 X$ : reserved |
| tregu_thld | 0x05 |  |  |  |  |
| reserved | $7: 2$ | RO | 0 | $/$ |  |
| tregu_thld | $1: 0$ | RW | 10b | System Reset | Thermal regulation threshold <br> 00: 60deg <br> 01: 80deg <br> 10: 100deg <br> 11: 120deg |
| chg_tmr_cfg | 0x07 |  |  |  |  |
| tmr_dt_en | 7 | RW | 1b | POR | safety timer $1 / 2$ setting during DPM or thermal regulation <br> 0 : safety timer not slowed during input DPM or thermal regulation <br> 1: safety timer slowed during input DPM or thermal regulation |
| chg_tmr2_en | 6 | RW | 1b | POR | charge done safe timer enable <br> 0 : disable <br> 1: enable |
| chg_tmr2_cfg | $5: 4$ | RW | 10b | POR | charge done safety timer config |

# Page 40

AXP2101
April 28, 2019

|  |  |  |  |  | 00: 5hours <br> 01: 8hours <br> 10: 12hours <br> 11: 20hours |
| :--: | :--: | :--: | :--: | :--: | :--: |
| reserved | 3 | RO | 0 | / |  |
| chg_tmr1_en | 2 | RW | 1b | POR | pre-charge safe timer enable <br> 0 : disable <br> 1: enable |
| chg_tmr1_cfg | $1: 0$ | RW | $10 b$ | POR | pre-charge safe timer config <br> 00: 40mins <br> 01: 50mins <br> 10: 60mins <br> 11: 70mins |
| bat_det | 0x68 |  |  |  |  |
| reserved | $7: 1$ | RO | 0 | / |  |
| bat_det_en | 0 | RW | $1 b$ | POR | battery detection enable <br> 0 : disable <br> 1: enable |
| chgled_cfg | 0x69 |  |  |  |  |
| reserved | $7: 6$ | RO | 0 | / |  |
| chgled_out_ctr1 | $5: 4$ | RW | $00 b$ | System Reset | CHGLED pin output whe the register of chgled_func is set to 10b <br> 00: Hiz; <br> 01: Low/Hiz 25\%/75\% duty 1Hz; <br> 10: Low/Hiz 25\%/75\% duty 4Hz; <br> 11: drive low; |
| reserved | 3 | RO | 0 | / |  |
| chgled_func | $2: 1$ | RW | EFUSE | POR | CHGLED pin display function config <br> 00: dispaly with type A function <br> 01: display with type B function <br> 10: output controlled by the register of <br> chgled_out_ctr1 <br> 11: reserved |
| chgled_en | 0 | RW | $1 b$ | POR | CHGLED pin enable <br> 0 : disable CHGLED pin function <br> 1: enable CHGLED pin function |
| btn_chg_cfg | 0x6A |  |  |  |  |
| reserved | $7: 3$ | RO | 0 | / |  |
| btn_chg_cfg | $2: 0$ | RW | 011b | POR | Button Battery charge termination voltage <br> $2.6^{\circ} 3.3 \mathrm{~V}, 100 \mathrm{mV} /$ step, 8 steps <br> $000: 2.6 \mathrm{~V}$ <br> $001: 2.7 \mathrm{~V}$ <br> $010: 2.8 \mathrm{~V}$ |

# Page 41

|  |  |  |  |  | $\begin{aligned} & 011: 2.9 \mathrm{~V} \\ & 100: 3.0 \mathrm{~V} \\ & 101: 3.1 \mathrm{~V} \\ & 110: 3.2 \mathrm{~V} \\ & 111: 3.3 \mathrm{~V} \end{aligned}$ |
| :--: | :--: | :--: | :--: | :--: | :--: |
| dcdc_cfg0 | 0x80 |  |  |  |  |
| reserved | 7 | R0 | 0b | / |  |
| dcdc_fccm | 6 | RW | 0b | System Reset | force DCDC work in CCM mode <br> 0 : disable <br> 1: enable |
| dvm_speed | 5 | RW | 0b | System Reset | DVM voltage ramp control <br>$0: 15.625$ us/step <br>1: 31.250 us/step |
| dcdc5_en | 4 | RW | EFUSE | System Reset | DCDC5 enable <br> 0 : disable <br> 1: enable |
| dcdc4_en | 3 | RW | EFUSE | System Reset | DCDC4 enable <br> 0 : disable <br> 1: enable |
| dcdc3_en | 2 | RW | EFUSE | System Reset | DCDC3 enable <br> 0 : disable <br> 1: enable |
| dcdc2_en | 1 | RW | EFUSE | System Reset | DCDC2 enable <br> 0 : disable <br> 1: enable |
| dcdc1_en | 0 | RW | EFUSE | System Reset | DCDC1 enable (EFUSE, aldo1_start_seq=7 时 default=0, 否则 default=1) <br> 0 : disable <br> 1: enable |
| dcdc_cfg1 | 0x81 |  |  |  |  |
| dcdc_fspd_en | 7 | RW | 0b | System Reset | DCDC frequency spread enable <br> 0 : disable <br> 1: enable |
| dcdc_fspd_ctr1 | 6 | RW | 0b | System Reset | DCDC frequency spead range contr1 <br>0: 50 KHz <br>1: 100 kHz |
| dcdc4_mode | 5 | RW | 0b | System Reset | DCDC4 PWM/PFM Control <br>0: Auto Switch <br>1: Always PWM |
| dcdc3_mode | 4 | RW | 0b | System Reset | DCDC3 PWM/PFM Control <br>0: Auto Switch <br> 1: Always PWM |
| dcdc2_mode | 3 | RW | 0b | System Reset | DCDC2 PWM/PFM Control <br>0: Auto Switch |

# Page 42

AXP2101
April 28, 2019

|  |  |  |  |  | 1: Always PWM |
| :--: | :--: | :--: | :--: | :--: | :--: |
| dcdc1_mode | 2 | RW | 0b | System Reset | DCDC1 PWM/PFM Control <br> 0: Auto Switch <br> 1: Always PWM |
| dcdc_uvp_dbc | $1: 0$ | RW | 00b | POR | DCDC UVP debounce time config <br> 00: 60us <br> 01: 120us <br> 10: 180us <br> 11: 240us |
| dcdc1_cfg | $0 \times 82$ |  |  |  |  |
| reserved | $7: 5$ | RO | 0 | / |  |
| dcdc1_out | $4: 0$ | RW | EFUSE | System Reset | DCDC1 output voltage config <br> $1.5^{\sim} 3.4 \mathrm{~V}, 100 \mathrm{mV} /$ step, 20steps <br> 00000: 1.5 V <br> 00001: 1.6 V <br> $\cdots \cdots$ <br> 10011: 3. 4 V <br> $10100^{\sim} 11111:$ reserved |
| dcdc2_cfg | $0 \times 83$ |  |  |  |  |
| dcdc2_dvm_en | 7 | RW | 0b | System Reset | DCDC2 DVM enable control <br> 0: disable <br> 1: enable |
| dcdc2_out | $6: 0$ | RW | EFUSE | System Reset | DCDC2 output voltage config <br> $0.5^{\sim} 1.2 \mathrm{~V}, 10 \mathrm{mV} /$ step, 71 steps <br> $1.22^{\sim} 1.54 \mathrm{~V}, 20 \mathrm{mV} /$ step, 17 steps <br> 0000000: 0.50 V <br> 0000001: 0.51 V <br> $\cdots \cdots$ <br> 1000110: 1. 20 V <br> 1000111: 1. 22 V <br> 1001000: 1. 24 V <br> $\cdots \cdots$ <br> 1010111: 1. 54 V <br> $1011000^{\sim} 1111111:$ reserved |
| dcdc3_cfg | $0 \times 84$ |  |  |  |  |
| dcdc3_dvm_en | 7 | RW | 0b | System Reset | DCDC3 DVM enable control <br> 0: disable <br> 1: enable |
| dcdc3_out | $6: 0$ | RW | EFUSE | System Reset | DCDC3 output voltage config <br> $0.5^{\sim} 1.2 \mathrm{~V}, 10 \mathrm{mV} /$ step, 71 steps <br> $1.22^{\sim} 1.54 \mathrm{~V}, 20 \mathrm{mV} /$ step, 17 steps <br> $1.6^{\sim} 3.4 \mathrm{~V}, 100 \mathrm{mV} /$ step, 19 steps <br> 0000000: 0.50 V <br> 0000001: 0.51 V |

# Page 43

![img-5.jpeg](img-5.jpeg)

AXP2101 April, 28, 2019

|   |  |  |  |  | ......  |
| --- | --- | --- | --- | --- | --- |
|   |  |  |  |  | 1000110: 1.20V  |
|   |  |  |  |  | 1000111: 1.22V  |
|   |  |  |  |  | 1001000: 1.24V  |
|   |  |  |  |  | 1010111: 1.54V  |
|   |  |  |  |  | 1011000: 1.60V  |
|   |  |  |  |  | 1011001: 1.70V  |
|   |  |  |  |  | 1101011: 3.40V  |
|   |  |  |  |  | 1101100~1111111: reserved  |
|  dcdc4_cfg | 0x85 |  |  |  |   |
|  reserved | 7 | RO | 0 | / |   |
|  dcdc4_out | 6:0 | RW | EFUSE | System Reset | DCDC4 output voltage config  |
|   |  |  |  |  | 0.5~1.2V, 10mV/step, 71steps  |
|   |  |  |  |  | 1.22~1.84V, 20mV/step, 32steps  |
|   |  |  |  |  | 0000000: 0.50V  |
|   |  |  |  |  | 0000001: 0.51V  |
|   |  |  |  |  | 1000110: 1.20V  |
|   |  |  |  |  | 1000111: 1.22V  |
|   |  |  |  |  | 1001000: 1.24V  |
|   |  |  |  |  | 1100110: 1.84V  |
|   |  |  |  |  | 1100111~1101000: reserved  |
|  dcdc5_cfg | 0x86 |  |  |  |   |
|  reserved | 7:6 | RO | 0 | / |   |
|  slow_comp | 5 | RW | 0b | System Reset | slow down dcdc5 frequency compensation  |
|   |  |  |  |  | enable  |
|   |  |  |  |  | 0: disable  |
|   |  |  |  |  | 1: enable  |
|  dcdc5_out | 4:0 | RW | EFUSE | System Reset | DCDC5 output voltage config  |
|   |  |  |  |  | 1.4~3.7V, 100mV/step, 24steps  |
|   |  |  |  |  | 00000: 1.4V  |
|   |  |  |  |  | 00001: 1.5V  |
|   |  |  |  |  | 10111: 3.7V  |
|   |  |  |  |  | 11000~11111: reserved  |
|  dcdc_oc_cfg | 0x87 |  |  |  |   |
|  reserved | 7:6 | RO | 0 | / |   |
|  dcdc3_oc | 5:4 | RW | EFUSE | POR | DCDC3 OC threshold config:  |
|   |  |  |  |  | 00: 3A  |
|   |  |  |  |  | 01: 3.5A  |
|   |  |  |  |  | 10: 4A  |

Revision 0.1 Copyright © 2019 X-Powers Limited. All Rights Reserved 43

# Page 44

AXP2101
April, 28, 2019

|  |  |  |  |  | 11: 5A |
| :--: | :--: | :--: | :--: | :--: | :--: |
| dcdc2_oc | $3: 2$ | RW | EFUSE | POR | DCDC2 OC threshold config: <br> 00: 2.5A <br> 01: 3A <br> 10: 3.5A <br> 11: 4A |
| dcdc1_oc | $1: 0$ | RW | EFUSE | POR | DCDC1 OC threshold config: <br> 00: 2.5A <br> 01: 3A <br> 10: 3.5A <br> 11: 4A |
| 1do_en_cfg0 | 0x90 |  |  |  |  |
| d1do1_en | 7 | RW | EFUSE | System Reset | d1do1 enable <br> 0: disable <br> 1: enable |
| cpus1do_en | 6 | RW | EFUSE | System Reset | cpus1do enable <br> 0: disable <br> 1: enable |
| b1do2_en | 5 | RW | EFUSE | System Reset | b1do2 enable <br> 0: disable <br> 1: enable |
| b1do1_en | 4 | RW | EFUSE | System Reset | a1do1 enable <br> 0: disable <br> 1: enable |
| a1do4_en | 3 | RW | EFUSE | System Reset | a1do4 enable <br> 0: disable <br> 1: enable |
| a1do3_en | 2 | RW | EFUSE | System Reset | a1do3 enable <br> 0: disable <br> 1: enable |
| a1do2_en | 1 | RW | EFUSE | System Reset | a1do2 enable <br> 0: disable <br> 1: enable |
| a1do1_en | 0 | RW | EFUSE | System Reset | a1do1 enable (EFUSE, a1do1_start_seq=7 时 default=0, 告别 default=1) <br> 0: disable <br> 1: enable |
| 1do_en_cfg1 | 0x91 |  |  |  |  |
| reserved | $7: 1$ | RO | 0 | / |  |
| d1do2_en | 0 | RW | EFUSE | System Reset | d1do2 enable <br> 0: disable <br> 1: enable |
| a1do1_cfg | 0x92 |  |  |  |  |
| reserved | $7: 5$ | RO | 0 | / |  |

# Page 45

AXP2101
April 28, 2019

| aldo1_out | 4:0 | RW | EFUSE | System Reset | aldo1 output voltage config <br> $0.5^{\sim} 3.5 \mathrm{~V}, 100 \mathrm{~mV} /$ step, 31steps <br> 00000: 0.5 V <br> 00001: 0.6 V <br> $\qquad$ <br> 11110: 3.5 V <br> 11111: reserved |
| :--: | :--: | :--: | :--: | :--: | :--: |
| aldo2_cfg | 0x93 |  |  |  |  |
| reserved | 7:5 | HO | 0 | / |  |
| aldo2_out | 4:0 | RW | EFUSE | System Reset | aldo2 output voltage config <br> $0.5^{\sim} 3.5 \mathrm{~V}, 100 \mathrm{mV} /$ step, 31steps <br> 00000: 0.5 V <br> 00001: 0.6 V <br> $\qquad$ <br> 11110: 3.5 V <br> 11111: reserved |
| aldo3_cfg | 0x94 |  |  |  |  |
| reserved | 7:6 | HO | 0 | / |  |
| aldo3_out | 4:0 | RW | EFUSE | System Reset | aldo3 output voltage config <br> $0.5^{\sim} 3.5 \mathrm{~V}, 100 \mathrm{mV} /$ step, 31steps <br> 00000: 0.5 V <br> 00001: 0.6 V <br> $\qquad$ <br> 11110: 3.5 V <br> 11111: reserved |
| aldo4_cfg | 0x95 |  |  |  |  |
| reserved | 7:6 | HO | 0 | / |  |
| aldo4_out | 4:0 | RW | EFUSE | System Reset | aldo4 output voltage config <br> $0.5^{\sim} 3.5 \mathrm{~V}, 100 \mathrm{mV} /$ step, 31steps <br> 00000: 0.5 V <br> 00001: 0.6 V <br> $\qquad$ <br> 11110: 3.5 V <br> 11111: reserved |
| b1do1_cfg | 0x96 |  |  |  |  |
| reserved | 7:5 | HO | 0 | / |  |
| b1do1_out | 4:0 | RW | EFUSE | System Reset | b1do1 output voltage config <br> $0.5^{\sim} 3.5 \mathrm{~V}, 100 \mathrm{mV} /$ step, 31steps <br> 00000: 0.5 V <br> 00001: 0.6 V <br> $\qquad$ <br> 11110: 3.5 V <br> 11111: reserved |
| b1do2_cfg | 0x97 |  |  |  |  |

# Page 46

AXP2101
April, 28, 2019

| reserved | 7:5 | RO | 0 | / |  |
| :--: | :--: | :--: | :--: | :--: | :--: |
| b1do2_out | $4: 0$ | RW | EFUSE | System Reset | b1do2 output voltage config <br> $0.5^{\sim} 3.5 \mathrm{~V}, 100 \mathrm{~mV} /$ step, 31steps <br> 00000: 0.5 V <br> 00001: 0.6 V <br> $\qquad$ <br> 11110: 3.5 V <br> 11111: reserved |
| cpus1do_cfg | 0x98 |  |  |  |  |
| reserved | $7: 5$ | RO | 0 | / |  |
| cpus1do_out | $4: 0$ | RW | EFUSE | System Reset | cpus1do output voltage config <br> $0.5^{\sim} 1.4 \mathrm{~V}, 50 \mathrm{mV} /$ step, 20steps <br> 00000: 0.50 V <br> 00001: 0.55 V <br> $\qquad$ <br> 10011: 1.40 V <br> $10100^{\sim} 11111:$ reserved |
| d1do1_cfg | 0x99 |  |  |  |  |
| reserved | $7: 5$ | RO | 0 | / |  |
| d1do1_out | $4: 0$ | RW | EFUSE | System Reset | d1do1 output voltage config <br> $0.5^{\sim} 3.5 \mathrm{~V}, 100 \mathrm{mV} /$ step, 31steps <br> 00000: 0.5 V <br> 00001: 0.6 V <br> $\qquad$ <br> 11110: 3.5 V <br> 11111: reserved |
| d1do2_cfg | 0x9A |  |  |  |  |
| reserved | $7: 5$ | RO | 0 | / |  |
| d1do2_out | $4: 0$ | RW | EFUSE | System Reset | d1do2 output voltage config <br> $0.5^{\sim} 1.4 \mathrm{~V}, 50 \mathrm{mV} /$ step, 20steps <br> 00000: 0.50 V <br> 00001: 0.55 V <br> $\qquad$ <br> 10011: 1.40 V <br> $10100^{\sim} 11111:$ reserved |
| ip_ver | 0x00 |  |  |  |  |
| ip_ver | 7:0 | RO | 01h | POR | Egauge IP version |
| brom | 0x01 |  |  |  |  |
| brom | 7:0 | RW | xx | POR | Battery parameter ROM |
| config | 0x02 |  |  |  |  |
| reserved | 7:6 | RO | 0b | / | reserved |
| reserved | 5 | RW | 0b | POR | reserved |
| rom_se1 | 4 | RW | 0b | POR | ROM or SRAM select <br> 1; select sram; |

# Page 47

|  |  |  |  |  | $0_{1}$ select rom; |
| :-- | :--: | :--: | :--: | :--: | :--: |
| reserved | $3: 1$ | RO | 0 b | $/$ | reserved |
|  |  |  |  |  |  |
| bromup_en | 0 | RW | 0 b | POR | brom writer control <br> 1:enable <br> 0:disable |
| soc | $0 \times 04$ |  |  |  |  |
| soc | $7: 0$ | RO | 00 h | POR | battery persentage |

# 9.Application Information 

### 9.1 Typical Application

![img-6.jpeg](img-6.jpeg)

# Page 48

# 10.Package and Ordering Information 

### 10.1 Package Information

AXP2101 package is QFN5*5, 40-pin. Figure 10-1 shows AXP2101 package.

## 待提供

Figure 10-1 Package Information

### 10.2 Marking information

Figure 10-2 shows AXP2101 marking.
待提供
Figure 10-2 AXP2101 Marking
Table 10-1 describes AXP2101 marking information.
Table 10-1 AXP2101 Marking Definitions

| No. | Marking | Description | Fixed/Dynamic |
| :--: | :--: | :--: | :--: |
| 1 | AXP2101 | Product name | Fixed |
| 2 | LLLLLCB | Lot number | Dynamic |
| 3 | XXX1 | Date code | Dynamic |
| 4 |  | X-POWERS logo | Fixed |
| 5 | White dot | Package pin 1 | Fixed |

### 10.3 Carrier

Table 10-2 shows AXP2101 tray carrier information
Table 10-2 Tray Carrier Information

| Item | Color | Size |
| :--: | :--: | :--: |
| Aluminum foil bags | Silvery white | $540 \mathrm{~mm} \times 300 \mathrm{~mm} \times 0.14 \mathrm{~mm}$ |
| Pearl cotton cushion(Vacuum bag) | White | $12 \mathrm{~mm} \times 680 \mathrm{~mm} \times 185 \mathrm{~mm}$ |
| Pearl cotton cushion (The Gap <br> between vacuum bag and inside <br> box) | White | Left-Right: $12 \mathrm{~mm} \times 180 \mathrm{~mm} \times 85 \mathrm{~mm}$ <br> Front-Back: $12 \mathrm{~mm} \times 350 \mathrm{~mm} \times 70 \mathrm{~mm}$ |
| Inside Box | White | $396 \mathrm{~mm} \times 196 \mathrm{~mm} \times 96 \mathrm{~mm}$ |
| Outside Box | White | $420 \mathrm{~mm} \times 410 \mathrm{~mm} \times 320 \mathrm{~mm}$ |

Figure 10-3 shows tray dimension drawing of AXP2101.

# Page 49

![img-7.jpeg](img-7.jpeg)

![img-8.jpeg](img-8.jpeg)

Figure 10-3 Tray Dimension Drawing

Table 10-3 shows AXP2101 packing quantity.

Table 10-3 Packing Quantity Information

|  Type | Quantity | Part Number  |
| --- | --- | --- |
|  Tray | 490pcs/Tray | AXP2101  |
|   | 10Trays/package |   |

### 10.4 Storage

#### 10.4.1 Moisture Sensitivity Level (MSL)

A package's MSL indicates its ability to withstand exposure after it is removed from its shipment bag, a low MSL device sample can be exposed on the factor floor longer than a high MSL device sample. ALL MSL are defined in Table 10-4.

Table 10-4 MSL Summary

|  MSL | Out-of-bag floor life | Comments  |
| --- | --- | --- |
|  1 | Unlimited | ≤30℃, 85%RH  |
|  2 | 1 year | ≤30℃, 60%RH  |
|  2a | 4 weeks | ≤30℃, 60%RH  |
|  3 | 168 hours | ≤30℃, 60%RH  |
|  4 | 72 hours | ≤30℃, 60%RH  |
|  5 | 48 hours | ≤30℃, 60%RH  |
|  5a | 24 hours | ≤30℃, 60%RH  |
|  6 | Time on Label (TOL) | ≤30℃, 60%RH  |

AXP2101 device samples are classified as MSL3.

# Page 50

# 10.4.2 Bagged Storage Conditions 

The shelf life of AXP2101 are defined in Table 10-5.
Table 10-5

| Packing mode | Vacuum packing |
| :-- | :-- |
| Storage temperature | $20^{\circ} \mathrm{C} \sim 26^{\circ} \mathrm{C}$ |
| Storage humidity | $40 \% \sim 60 \% \mathrm{RH}$ |
| Shelf life | 6 months |

### 10.4.3 Out-of-bag Duration

It is defined by the device MSL rating. The out-of-bag duration of AXP2101 is as follows.
Table 10-6 Out-of-bag Duration

| Storage temperature | $20^{\circ} \mathrm{C} \sim 26^{\circ} \mathrm{C}$ |
| :-- | :-- |
| Storage humidity | $40 \% \sim 60 \% \mathrm{RH}$ |
| Moisture Sensitivity Level(MSL) | 3 |
| Floor life | 168 hours |

For no mention of storage rules in this document, please refer to the latest IPC/JEDEC J-STD-020C.

### 10.5 Baking

It is not necessary to bake AXP2101 if the conditions specified in Section 16.4.2 and Section 16.4.3 have not been exceeded. It is necessary to bake AXP2101 if any condition specified in Section 10.4.2 and Section 10.4.3 have been exceeded.
It is necessary to bake AXP2101 if the storage humidity condition has been exceeded. We recommend that the device sample removed from its vacuum bag more than 2 days should be baked to guarantee production.

Table 10-7 Baking Conditions

| Surrounding | Bake@125 ${ }^{\circ} \mathrm{C}$ | Note |
| :-- | :-- | :-- |
| Nitrogen | 8 hours | Recommended condition. Not exceed 3 times. |
| Air | 2 hours | Acceptable condition. Not exceed 3 times. |
| CAUTION: If baking is required, the devices must be transferred into trays that can be baked to at least $125^{\circ} \mathrm{C}$. Devices should <br> not be baked in tape and reel carriers at any temperature |  |  |

## 11. Reflow Profile

The reflow profile recommended in this document is a lead-free reflow profile that is suitable for pure lead-free technology of lead-free solder paste.
Figure 10-1 shows the typical reflow profile of AXP2101 device sample.
![img-9.jpeg](img-9.jpeg)

# Page 51

Figure 11-1 AXP2101 Typical Reflow Profile
Reflow profile conditions of AXP2101 device sample is given in Table 11-1.
Table 11-1 AXP2101 Reflow Profile Conditions

|  | QTI typical SMT reflow profile conditions (for reference only) |  |
| :--: | :--: | :--: |
|  | Step | Reflow condition |
| Environment | N2 purge reflow usage (yes/no) | Yes, N2 purge used |
|  | If yes, 02 ppm level | $02<1500$ ppm |
| A | Preheat ramp up temperature range | $25^{\circ} \mathrm{C}->150^{\circ} \mathrm{C}$ |
| B | Preheat ramp up rate | $1.5^{\circ} 2.5^{\circ} \mathrm{C} / \mathrm{sec}$ |
| C | Soak temperature range | $150^{\circ} \mathrm{C}->190^{\circ} \mathrm{C}$ |
| D | Soak time | $80^{\circ} 110 \mathrm{sec}$ |
| E | Liquidus temperature | $217^{\circ} \mathrm{C}$ |
| F | Time above liquidus | $60-90 \mathrm{sec}$ |
| G | Peak temperature | $240-250^{\circ} \mathrm{C}$ |
| H | Cool down temperature rate | $\leq 4^{\circ} \mathrm{C} / \mathrm{sec}$ |

# Page 52

# Declaration 

THIS DOCUMENTATION IS THE ORIGINAL WORK AND COPYRIGHTED PROPERTY OF X-POWERS. REPRODUCTION IN WHOLE OR IN PART MUST OBTAIN THE WRITTEN APPROVAL OF X-POWERS AND GIVE CLEAR ACKNOWLEDGEMENT TO THE COPYRIGHT OWNER.

THE PURCHASED PRODUCTS, SERVICES AND FEATURES ARE STIPULATED BY THE CONTRACT MADE BETWEEN X-POWERS AND THE CUSTOMER. PLEASE READ THE TERMS AND CONDITIONS OF THE CONTRACT AND RELEVANT INSTRUCTIONS CAREFULLY BEFORE USING, AND FOLLOW THE INSTRUCTIONS IN THIS DOCUMENTATION STRICTLY. X-POWERS ASSUMES NO RESPONSIBILITY FOR THE CONSEQUENCES OF IMPROPER USE(INCLUDING BUT NOT LIMITED TO OVERVOLTAGE, OVERCLOCK, OR EXCESSIVE TEMPERATURE).

THE INFORMATION FURNISHED BY X-POWERS IS PROVIDED JUST AS A REFERENCE OR TYPICAL APPLICATIONS. ALL STATEMENTS, INFORMATION, AND RECOMMENDATIONS IN THIS DOCUMENT DO NOT CONSTITUTE A WARRANTY OF ANY KIND, EXPRESS OR IMPLIED. X-POWERS RESERVES THE RIGHT TO MAKE CHANGES IN CIRCUIT DESIGN AND/OR SPECIFICATIONS AT ANY TIME WITHOUT NOTICE.

NOR FOR ANY INFRINGEMENTS OF PATENTS OR OTHER RIGHTS OF THE THIRD PARTIES WHICH MAY RESULT FROM ITS USE. NO LICENSE IS GRANTED BY IMPLICATION OR OTHERWISE UNDER ANY PATENT OR PATENT RIGHTS OF X-POWERS. THIRD PARTY LICENCES MAY BE REQUIRED TO IMPLEMENT THE SOLUTION/PRODUCT. CUSTOMERS SHALL BE SOLELY RESPONSIBLE TO OBTAIN ALL APPROPRIATELY REQUIRED THIRD PARTY LICENCES. X-POWERS SHALL NOT BE LIABLE FOR ANY LICENCE FEE OR ROYALTY DUE IN RESPECT OF ANY REQUIRED THIRD PARTY LICENCE. X-POWERS SHALL HAVE NO WARRANTY, INDEMNITY OR OTHER OBLIGATIONS WITH RESPECT TO MATTERS COVERED UNDER ANY REQUIRED THIRD PARTY LICENCE.

