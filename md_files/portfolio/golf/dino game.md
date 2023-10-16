
# Dino Game

A golfed version of the chrome dino game that can be played in the terminal
This was made to be able to fit into my discord description

Thanks to [BelgianSalamander](https://github.com/BelgianSalamander) for helping

<pre>
<code class="language-python">
import time as q,msvcrt as a;d=t=1;l=' '*20
while' '==l[3]or d:
 d>>=1;t*=.99;print(l[:3]+'🦖^'[d>0]+l[4:],end='\r');l=l[1:]+'🌵 '[hash(t)%9>0];q.sleep(t)
 if a.kbhit()*(d<1):d=9;a.getch()

</code>
</pre>

looks a bit like this:
<pre>
<code class="language-">
C:\User\dino.py>   🦖    🌵🌵      🌵   
</code>
</pre>

