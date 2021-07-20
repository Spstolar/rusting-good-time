Our spiral is a function from $[0, 2\pi n] \to \R^2$ which sends $t \mapsto (r(t) cos(t), r(t) sin(t))$ where $r(t)$ is the radius that we would like to find a nice function for.

Want 
1. $r(t + 2\pi) - r(t) > 0$
2. $r(t) \to 0$ as $t \to 2\pi n$
3. $r(t + 2\pi) - r(t) > r(t) - r(t - 2\pi)$

max_angle = n * 2 * PI;
SPIRAL_RADIUS = 350.0;

So, 
r(0) = SPIRAL_RADIUS
r(max_angle) = 0

r(t) = SPIRAL_RADIUS (max_angle - t) / max_angle
