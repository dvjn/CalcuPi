# **[`CalcuPi 🚀`](https://divykj.github.io/CalcuPi/)**

> Calculate Pi using Monte Carlo method

Please ⭐ if you like it!

## Explanation 🤔

### Steps 🎲

- Use a square of arbitrary side length and inscribe a circle inside it.
- Put random points on the square using a uniform distribution.
- Count the number of random points falling inside the circle.
- Calculate the approximate value of pi using, <br />`4 * points inside the circle / total number of points`.

### Maths 📏

<details>

<summary>
Why <code>4 * points inside the circle / total number of points</code>?
</summary>

<br />

Let's consider,
<br />
Radius of circle, **`r`**
<br />
Side of square, **`2r`**
<br />
Number of points inside the circle, **`C`**
<br />
Total number of points, **`S`**

Thus,
<br />
Area of circle = **`πr²`**
<br />
Area of square = **`4r²`**

For large enough number of random points, we can consider that the ratio of areas of circle to square is equal to the ratio of points inside the circle to the total number of points, ie, **`πr²/4r² = C/S`**.
<br />
Which can be simplified to, **`π = 4*C/S`**

</details>

## Links ✨

- [What is Monte Carlo Method? 🤓](https://en.wikipedia.org/wiki/Monte_Carlo_method "Monte Carlo method - Wikipedia")
- [Inspiration for the Project 💖](https://www.youtube.com/watch?v=5cNnf_7e92Q "Coding Challenge #95: Approximating the Value of Pi - Youtube")
