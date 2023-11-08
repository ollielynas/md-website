## probability distributions

<!-- META use demsos to fit a list of continuous data to probability distribution. Then use this distribution to calculate probities  META -->

In order to input your own data you will need to open in Desmos

### how to use

takes data input in the for of a list of values

can calculate normal, triangular, and rectangular prediction models based on the data provided.

Equations
$$f_{unc}\ =\ \left[f_{n}\left(x\right),f_{r}\left(x\right),f_{t}\left(x\right)\right]$$

Normal Distribution
$$f_{n}\left(x\right)=\frac{1}{s_{d}\cdot\left(2\pi\right)^{0.5}}e^{-\frac{1}{2}\left(\frac{x-m}{s_{d}}\right)^{2}}$$

Rectangular Distribution
$$f_{r}\left(x\right)=\{\min\left(d\right)<x<\max\left(d\right):\frac{1}{\left(\max\left(d\right)-\min\left(d\right)\right)},\}$$

Triangular Distribution
$$f_{t}\left(x\right)=\{\min\left(d\right)<x<t_{c}:\frac{2\left(x-\min\left(d\right)\right)}{\left(\max\left(d\right)-\min\left(d\right)\right)\left(t_{c}-\min\left(d\right)\right)},t_{c}<x<\max\left(d\right):\frac{2\left(\max\left(d\right)-x\right)}{\left(\max\left(d\right)-\min\left(d\right)\right)\left(\max\left(d\right)-t_{c}\right)},\}$$

The best distribution is calculated using this function
$$m_{atch}=\left[\sum_{w=\frac{r_{g}}{100}}^{100}\left(f_{p}\left(w,w+\frac{r_{g}}{100},i_{12}\right)-\frac{\operatorname{count}\left(d\left[w<d<w+\frac{r_{g}}{10}\right]\right)}{\operatorname{count}\left(d\right)}\right)^{2}\ \operatorname{for}\ i_{12}=\left[1...f_{unc}.\operatorname{length}\right]\right]$$

click *find best distro* to find the best model to fit your data

<iframe src="https://www.desmos.com/calculator/acydcvlec3?embed" width="500" height="500" style="border: 1px solid #ccc" frameborder=0></iframe>
<!-- LAST EDITED Wed Nov  8 14:36:37 2023 LAST EDITED-->