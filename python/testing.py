from rustsum import sum

try:
    result = sum(5, 7)
    print(f"The sum is: {result}")
except TypeError as e:
    print(f"Error: {e}")