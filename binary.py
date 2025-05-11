def binary_search(list, target):
    list.sort()  # Ensure the list is sorted
    left = 0
    right = len(list) - 1

    while left <= right:
        midpoint = (left + right) // 2

        if target > list[midpoint]:
            left = midpoint + 1
        elif target < list[midpoint]:
            right = midpoint - 1
        else:
            return midpoint  # Target found

    return -1  # Target not found

if __name__ == "__main__":
    list1 = [5, 47, 58, 39, 74, 93, 64, 2]
    target1 = 5
    
    print(binary_search(list1, target1))