from collections import defaultdict
import numpy as np
from numpy._typing import NDArray

X = 0
DATA = set(range(1,10))

def fill_with_one_possibility(array: NDArray):
    new_array = array.copy()
    for x in range(9):
        for y in range(9):
            pos = (x,y)
            if new_array[x,y] is None:
                posibilities = get_posibilities(pos, new_array)
                print('posibilities for ', pos,'are', posibilities)
                if len(posibilities) == 1:
                    new_array[x,y] = posibilities.pop()
    return array


def get_posibilities(el_pos: tuple, array):
    x, y = el_pos

    # print('vertical', array[x, :])
    # print('horizontal', array[:, y])
    vertical = list(array[x, :])
    horizontal = list(array[:, y])
    square_x = x // 3 * 3
    square_y = y // 3 * 3
    square = list(array[square_x:square_x+3, square_y:square_y+3].flatten())
    # print('square pos', square_x, square_y)
    return DATA - set(vertical + horizontal + square)


def solve(array: NDArray):
    jumps = 0
    def _solve(array: NDArray):
        nonlocal jumps
        jumps += 1

        posibilities_by_cords = [
            ((x,y), get_posibilities((x,y), array))
            for (x,y) in np.argwhere(array == X)
        ]

        if not posibilities_by_cords:
            return (True, array)

        posibilities_by_cords = sorted(posibilities_by_cords, key=lambda x: len(x[1]))
        row, col = posibilities_by_cords[0][0]

        print(f'row={row},col={col},to find={len(posibilities_by_cords)}, posibilities={posibilities_by_cords[0][1]}')
        # print(cords)
        for i in posibilities_by_cords[0][1]:
            if valid(array, i, (row, col)):
                print('valid', i, 'in', i in posibilities_by_cords[0][1])
                # do przyspieszenia
                array[row][col] = i

                if _solve(array)[0]:
                    # print('found')
                    return (True, array)

                array[row][col] = 0
        else:
            print(f'end of {row},{col}')

        # print(array)

        return (False, array)
    ex = _solve(array)
    print(jumps)
    return ex


def valid(array, num, cord):
    x, y = cord
    # Check row
    if num in array[x, :]:
        return False

    # Check column
    if num in array[:, y]:
        return False

    # Check box
    square_x = x // 3 * 3
    square_y = y // 3 * 3
    if num in array[square_x:square_x+3, square_y:square_y+3].flatten():
        return False

    return True

def find_empty(array: NDArray):
    cords = np.argwhere(array == X)
    return cords[0] if cords.any() else None


def main():
    x = X
    table = [
        [x,5,x,x,x,x,x,x,9],
        [x,x,6,x,x,8,5,x,x],
        [x,x,3,4,5,x,2,x,x],
        [x,6,x,3,9,7,x,x,4],
        [x,9,x,2,6,1,x,x,5],
        [x,2,x,x,x,x,x,x,3],
        [x,x,1,9,4,x,7,x,x],
        [x,3,x,x,x,x,x,x,1],
        [x,x,2,x,x,5,3,x,x],
    ]
    tab2 = [
        [x,2,x,x,6,x,x,x,x],
        [x,x,x,x,2,x,7,x,1],
        [x,3,x,8,x,x,x,x,4],
        [1,x,x,x,x,7,x,x,9],
        [x,4,x,x,8,x,x,7,x],
        [6,x,x,5,x,x,x,x,2],
        [9,x,x,x,x,6,x,4,x],
        [5,x,7,x,3,x,x,x,x],
        [x,x,x,x,7,x,x,9,x],
    ]
    resolved, arr = solve(np.array(tab2))
    
    print(resolved)
    print(arr)
    # print(get_posibilities((0,3), np.array(table)))



if __name__ == "__main__":
    main()
