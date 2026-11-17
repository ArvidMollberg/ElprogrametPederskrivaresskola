här hamnar info om abb armen ifall vi har ork att ladda upp de de vill säga


ps om sprikan i skärme ej är fixad är det Eddies fel (+46 76 345 62 82) här kan man online mobba honom (karlbergsvägen 10) för dig som är mer fysisk.
grobot configuration
0.9 quick guide.
to get system functional a compressed guide is provided here.
firstly make sure you have the HMI(ipad machine).
then take left hand place it under slide it under the band tighten it.
after it is in a good position proceed with control learning.
right side of HMI theres an big plastic button. push it in and robot control joystick functions.
logger will log activations and also log if an ES or EM is active es refers to external stop and em refers to emergency stop.
if arm has configurations you can choose to move linearly. refer to column of buttons right side of the screen. if not configured there should be motor control with button 1 and 2 with axis depictions. you choose between the 6 axis in pairs of 3. 
dont press button to hard or it will go past trigger point and not react on trigger. its a safety design to keep continous wear trough brute force away.


to configure align each axis white paint to the other axis side. then when done. go into the settings area. left hand side. then choose service. top of screen you should see menus. press buttons above to choose. then look around until you find the calibration. press calib button and you engage in calibration. then look around until you find correct button avoid fine calibration as that is not something we require. but rough calibration should be engaged. when done you select all by column bottom below screen and press all button then apply and it calibrates the arm rotational counters and linear mode is functional.

DO NOT SHUTDOWN IMPROPER WAY.
shutting down sequence is very important to do properly. in service menu in archive you can see backup and restart and shutdown. this is industrial so shutdown will require power manually shutdown WHEN the HMI says its allowed. DO NOT cut power before that. power cycling seems to do minimal damage but this machine instability is on a higher amount.

machine clock is also unable to count more than 2024. it utilizes floppy disks.

Radar units behaviour is a bit weird. but should suffice. do not modify wiring without looking up manuals. modules have over current protections. but damage to the optocouplers may cause entire machine wide failure. IO unit is a bit more tolerant. but only use output pin 10 and 9 for wiring internally all others shall run trough relays to protect machine from damage. it has an extreme safety protocol but damaging any hardware will cause irreparable damage.

arm supports 6 kg. it theoretically supports more due to law of the lever. BUT 6kg shall not be exceeded by any means. as it can cause arm motor to die or emergency brake locks.

radar units username is “admin” it carries a second administrative entity. this entity is for bypassing register lock. if issues appears with passwords. the reset button is seated inside the plastic case.

machine has also a customer connection port to the arm. each wire supports 500ma at 24v. the customer connection port is connected to the seat in the cabinet using screw seats. 


IO system 1.0
1.1 safety for IO
the system shall have an isolation relay matrix. this modules task is to have A1 and A2 to take signals. the X1 X2 connectors are output system which optically isolated using optocouplers. but as this system will assume user will not know fully what they are doing, an isolation of externals and communication to protect it further from any potential damage. relays short the output X1:10 and X2:10 are constantly on 24v pins. so these will be used to short to the input system. X1:10 will be used to support X3:1 to X3:8 and the same for X2 to X4. all wires will be routed through relays with single contacts and NO contacts.
1.2 configuration software.
for the configuration of the io unit the software has a second button from bottom left hand side. in system parameters, then in communication(needs refining as i do not remember exact directory). when inside you should see the IO unit A328 if configuration was remembered. if not the way to resolve issue is looking down on the lower part of screen. theres buttons below the lcd screen. press button to the left bottom which has “Add” this adds an external device for connection. S4C+ uses an bus connection using 5 wires. theres 12 pins in X5. first 5 pins are bus slots. not relevant to hardware id. the pins which follow are a hardware id configuration. shorting them in a configuration produces a id. if config stays it should be 10. pin 12 is reference pin. shorting this inverts connections from 1 1 0 1 produces 13. shorting to 12 causes an inversion of binary code and produces 0 0 1 0 and decimal is 2.
1.3 Safety modules.
radar controller uses an wire going up to 2 radars and triggers an ES1 and ES2. ES refers to external safety. and it functions trough a serial connection. so if any connection in that daisy chain of contacts the external safety triggers and locks down.
to reset the safety first make sure no emergency stops nor any external safeties.
white blinking module. on the front panel close to the emergency stop button on the controller unit. to be clear, controller unit is the large machine and not the HMI.
holding in the white button it will reset an emergency stop as machine remembers emergency stops after they are released.
behaviour of system is unpredictable due to program version but to run programs in test configuration while in manual control refer to 1.4 for manual and automatic control. when button is glowing constantly rather than blinking it means it will accept test runs of programs. resetting the button to this state is unpredictable and unknown to do properly. previous operators triggered radars and emergency stops then reset them and holding in button to reset emergency stop which resulted in the aforementioned ideal state and required state to test run programs.
But opting for an approach which is more consistent, is swapping mode from manual to production when a program is ready. then accepting on the screen swapping to production. where you no longer can edit programs. The only option is to start it, manual control of arm is also lost during this action until selecting the manual mode again. button will engage in a fully off mode during production start. hold in button to reset stop which makes a constant on state which allows for program runs. This system allows for a 2500mm/s rather than 250mm/s




1.4 auto and manual
on the outside of controller unit close to the eth port and emergency theres a key switch. it requires a key to switch between manual and automatic mode, also referred to as production mode. swapping between the modes causes arm to go more locked down and safe in production due to user not being in direct control of the arm.

1.5 configuration of linear control
to configure linear motion and in general arm the battery pack which is a blue battery pack with 3 massive standard size batteries inside the arm, positioned at the lower back plate of the chassis of the arm. this battery is one time and designed for the robot to be ran 24/7 but due to operation of robot by customer the robots battery depletes much quicker than supposed to.
if battery has charge in it the configuration state can be initialized. To begin, every axis on the arm has a white paint marker on it, the markers need to align when configuring to make it work properly. A rough position is satisfactory for the configuration. To configure it user must go into the service menu. service menu is accessed through pressing the left column and second to bottom button. then when in service menu looking at top row of button above the screen. one should have kalib(section requires more research). choose calibration. then press “all” to configure all axis. then press ok. it will take a few seconds then press column to the right of the screen second to top button should be able to select linear movement. user can test by pressing it. this configures for a mathematical operation to take into consideration limits as axis max degrees individually and tool pitch yaw roll must stay constant. pressing again selects a move around the tools angles. so now tools position gets a constant requirement and angles become the holder.

1.6 programming
main part of robot is programming. software for it is no longer in access. so HMI will be used to program. you move around hand first. then setting an move and * will refer to current position. and most of this system is a basic arithmetic operation set and a basic system for boolean algebra. Mostly basic stuff.

1.7 io unit
powergrid for IO is supplied by pin 9 gnd and pin 10 24v. these pins do not live on the bus power. theese modules live on power provided trough an external power supply. recommendation is for leaching out of radar power grid. the power control is galvanically isolated(refers to no material with an outer electron field which can move freely to allow current trough while still able to communicate) making this construct 3 separate modules. rather than one combined its 3 separate power grids which live trough communication.

1.8 defining io
in system parameters you can find a io settings menu. inside there and topics at the top you can find user defined inputs. there you can define inputs and outputs using the previously defined input output handling unit.
