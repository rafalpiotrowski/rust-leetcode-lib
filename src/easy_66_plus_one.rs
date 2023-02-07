/*
You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer. The digits are ordered from most significant to least significant in left-to-right order. The large integer does not contain any leading 0's.

Increment the large integer by one and return the resulting array of digits.

 

Example 1:

Input: digits = [1,2,3]
Output: [1,2,4]
Explanation: The array represents the integer 123.
Incrementing by one gives 123 + 1 = 124.
Thus, the result should be [1,2,4].
Example 2:

Input: digits = [4,3,2,1]
Output: [4,3,2,2]
Explanation: The array represents the integer 4321.
Incrementing by one gives 4321 + 1 = 4322.
Thus, the result should be [4,3,2,2].
Example 3:

Input: digits = [9]
Output: [1,0]
Explanation: The array represents the integer 9.
Incrementing by one gives 9 + 1 = 10.
Thus, the result should be [1,0].
 

Constraints:

1 <= digits.length <= 100
0 <= digits[i] <= 9
digits does not contain any leading 0's.
*/
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    //edge case is all digits are 9 then we have to have bigger vector
    //we need to travers back from the last position

    let mut is_first_digit = true;
    let mut reminder = 0;
    let mut result = digits.clone();

    result.reverse();

    let mut index = 0;
    while index < result.len() {
        if is_first_digit {
            is_first_digit = false;
            if result[index] == 9 {
                reminder = 1;
                result[0] = 0;
            } else {
                result[0] = result[0] + 1;
                break;
            }
        } else {
            if reminder == 0 {
                break;
            } else if result[index] == 9 {
                //reminder = 1;
                result[index] = 0;
            } else {
                result[index] = result[index] + 1;
                reminder = 0;
                break;
            }
        }
        index += 1;
    }

    if reminder != 0 {
        result.push(1);
    }
    result.reverse();
    result
}