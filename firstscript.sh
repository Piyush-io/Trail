#!/bin/bash

# Prompt the user to enter values for a and b
read -p "Enter a : " a
read -p "Enter b : " b

# Calculate the sum of a and b and store it in var
var=$((a + b))

# Print the result
echo "Sum of a : $a and b : $b is $var"

# Determine and print the bigger number
if [ $b -ge $a ]; then
    echo "$b is the bigger number."
elif [ $a -ge $b ]; then
    echo "$a is the bigger number."
fi

# Check if both numbers are less than 100
if [ $a -lt 100 ] && [ $b -lt 100 ]; then
    echo "Try bigger numbers"
fi

# Loop from 1 to 6 and print each number
for i in {1..6}; do
    echo "$i"
done

# Print Try bigger numbers
for x in Try bigger numbers
do
    echo "$x"
done
var=`df -h | grep tmpfs`
echo $var
