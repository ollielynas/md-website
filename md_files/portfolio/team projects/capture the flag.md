## Capture The Flag

Made by:

[BelgianSalamander](https://github.com/BelgianSalamander)

[Ollie Lynas](https://github.com/ollielynas)

[Sseptimus](https://github.com/Sseptimus)

<br>

[web interface](https://sseptimus.github.io/CTF/)

*The interface seems to be broken at the moment*

<br>

This project was inspired by [Jet Lag:The Game](https://www.youtube.com/@jetlagthegame). I was in charge of the web interface. The web interface tracks the position of all of the players and relays this position back to the server. The positions of all of the players and the flags are rendered on a map of the globe. The web interface is also in charge of calculating which pieces of land belong to which teams. All of the data that is sent to te server is sent through https and the server will not send a device data unless they provide a randomly generated password. for security reasons the server host cannot pick the password because I know how bad my friends are at picking passwords. 

<br>

Those are the features that are currently implemented. The plan is to add the ability to tag a person by takeing a picture of them. The web interface allows you to send pictures to any of the people playing. Once they conform that the picture is of them, they will be marked as dead. They will then need to return to their sides in order to respawn. This works on the honor system, however the pictures are kept on the server until the game is over so they can be used to settle disagreements after the fact. This also serves to discourage cheating. 


<!-- LAST EDITED 1699417610 LAST EDITED-->