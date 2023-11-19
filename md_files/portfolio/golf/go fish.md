


## Go Fish

<!-- META A golfed version of the game Go Fish made with python META -->

Includes an ai opponent

<pre>
<code class="language-python">
T='cls';S=set
R=input;I=print;H=len
import random as J,os
E=[];N='♣♠♦♡';K=['ai','you']
for O in N:E+=[A.replace('t','10')+O for A in'A2345689tJQK']
Q=J.shuffle;Q(E);B=0;D=[[],[]];A=[E[:6],E[6:13]];E=E[13:]
os.system(T)
while H(D[0]+D[1])<12:
 A[1].sort();B=[1,0][B];L=[1,0][B];F=[C[:-1]for C in A[B]];P=[B[:-1]for B in A[L]];M=['your cards:']
 for C in range(0,H(A[1])):
  if A[1][C][:-1]==A[1][C-1][:-1]and M[-1][0]==A[1][C][0]:M[-1]+='|'+A[1][C]
  else:M.append(A[1][C])
 if B:G=R(f"ai cards:{' [?]'*H(A[0])}\n{D[0]}\n{D[1]}\n{M}\ngive me a :").upper();os.system(T)
 else:
  if F==[]:G=J.choice(A[1])[-1]
  try:G=max(S(F),key=F.count)
  except ValueError:G=J.choice(A[1])[-1]
  if J.random()>0.8:G=J.choice(F)
 I(f"{K[B]} requested {G}")
 if G in P:A[B].append(A[L].pop(P.index(G)));I(f"{K[L]} gave away {A[B][-1]}")
 else:
  I(f"{K[L]} said go fish")
  if H(E)>0:
   A[B].append(E.pop())
   if B:I(f"you picked up {A[1][-1]}")
  else:I('there are no cards left to pick up')
 F=[C[:-1]for C in A[B]]
 for C in S(F):
  if F.count(C)==4:
   for O in N:A[B].remove(C+O)
   D[B].append(C+N);I(f"{K[B]} placed a set of {C}s")
if H(D[0])==H(D[1]):R('tie');quit()
I(K[int(H(D[0])<H(D[1]))]+' won')

</code>
</pre>


<!-- LAST EDITED 1700392251 LAST EDITED-->