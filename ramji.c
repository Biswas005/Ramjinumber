#include <stdio.h>
#include <stdlib.h>

//function to take input from user
int take_input()
{
    int number;
    printf("Please enter a number: ");
    scanf("%d", &number);
    return number;
}

//function to slice a number into individual numbers
int* slice_number(int number, int* size) 
{
    int* number_slice = malloc(10 * sizeof(int));
    int number_copy = number;
    int i = 0;
    while (number_copy > 0) 
    {
        number_slice[i] = number_copy % 10;
        number_copy = number_copy / 10;
        i++;
    }
    *size = i;
    return number_slice;
}

//function to find factorial
int factorial(int number) 
{
    int factorial = 1;
    for (int i = 1; i <= number; i++) 
    {
        factorial *= i;
    }
    return factorial;
}

//function to calculate sum of factorials
int sum_of_factorials(int* number_slice, int size) 
{
    int sum = 0;
    for (int i = 0; i < size; i++) 
    {
        sum += factorial(number_slice[i]);
    }
    return sum;
}

//main function
int main() 
{
    printf("This program is used to take in a number and check if it is a ramji number or not\n");
    int number = take_input();
    int size;
    int* number_slice = slice_number(number, &size);
    int sum = sum_of_factorials(number_slice, size);
    if (sum == number) 
    {
        printf("The number is a ramji number\n");
    } 
    else 
    {
        printf("The number is not a ramji number\n");
    }
    free(number_slice);
    return 0;
}