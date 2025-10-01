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


def functional_example_1() -> None:
    points: list[tuple[float, float]] = [(0.0, 5.0), (8.0, 3.0), (1.0, 7.0)]

    def distance_f(point: tuple[float, float]) -> float:
        return math.sqrt(point[0] ** 2 + (point[1]) ** 2)

    distances = [distance_f(pt) for pt in points]

    shortest_distance = min(distances)
    largest_distance = max(distances)
    average_distance = sum(distances) / len(distances)

    print(f"Min Distance: {shortest_distance:4.2}")
    print(f"Max Distance: {largest_distance:4.2}")
    print(f"Avg Distance: {average_distance:4.2}")


def main() -> None:
    procedural_example_1()

    print()
    procedural_example_2()

    print()
    functional_example_1()


if __name__ == "__main__":
    main()
