Another force acting on a plushie is the stuffing pushing it's walls out. The first attempt at implementing it in this project is called **per-round stuffing**. (Linear time).

### Failed attempt
There was an attempt to just repulse nodes from line X=0, Z=0. However if too many nodes were pulled to one quadrant of the XZ plane, the whole structure would get skewed in an absurd way

### Per-round stuffing
In this approach
- calculate the center of each round
- calculate the desired distance between points and center for each round
- push out points that are too close to the center

Center of a round is calculated as the average of positions of points.

#### Desired distance
- Let $R$ be a round with $N$ stitches, and let $d$ be the desired distance between stitches.
- With realistic stuffing and no other rounds acting on $R$, it's points should be approximating a circle of circumference $Nd$
- Therefore, we can take the radius $r=\frac{Nd}{2\pi}$ of that circle as the desired distance between points and round center, possibly scaled by a constant
- Turns out function continuity is important
  - ~~If the distance is less then desired, the node is pushed by a unit vector along the axis between center and itself~~
  - too_close = desired_radius - actual_distance
  - push = diff.normalize() * (too_close / 4.0).powi(2)

![](images/2024-02-17-23-50-01.png)
*ball generated with attraction + per-round stuffing*

![](images/2024-03-01-19-47-51.png)
*addition of gravity makes the shape more realistic, but it will eventually collapse without vertical stuffing. This may seem useful for simulations, but there is no clear point of stopping, and such is necessary for fitness functions*

![](images/2024-03-01-19-49-37.png)
*gravity + time*

### Need for vertical stuffing
Per-round stuffing works only horizontally. That might be enough if gravity is off and Y of the last point is approximated somewhat accurately in the beginning.

Gravity can be generally turned off for regular plushies, but is necessary for:
- more realistic simulations
- partially non-stuffed creations
- if a plushie has limbs, they could go to weird places

Vertical stuffing will also be required for plushies with "limbs"
![](images/2024-03-01-19-09-47.png)
- red: per-round stuffing force
- blue and green: link forces

if there is nothing counteracting the blue link force, the region where limb is connected to body may malform, and it certainly will collapse if gravity is on. With gravity it may collapse all the way to the ground, as there is no force between disconnected nodes.

Furthermore, the force in per-round stuffing may even work like this, depending on stitches used
![](images/2024-03-01-19-13-27.png)