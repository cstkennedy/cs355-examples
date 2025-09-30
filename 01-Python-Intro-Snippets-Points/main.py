import math


def procedural_example_1() -> None:
    point1: tuple[float, float] = (0.0, 5.0)
    point2: tuple[float, float] = (8.0, 3.0)
    point3: tuple[float, float] = (1.0, 7.0)

    points = [point1, point2, point3]

    for point in points:
        distance = math.sqrt(point[0] ** 2 + point[1] ** 2)
        print(
            f"From (0, 0) to {point!r} is {distance:4.2f} (distance units)",
        )


def procedural_example_2() -> None:
    points: list[tuple[float, float]] = [(0.0, 5.0), (8.0, 3.0), (1.0, 7.0)]

    for point in points:
        distance = math.sqrt(point[0] ** 2 + point[1] ** 2)
        print(
            f"From (0, 0) to {point!r} is {distance:4.2f} (distance units)",
        )


def functional_example_1():
    points: list[tuple[float, float]] = [(0.0, 5.0), (8.0, 3.0), (1.0, 7.0)]

    distance_f = lambda point: math.sqrt(point[0] ** 2 + (point[1]) ** 2)

    distances = [distance_f(pt) for pt in points]

    """
    shortest_distance: float = *distances.iter().min_by_key(|c| OrderedFloat(**c)).unwrap()
    largest_distance: float = *distances.iter().max_by_key(|c| OrderedFloat(**c)).unwrap()
    average_distance: float = distances.iter().sum::<float>() / (distances.len() as float)
    """


def main() -> None:
    procedural_example_1()

    print()
    procedural_example_2()

    print()
    functional_example_1()


if __name__ == "__main__":
    main()
