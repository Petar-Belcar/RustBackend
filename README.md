# Project

## Purpose

- the project was created as part of my bachelors thesis on the simplex method
- this is a web server written in Rust designed to receive post request containing a JSON file

## Current functionality

As things stand the program is able to solve linear programs that look like

$$
\begin{bmatrix}
    1 && \dots && 0 && \dots && 0 && a_{11} && \dots && a_{1j} && \dots && a_{1n} \\
    \vdots &&  && \vdots && && \vdots && \vdots &&  && \vdots &&  && \vdots \\
    0 && \dots && 1 && \dots && 0 && a_{i1} && \dots && a_{ij} && \dots && a_{in} \\
    \vdots &&  && \vdots && && \vdots && \vdots &&  && \vdots &&  && \vdots \\
    0 && \dots && 0 && \dots && 1 && a_{m1} && \dots && a_{mj} && \dots && a_{mn} \\
\end{bmatrix} =
\begin{bmatrix}
b_1 \\
\vdots \\
b_i \\
\vdots \\
b_m
\end{bmatrix}
$$

$$
a_{ij}, b_i \in \mathbb{R} , \quad \forall i,j \\
b_i \geq 0, \quad \forall i\\
i \geq j
$$

The program has to expressly start with an identity so in reality the problems it can solve would look like

$$
\begin{bmatrix}
    a_{11} && \dots && a_{1j} && \dots && a_{1n} \\
    \vdots &&  && \vdots &&  && \vdots \\
    a_{i1} && \dots && a_{ij} && \dots && a_{in} \\
    \vdots &&  && \vdots &&  && \vdots \\
    a_{m1} && \dots && a_{mj} && \dots && a_{mn} \\
\end{bmatrix} \leq
\begin{bmatrix}
b_1 \\
\vdots \\
b_i \\
\vdots \\
b_m
\end{bmatrix}
$$

which limits its usefulness.

You would have to send the server a POST request containing a JSON that would look like

```json
{
    "tableau": [
        {
            "a_ij": [
                1.0,
                0.0,
                1.0,
                1.0
            ],
            "b_i": 1.0
        },
        {
            "a_ij": [
                0.0,
                1.0,
                2.0,
                1.0
            ],
            "b_i": 1.0
        }
    ],
    "costs": [
        0.0,
        0.0,
        1.0,
        2.0
    ],
    "relative_costs": {
        "a_ij": [
            0.0,
            0.0,
            0.0,
            0.0
        ],
        "b_i": 0.0
    },
    "solution": [
        0.0,
        0.0,
        0.0,
        0.0
    ]
}
```

where the `tableau` field represents the linear equations, the `costs` field represents the costs of each corresponding variable. The fields `relative_costs` and `solution` are also needed but the data contained in them is not need, so they could be filed with random numbers. This is because I didn't have the foresight to make them an `Option<>`.

## Future functionality

I'm currently working on solving a set of linear equations in such a way that all of the members of a solution would be positive numbers. I'm sure someone already came up with an algorithm, but I haven't spent much time looking for one. The status on that is that a rudimentary algorithm for solving a set of linear equations is implemented on this branch but it simply returns a solution if one exists regardless if it contains negative numbers.
