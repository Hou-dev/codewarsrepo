# Write a function that takes an integer as input, and returns the number of bits that are equal to one in the binary representation of that number. You can guarantee that input is non-negative.
# Example: The binary representation of 1234 is 10011010010, so the function should return 5 in this case



def count_bits(n):
    count = abs(n)
    binary_rep = bin(count)
    representation = 0
    for i in binary_rep:
        if i == '1':
            representation += 1
    return representation

count_bits(1234)