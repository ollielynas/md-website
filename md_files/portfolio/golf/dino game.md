<link rel="stylesheet" type="text/css" media="screen" href="prism.css" loading="lazy">

# Dino Game

A golfed version of the chrome dino game that can be played in the terminal
```python
import time as q,msvcrt as a;d=t=1;l=' '*20
while' '==l[3]or d:
 d>>=1;t*=.99;print(l[:3]+'🦖^'[d>0]+l[4:],end='\r');l=l[1:]+'🌵 '[hash(t)%9>0];q.sleep(t)
 if a.kbhit()*(d<1):d=9;a.getch()
 ```
looks a bit like this:
```
C:\User\dino.py>   🦖    🌵🌵      🌵   
```