## Boids
<!-- META a boid simulation made entirely within desmos META -->
The boids follow each other and this causes them to demonstrate a sort of flocking behavior. In this example the boids are also attracted towards the center of the graph. This prevents them from flying out of view. 

The following variables can be changed to influence behavior:

 - Attraction, *a multiplier for the attractive forces between boids*

$$p_{ull}\left(a\right)\ =\ n_{orm}\left(b_{rb}\left[d_{is}\left(a,b_{rb}\right)<10\right]-a,1\right)$$

 - Inertia, *a multiplier for the inertia of the boids*

 - CenterAttraction, *how much the boids are attracted to the center of the graph*

 - Speed *how fast the boids move*

 - Repulsion *how much the boids are repelled away from each other*

$$r_{epell}\left(a\right)\ =\frac{n_{orm}\left(a-b_{rb}\left[d_{is}\left(a,b_{rb}\right)<5\right],1\right)}{1.2}$$

 - Randomness *how much random variation is in the boids movement*

 - Follow *how much the boids follow each other*

<iframe src="https://www.desmos.com/calculator/xbiv9gh80l?embed" width="500" height="500" style="border: 1px solid #ccc" frameborder=0></iframe>
<!-- LAST EDITED 1699411022 LAST EDITED-->